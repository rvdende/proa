// Copyright Proa Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Static export: render an application's pages and the assets they reference
//! into a directory of plain files, for hosts that only serve files (GitHub
//! Pages, an object store, a CDN bucket).
//!
//! `proa export` writes a [`StaticExportSpec`] and starts the release binary
//! with [`STATIC_EXPORT_ENV`] naming it. [`serve`](crate::server::serve) sees
//! the variable and, instead of accepting connections, sends each route
//! through the application's own router in process, writes the responses, and
//! returns. No port is bound for requests and nothing crawls over the network.
//!
//! Pages come from the spec's routes. With `follow_links`, every same-origin
//! URL a written HTML page, stylesheet or script references is exported too:
//! anchors, `src`/`href`/`srcset`, import maps, module imports and CSS
//! `url()`s. That picks up hashed island assets, fonts and pages behind
//! dynamic routes that some exported page links to.

use std::{
    collections::{HashSet, VecDeque},
    io,
    path::{Component, Path, PathBuf},
};

use axum::{
    body::{to_bytes, Body},
    http::{header, Request, StatusCode},
    Router,
};
use proa_core::BasePath;
use serde::{Deserialize, Serialize};
use tower::ServiceExt as _;
use url::Url;

use crate::asset_urls::{css_urls, js_urls};

/// Environment variable naming the JSON [`StaticExportSpec`] file. When it is
/// set, [`serve`](crate::server::serve) exports instead of serving.
pub const STATIC_EXPORT_ENV: &str = "PROA_STATIC_EXPORT";

/// The page a host shows for a missing file; requested at a path no
/// application routes.
const NOT_FOUND_PROBE: &str = "/__proa_static_export_not_found__";

/// Origin the exported requests are resolved against; never contacted.
const EXPORT_ORIGIN: &str = "http://proa-export.invalid";

/// What to export and where.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticExportSpec {
    /// Directory the files are written into.
    pub out_dir: PathBuf,
    /// Where the [`StaticExportReport`] is written as JSON.
    pub report: PathBuf,
    /// The URL prefix the site is served under (`PROA_BASE_PATH`); the same
    /// one the application was started with.
    #[serde(default)]
    pub base_path: String,
    /// Application paths that must render; one that fails fails the export.
    pub routes: Vec<String>,
    /// Paths exported when they exist, such as `/robots.txt`.
    #[serde(default)]
    pub optional_routes: Vec<String>,
    /// Also export every same-origin URL an exported page or asset references.
    #[serde(default = "default_true")]
    pub follow_links: bool,
    /// Write the application's not-found page to `404.html`.
    #[serde(default = "default_true")]
    pub not_found_page: bool,
}

fn default_true() -> bool {
    true
}

/// What an export wrote.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StaticExportReport {
    /// Every written file, relative to the output directory.
    pub files: Vec<ExportedFile>,
    /// Referenced URLs that could not be exported; the export still succeeds.
    pub warnings: Vec<String>,
}

/// One written file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedFile {
    /// The application path it was rendered from.
    pub path: String,
    /// The file, relative to the output directory.
    pub file: PathBuf,
    /// The response's content type.
    pub content_type: String,
    /// A redirect page standing in for the application's redirect.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub redirect: bool,
}

/// Read the spec named by [`STATIC_EXPORT_ENV`], if the variable is set.
pub fn spec_from_env() -> Option<io::Result<StaticExportSpec>> {
    let path = std::env::var_os(STATIC_EXPORT_ENV)?;
    Some(std::fs::read(&path).and_then(|bytes| {
        serde_json::from_slice(&bytes).map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "{STATIC_EXPORT_ENV} {}: {error}",
                    Path::new(&path).display()
                ),
            )
        })
    }))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Need {
    Required,
    Optional,
    Referenced,
}

