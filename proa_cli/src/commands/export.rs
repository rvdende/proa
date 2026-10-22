// Copyright Proa Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! `proa export`: render a built release into a directory of static files.
//!
//! The release binary does the rendering. This command writes an export spec
//! and starts the binary with `PROA_STATIC_EXPORT` naming it;
//! `proa_framework_axum::serve` then sends each route through the
//! application's router in process and writes the responses instead of
//! serving. The spec and report formats are `StaticExportSpec` and
//! `StaticExportReport` in `proa_framework_axum::static_export`.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{bail, Context, Result};
use clap::Args;
use serde_json::json;

use crate::{commands::start::release_executable, scaffold, util::resolve_config_root};

/// The report the binary writes; its presence also marks a directory as a
/// previous export that may be replaced.
const REPORT_FILE: &str = ".proa-export.json";

/// Root files a host serves by convention, exported when the site has them.
const CONVENTIONAL_FILES: &[&str] = &[
    "/robots.txt",
    "/sitemap.xml",
    "/favicon.ico",
    "/manifest.webmanifest",
];

#[derive(Debug, Args)]
pub(crate) struct ExportArgs {
    /// Proa project root. Defaults to the nearest root.
    #[arg(long)]
    root: Option<PathBuf>,
    /// Exact release executable path. Relative paths resolve from the project root.
    #[arg(long)]
    bin: Option<PathBuf>,
    /// Output directory. Relative paths resolve from the project root. A
    /// previous export there is replaced; any other non-empty directory is refused.
    #[arg(long, default_value = "dist")]
    out: PathBuf,
    /// URL prefix the site is published under, such as `/my-repo` for a
    /// GitHub Pages project site. Sets `PROA_BASE_PATH` for the render.
    #[arg(long)]
    base_path: Option<String>,
    /// Another path to render, such as one behind a dynamic route. Repeatable.
    #[arg(long = "route", value_name = "PATH")]
    routes: Vec<String>,
    /// Render only the listed routes, not the pages and assets they reference.
    #[arg(long)]
    no_follow: bool,
    /// Do not write the application's not-found page to `404.html`.
    #[arg(long)]
    no_404: bool,
}

pub(crate) fn run(args: ExportArgs) -> Result<()> {
    let root = resolve_config_root(args.root)?;
    let executable = release_executable(&root, args.bin)?;
    let out_dir = if args.out.is_absolute() {
        args.out
    } else {
        root.join(args.out)
    };
    prepare_out_dir(&out_dir)?;
    let base_path = args.base_path.unwrap_or_default();

    let mut routes = vec!["/".to_string()];
    for route in scaffold::site_routes(Some(root.clone()))? {
        if route.method == "GET"
            && matches!(route.kind, "page" | "representation")
            && !is_pattern(&route.path)
        {
            routes.push(route.path);
        }
    }
    for route in args.routes {
        if is_pattern(&route) {
            bail!("--route {route} is a pattern; pass a concrete path such as /blog/first-post");
        }
        routes.push(route);
    }
    let mut optional_routes: Vec<String> = CONVENTIONAL_FILES
        .iter()
        .map(|path| path.to_string())
        .collect();
    optional_routes.extend(public_files(&root.join("public"))?);

    let spec_dir = root.join("target").join("proa-export");
    fs::create_dir_all(&spec_dir).with_context(|| format!("create {}", spec_dir.display()))?;
    let spec_file = spec_dir.join("spec.json");
    let report_file = out_dir.join(REPORT_FILE);
    let spec = json!({
        "out_dir": out_dir,
        "report": report_file,
        "base_path": base_path,
        "routes": routes,
        "optional_routes": optional_routes,
        "follow_links": !args.no_follow,
        "not_found_page": !args.no_404,
    });
    fs::write(&spec_file, serde_json::to_vec_pretty(&spec)?)
        .with_context(|| format!("write {}", spec_file.display()))?;

    let status = Command::new(&executable)
        .current_dir(&root)
        .env("PROA_STATIC_EXPORT", &spec_file)
        .env("PROA_BASE_PATH", &base_path)
        // The application binds its listener before `serve` sees the export
        // request; an ephemeral loopback port keeps that from colliding.
        .env("PROA_HOST", "127.0.0.1")
        .env("PORT", "0")
        .status()
        .with_context(|| format!("start {}", executable.display()))?;
    if !status.success() {
        bail!(
            "static export failed: {} exited with {status}",
            executable.display()
        );
    }
    let report = fs::read(&report_file).map_err(|_| {
        anyhow::anyhow!(
            "{} exited without exporting; it must start through proa_framework_axum::serve",
            executable.display()
        )
    })?;
    let report: serde_json::Value = serde_json::from_slice(&report)
        .with_context(|| format!("read {}", report_file.display()))?;

    // GitHub Pages would otherwise run Jekyll, which drops `_`-prefixed paths.
    fs::write(out_dir.join(".nojekyll"), "")?;

    let files = report["files"].as_array().map_or(&[][..], Vec::as_slice);
    let pages = files
        .iter()
        .filter(|file| file["content_type"] == "text/html")
        .count();
    let warnings = report["warnings"].as_array().map_or(&[][..], Vec::as_slice);
    for warning in warnings {
        eprintln!("warning: {}", warning.as_str().unwrap_or_default());
    }
    println!(
        "Exported {pages} pages and {} other files to {}",
        files.len() - pages,
        out_dir.display()
    );
    Ok(())
}