/// Render `spec`'s routes through `app` into `spec.out_dir`, and write the
/// report to `spec.report`.
pub async fn export(app: Router, spec: &StaticExportSpec) -> io::Result<StaticExportReport> {
    let base = BasePath::new(&spec.base_path)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error.to_string()))?;
    let origin = Url::parse(EXPORT_ORIGIN).expect("valid export origin");
    let mut report = StaticExportReport::default();
    let mut errors = Vec::new();
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    for (paths, need) in [
        (&spec.routes, Need::Required),
        (&spec.optional_routes, Need::Optional),
    ] {
        for path in paths {
            let path = if path.starts_with('/') {
                path.clone()
            } else {
                format!("/{path}")
            };
            if seen.insert(path.clone()) {
                queue.push_back((path, need, None::<String>));
            }
        }
    }
    std::fs::create_dir_all(&spec.out_dir)?;

    while let Some((path, need, referrer)) = queue.pop_front() {
        let failure = |message: String| match &referrer {
            Some(referrer) => format!("{path} (referenced by {referrer}): {message}"),
            None => format!("{path}: {message}"),
        };
        let response = get(&app, &path).await?;
        let status = response.status();
        if status.is_redirection() {
            let Some(location) = response
                .headers()
                .get(header::LOCATION)
                .and_then(|value| value.to_str().ok())
            else {
                errors.push(failure(format!("{status} without a Location header")));
                continue;
            };
            let target = page_url(&origin, &base, &path).join(location);
            let file = output_file(&path, true);
            let Some(file) = file else {
                errors.push(failure("cannot be written as a file".into()));
                continue;
            };
            let href = target
                .as_ref()
                .map(|url| {
                    if url.origin() == origin.origin() {
                        url[url::Position::BeforePath..].to_string()
                    } else {
                        url.to_string()
                    }
                })
                .unwrap_or_else(|_| location.to_string());
            write(&spec.out_dir, &file, redirect_page(&href).as_bytes())?;
            report.files.push(ExportedFile {
                path: path.clone(),
                file,
                content_type: "text/html".into(),
                redirect: true,
            });
            if let Some(next) = target.ok().and_then(|url| app_path(&origin, &base, &url)) {
                if seen.insert(next.clone()) {
                    queue.push_back((next, need, Some(path.clone())));
                }
            }
            continue;
        }
        if status != StatusCode::OK {
            match need {
                Need::Required => errors.push(failure(format!("responded {status}"))),
                Need::Referenced => report.warnings.push(failure(format!("responded {status}"))),
                Need::Optional => {}
            }
            continue;
        }
        let content_type = content_type(&response);
        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .map_err(|error| io::Error::other(failure(error.to_string())))?;
        let Some(file) = output_file(&path, content_type == "text/html") else {
            errors.push(failure("cannot be written as a file".into()));
            continue;
        };
        write(&spec.out_dir, &file, &body)?;
        if spec.follow_links {
            let page = page_url(&origin, &base, &path);
            for reference in references(&content_type, &body) {
                let Ok(url) = page.join(&reference) else {
                    continue;
                };
                if let Some(next) = app_path(&origin, &base, &url) {
                    if seen.insert(next.clone()) {
                        queue.push_back((next, Need::Referenced, Some(path.clone())));
                    }
                }
            }
        }
        report.files.push(ExportedFile {
            path,
            file,
            content_type,
            redirect: false,
        });
    }

    if spec.not_found_page {
        let response = get(&app, NOT_FOUND_PROBE).await?;
        if response.status() == StatusCode::NOT_FOUND && content_type(&response) == "text/html" {
            let body = to_bytes(response.into_body(), usize::MAX)
                .await
                .map_err(|error| io::Error::other(format!("not-found page: {error}")))?;
            let file = PathBuf::from("404.html");
            write(&spec.out_dir, &file, &body)?;
            report.files.push(ExportedFile {
                path: NOT_FOUND_PROBE.into(),
                file,
                content_type: "text/html".into(),
                redirect: false,
            });
        }
    }

    let json = serde_json::to_vec_pretty(&report).map_err(io::Error::other)?;
    std::fs::write(&spec.report, json)?;
    if errors.is_empty() {
        Ok(report)
    } else {
        Err(io::Error::other(format!(
            "static export failed:\n  {}",
            errors.join("\n  ")
        )))
    }
}

async fn get(app: &Router, path: &str) -> io::Result<axum::response::Response> {
    let request = Request::get(path)
        .header(header::HOST, "localhost")
        .header(header::ACCEPT, "text/html,*/*;q=0.8")
        .body(Body::empty())
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, format!("{path}: {error}")))?;
    match app.clone().oneshot(request).await {
        Ok(response) => Ok(response),
        Err(never) => match never {},
    }
}

fn content_type(response: &axum::response::Response) -> String {
    response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(';').next())
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase()
}

/// The browser-visible URL of an application path.
fn page_url(origin: &Url, base: &BasePath, path: &str) -> Url {
    origin
        .join(&format!("{}{path}", base.as_str()))
        .unwrap_or_else(|_| origin.clone())
}

/// The application path a browser-visible URL names, if it is this site's.
fn app_path(origin: &Url, base: &BasePath, url: &Url) -> Option<String> {
    if url.origin() != origin.origin() {
        return None;
    }
    let path = url.path();
    let prefix = base.as_str();
    let rest = path.strip_prefix(prefix)?;
    if rest.is_empty() {
        Some("/".into())
    } else if rest.starts_with('/') {
        Some(rest.into())
    } else {
        None
    }
}

/// The file an application path is written to: pages become `index.html` in
/// a directory of their name so `/docs` and `/docs/` both resolve on a static
/// host; everything else keeps its path. `None` for a path that would leave
/// the output directory.
fn output_file(path: &str, html: bool) -> Option<PathBuf> {
    let decoded = percent_decode(path)?;
    let mut file = PathBuf::new();
    for segment in decoded.split('/').filter(|segment| !segment.is_empty()) {
        let segment = Path::new(segment);
        if segment.components().count() != 1
            || !matches!(segment.components().next(), Some(Component::Normal(_)))
        {
            return None;
        }
        file.push(segment);
    }
    let last = decoded.rsplit('/').next().unwrap_or("");
    if decoded.ends_with('/') || (html && !last.contains('.')) {
        file.push("index.html");
    }
    Some(file)
}

fn percent_decode(path: &str) -> Option<String> {
    let bytes = path.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let hex = std::str::from_utf8(bytes.get(index + 1..index + 3)?).ok()?;
            let byte = u8::from_str_radix(hex, 16).ok()?;
            if byte == b'/' || byte == b'\\' || byte == 0 {
                return None;
            }
            out.push(byte);
            index += 3;
        } else {
            out.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(out).ok()
}

fn write(out_dir: &Path, file: &Path, bytes: &[u8]) -> io::Result<()> {
    let path = out_dir.join(file);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, bytes)
        .map_err(|error| io::Error::new(error.kind(), format!("{}: {error}", path.display())))
}

fn redirect_page(href: &str) -> String {
    let href = escape_attribute(href);
    format!(
        "<!doctype html>\n<meta charset=\"utf-8\">\n<title>Redirecting</title>\n\
         <meta http-equiv=\"refresh\" content=\"0; url={href}\">\n\
         <link rel=\"canonical\" href=\"{href}\">\n<a href=\"{href}\">Continue</a>\n"
    )
}