/// Whether a route path has parameters or a wildcard.
fn is_pattern(path: &str) -> bool {
    path.contains([':', '{', '*'])
}

/// Empty `out_dir`, or refuse when it holds something other than an export.
fn prepare_out_dir(out_dir: &Path) -> Result<()> {
    let Ok(mut entries) = fs::read_dir(out_dir) else {
        return Ok(());
    };
    if entries.next().is_none() {
        return Ok(());
    }
    if !out_dir.join(REPORT_FILE).is_file() {
        bail!(
            "{} is not empty and holds no previous export ({REPORT_FILE}); \
             choose another --out or empty it",
            out_dir.display()
        );
    }
    fs::remove_dir_all(out_dir).with_context(|| format!("remove {}", out_dir.display()))
}

/// `/static/<path>` for every file in the project's `public/` directory, the
/// mount scaffolded sites serve it at.
fn public_files(public: &Path) -> Result<Vec<String>> {
    if !public.is_dir() {
        return Ok(Vec::new());
    }
    let mut paths = Vec::new();
    for entry in walkdir::WalkDir::new(public).sort_by_file_name() {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let relative = entry.path().strip_prefix(public)?;
        let segments: Vec<_> = relative
            .components()
            .map(|component| component.as_os_str().to_string_lossy())
            .collect();
        paths.push(format!("/static/{}", segments.join("/")));
    }
    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patterns_are_not_rendered() {
        assert!(is_pattern("/blog/:slug"));
        assert!(is_pattern("/blog/{slug}"));
        assert!(is_pattern("/files/*rest"));
        assert!(!is_pattern("/blog/first-post"));
    }

    #[test]
    fn out_dir_is_replaced_only_when_it_holds_an_export() {
        let dir = tempfile::tempdir().unwrap();
        let out = dir.path().join("dist");
        prepare_out_dir(&out).unwrap();
        fs::create_dir_all(&out).unwrap();
        prepare_out_dir(&out).unwrap();
        fs::write(out.join("keep.txt"), "mine").unwrap();
        assert!(prepare_out_dir(&out).is_err());
        assert!(out.join("keep.txt").is_file());
        fs::write(out.join(REPORT_FILE), "{}").unwrap();
        prepare_out_dir(&out).unwrap();
        assert!(!out.exists());
    }

    #[test]
    fn public_files_map_to_the_static_mount() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("fonts")).unwrap();
        fs::write(dir.path().join("styles.css"), "").unwrap();
        fs::write(dir.path().join("fonts/a.woff2"), "").unwrap();
        assert_eq!(
            public_files(dir.path()).unwrap(),
            ["/static/fonts/a.woff2", "/static/styles.css"]
        );
    }
}