fn escape_attribute(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// The URLs a response body references, as written.
fn references(content_type: &str, body: &[u8]) -> Vec<String> {
    let Ok(source) = std::str::from_utf8(body) else {
        return Vec::new();
    };
    match content_type {
        "text/html" => html_urls(source),
        "text/css" => css_urls(source).unwrap_or_default(),
        "text/javascript" | "application/javascript" => script_urls(source),
        _ => Vec::new(),
    }
}

/// A script's URLs, without bare module specifiers an import map resolves.
fn script_urls(source: &str) -> Vec<String> {
    js_urls(source)
        .unwrap_or_default()
        .into_iter()
        .filter(|url| {
            url.starts_with('/')
                || url.starts_with("./")
                || url.starts_with("../")
                || url.starts_with("http:")
                || url.starts_with("https:")
        })
        .collect()
}

/// Attributes whose value is a URL a page loads or links to.
const URL_ATTRIBUTES: &[&str] = &["href", "src", "poster", "data", "background"];

/// The URLs an HTML document references through URL attributes, `srcset`,
/// inline styles, import maps, and inline module scripts and stylesheets.
fn html_urls(source: &str) -> Vec<String> {
    let mut urls = Vec::new();
    let mut rest = source;
    while let Some(start) = rest.find('<') {
        rest = &rest[start + 1..];
        if let Some(comment) = rest.strip_prefix("!--") {
            rest = comment.find("-->").map_or("", |end| &comment[end + 3..]);
            continue;
        }
        let name_end = rest
            .find(|c: char| c.is_ascii_whitespace() || c == '>' || c == '/')
            .unwrap_or(rest.len());
        let name = rest[..name_end].to_ascii_lowercase();
        if name.is_empty() || !name.starts_with(|c: char| c.is_ascii_alphabetic()) {
            continue;
        }
        let (attributes, after) = tag_attributes(&rest[name_end..]);
        rest = after;
        for (attribute, value) in &attributes {
            if URL_ATTRIBUTES.contains(&attribute.as_str()) {
                urls.push(value.clone());
            } else if attribute == "srcset" || attribute == "imagesrcset" {
                urls.extend(
                    value
                        .split(',')
                        .filter_map(|candidate| candidate.split_whitespace().next())
                        .map(str::to_owned),
                );
            } else if attribute == "style" {
                urls.extend(css_urls(&format!("a{{{value}}}")).unwrap_or_default());
            }
        }
        if name == "script" || name == "style" {
            let close = format!("</{name}");
            let end = find_ignore_case(rest, &close).unwrap_or(rest.len());
            let body = &rest[..end];
            rest = &rest[end..];
            let kind = attributes
                .iter()
                .find(|(attribute, _)| attribute == "type")
                .map(|(_, value)| value.to_ascii_lowercase());
            match (name.as_str(), kind.as_deref()) {
                ("style", _) => urls.extend(css_urls(body).unwrap_or_default()),
                ("script", Some("importmap")) => import_map_urls(body, &mut urls),
                ("script", None | Some("module" | "text/javascript")) => {
                    urls.extend(script_urls(body))
                }
                _ => {}
            }
        }
    }
    urls
}

fn find_ignore_case(haystack: &str, needle: &str) -> Option<usize> {
    haystack
        .as_bytes()
        .windows(needle.len())
        .position(|window| window.eq_ignore_ascii_case(needle.as_bytes()))
}

/// Parse the attributes of a start tag, returning them lowercased by name with
/// entity-decoded values, and the text after the tag.
fn tag_attributes(mut rest: &str) -> (Vec<(String, String)>, &str) {
    let mut attributes = Vec::new();
    loop {
        rest = rest.trim_start_matches(|c: char| c.is_ascii_whitespace() || c == '/');
        if rest.is_empty() {
            return (attributes, rest);
        }
        if let Some(after) = rest.strip_prefix('>') {
            return (attributes, after);
        }
        let name_end = rest
            .find(|c: char| c.is_ascii_whitespace() || matches!(c, '=' | '>' | '/'))
            .unwrap_or(rest.len())
            .max(1);
        let name = rest[..name_end].to_ascii_lowercase();
        rest = rest[name_end..].trim_start();
        let Some(after_equals) = rest.strip_prefix('=') else {
            attributes.push((name, String::new()));
            continue;
        };
        rest = after_equals.trim_start();
        let value = match rest.chars().next() {
            Some(quote @ ('"' | '\'')) => {
                let body = &rest[1..];
                let end = body.find(quote).unwrap_or(body.len());
                rest = body.get(end + 1..).unwrap_or("");
                &body[..end]
            }
            _ => {
                let end = rest
                    .find(|c: char| c.is_ascii_whitespace() || c == '>')
                    .unwrap_or(rest.len());
                let value = &rest[..end];
                rest = &rest[end..];
                value
            }
        };
        attributes.push((name, decode_entities(value)));
    }
}

fn decode_entities(value: &str) -> String {
    if !value.contains('&') {
        return value.to_owned();
    }
    value
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&#x27;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

fn import_map_urls(source: &str, urls: &mut Vec<String>) {
    let Ok(map) = serde_json::from_str::<serde_json::Value>(source) else {
        return;
    };
    let imports = map.get("imports").into_iter();
    let scopes = map
        .get("scopes")
        .and_then(serde_json::Value::as_object)
        .into_iter()
        .flat_map(|scopes| scopes.values());
    for specifiers in imports.chain(scopes) {
        if let Some(specifiers) = specifiers.as_object() {
            urls.extend(
                specifiers
                    .values()
                    .filter_map(serde_json::Value::as_str)
                    .map(str::to_owned),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        response::{Html, Redirect},
        routing::get as route_get,
    };

    #[test]
    fn pages_become_directory_indexes_and_assets_keep_their_paths() {
        let file = |path, html| output_file(path, html).map(|file| file.display().to_string());
        assert_eq!(file("/", true).as_deref(), Some("index.html"));
        assert_eq!(file("/docs", true).as_deref(), Some("docs/index.html"));
        assert_eq!(file("/docs/", true).as_deref(), Some("docs/index.html"));
        assert_eq!(file("/page.html", true).as_deref(), Some("page.html"));
        assert_eq!(file("/index.md", false).as_deref(), Some("index.md"));
        assert_eq!(
            file("/static/a%20b.svg", false).as_deref(),
            Some("static/a b.svg")
        );
        assert_eq!(file("/../etc/passwd", false), None);
        assert_eq!(file("/a/%2e%2e/b", false), None);
        assert_eq!(file("/a%2fb", false), None);
    }

    #[test]
    fn html_references_cover_attributes_import_maps_and_inline_code() {
        let source = r#"<!doctype html><html><head>
<!-- <a href="/commented"> -->
<link rel=stylesheet href=/static/app.css>
<link rel="icon" href="/favicon.svg?v=1">
<script type="importmap">{"imports":{"rsjs":"/rsjs-runtime.js"},"scopes":{"/x/":{"y":"/y.js"}}}</script>
<script type="module">import "/island.js"; import "rsjs";</script>
<style>body { background: url(/bg.png) }</style>
<script type="application/ld+json">{"url":"/not-a-load"}</script>
</head><body>
<a class="nav" href='/about'>About</a>
<img srcset="/small.png 1x, /large.png 2x" src="/img.png" alt="">
<div style="background-image:url('/inline.png')"></div>
<a href="https://example.com/off-site">x</a><a href="/q?a=1&amp;b=2">q</a>
</body></html>"#;
        let urls = html_urls(source);
        for expected in [
            "/static/app.css",
            "/favicon.svg?v=1",
            "/rsjs-runtime.js",
            "/y.js",
            "/island.js",
            "/bg.png",
            "/about",
            "/small.png",
            "/large.png",
            "/img.png",
            "/inline.png",
            "https://example.com/off-site",
            "/q?a=1&b=2",
        ] {
            assert!(
                urls.iter().any(|url| url == expected),
                "{expected}: {urls:?}"
            );
        }
        for unexpected in ["/commented", "/not-a-load", "rsjs"] {
            assert!(
                !urls.iter().any(|url| url == unexpected),
                "{unexpected}: {urls:?}"
            );
        }
    }

    #[test]
    fn app_paths_stay_inside_the_base_path() {
        let origin = Url::parse(EXPORT_ORIGIN).unwrap();
        let base = BasePath::new("/cadrs").unwrap();
        let page = page_url(&origin, &base, "/docs/intro");
        let path = |reference: &str| app_path(&origin, &base, &page.join(reference).unwrap());
        assert_eq!(path("/cadrs/about").as_deref(), Some("/about"));
        assert_eq!(path("/cadrs").as_deref(), Some("/"));
        assert_eq!(path("next").as_deref(), Some("/docs/next"));
        assert_eq!(path("../app.css?v=2#x").as_deref(), Some("/app.css"));
        assert_eq!(path("/cadrsish/page"), None);
        assert_eq!(path("/other"), None);
        assert_eq!(path("https://example.com/cadrs/x"), None);
    }

    #[tokio::test]
    async fn export_writes_routes_their_references_redirects_and_not_found() {
        let out = std::env::temp_dir().join(format!(
            "proa-static-export-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&out);
        let app = Router::new()
            .route(
                "/",
                route_get(|| async {
                    Html(r#"<link rel="stylesheet" href="/app.css"><a href="/old">old</a><a href="/missing">gone</a>"#)
                }),
            )
            .route(
                "/app.css",
                route_get(|| async {
                    ([(header::CONTENT_TYPE, "text/css")], "a{background:url(img/dot.svg)}")
                }),
            )
            .route(
                "/img/dot.svg",
                route_get(|| async { ([(header::CONTENT_TYPE, "image/svg+xml")], "<svg/>") }),
            )
            .route("/old", route_get(|| async { Redirect::permanent("/new") }))
            .route("/new", route_get(|| async { Html("new page") }))
            .fallback(|| async { (StatusCode::NOT_FOUND, Html("not here")) });
        let spec = StaticExportSpec {
            out_dir: out.join("site"),
            report: out.join("report.json"),
            base_path: String::new(),
            routes: vec!["/".into()],
            optional_routes: vec!["/robots.txt".into()],
            follow_links: true,
            not_found_page: true,
        };
        let report = export(app.clone(), &spec).await.unwrap();
        let read = |file: &str| std::fs::read_to_string(spec.out_dir.join(file)).unwrap();
        assert!(read("index.html").contains("/app.css"));
        assert_eq!(read("app.css"), "a{background:url(img/dot.svg)}");
        assert_eq!(read("img/dot.svg"), "<svg/>");
        assert!(read("old/index.html").contains("url=/new"));
        assert_eq!(read("new/index.html"), "new page");
        assert_eq!(read("404.html"), "not here");
        assert!(!spec.out_dir.join("robots.txt").exists());
        assert_eq!(report.warnings.len(), 1, "{:?}", report.warnings);
        assert!(report.warnings[0].starts_with("/missing (referenced by /)"));
        assert!(spec.report.is_file());

        let failing = StaticExportSpec {
            routes: vec!["/nope".into()],
            ..spec.clone()
        };
        let error = export(app, &failing).await.unwrap_err();
        assert!(
            error.to_string().contains("/nope: responded 404"),
            "{error}"
        );
        let _ = std::fs::remove_dir_all(&out);
    }
}
