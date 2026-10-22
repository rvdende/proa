// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Image component — optimized `<img>` with lazy loading, srcset, and CLS prevention.
pub use super::recipe::ImageFit;
use super::recipe::{ImageRecipeProps, IMAGE_RECIPE};
use proa_core::shared::attr_value::AttrValue;
use proa_core::{WebContext, WebRenderSync, WriteBuf};
/// Loading strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageLoading {
    /// Lazy-load when the image enters the viewport.
    #[default]
    Lazy,
    /// Load immediately (use for above-the-fold / LCP images).
    Eager,
}
impl ImageLoading {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lazy => "lazy",
            Self::Eager => "eager",
        }
    }
}
/// Fetch priority hint for the browser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImagePriority {
    #[default]
    Auto,
    High,
    Low,
}
impl ImagePriority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::High => "high",
            Self::Low => "low",
        }
    }
}
/// Optimized image component with built-in performance best practices.
///
/// Renders an `<img>` tag with:
/// - `loading="lazy"` by default (opt into `eager` for LCP images)
/// - `decoding="async"` for non-blocking decode
/// - `width` / `height` attributes to prevent CLS
/// - Optional `srcset` for responsive images
/// - `fetchpriority` hint for the browser
/// - `object-fit` via Tailwind class
///
/// # Example
///
/// ```ignore
/// use crate::components::image::{Image, ImageLoading, ImagePriority};
///
/// Image {
///     src: "/images/hero.jpg",
///     alt: "Hero banner",
///     width: Some(1200),
///     height: Some(630),
///     loading: ImageLoading::Eager,
///     priority: ImagePriority::High,
///     ..Default::default()
/// }
/// ```
pub struct Image {
    /// Image URL (required).
    pub src: &'static str,
    /// Alt text for accessibility (required).
    pub alt: &'static str,
    /// Intrinsic width in pixels — prevents layout shift.
    pub width: Option<u32>,
    /// Intrinsic height in pixels — prevents layout shift.
    pub height: Option<u32>,
    /// Responsive image sources (e.g. `"/img-480.jpg 480w, /img-800.jpg 800w"`).
    pub srcset: Option<&'static str>,
    /// Responsive sizes hint (e.g. `"(max-width: 600px) 480px, 800px"`).
    pub sizes: Option<&'static str>,
    /// Loading strategy. Default: lazy.
    pub loading: ImageLoading,
    /// Fetch priority. Default: auto.
    pub priority: ImagePriority,
    /// Object-fit mode.
    pub fit: ImageFit,
    /// Fill mode — image stretches to fill its parent container. When `true`,
    /// `width` and `height` are ignored and the image renders with
    /// `position:absolute; inset:0; width:100%; height:100%`. The parent
    /// element MUST have `position:relative` (or `absolute`/`fixed`).
    /// Equivalent to Next.js `<Image fill>`.
    pub fill: bool,
    /// Whether to apply rounded corners.
    pub rounded: bool,
    /// Additional CSS classes.
    pub class: Option<&'static str>,
    /// Optional blur placeholder data URI (e.g. a tiny base64 image).
    pub placeholder: Option<&'static str>,
}
impl Default for Image {
    fn default() -> Self {
        Self {
            src: "",
            alt: "",
            width: None,
            height: None,
            srcset: None,
            sizes: None,
            loading: ImageLoading::Lazy,
            priority: ImagePriority::Auto,
            fit: ImageFit::None,
            fill: false,
            rounded: false,
            class: None,
            placeholder: None,
        }
    }
}
impl<B: WriteBuf, L: ::proa_core::DataLoader> WebRenderSync<L, B> for Image {
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        proa_macros::content_part!("image-document", {
            let style = IMAGE_RECIPE.resolve(ImageRecipeProps {
                fit: Some(self.fit),
                fill: Some(self.fill),
                rounded: Some(self.rounded),
            });
            let classes = proa_macros::text!("{} {}", style, self.class);
            if let Some(placeholder) = self.placeholder {
                let placeholder_classes = proa_macros::text!(
                    "{} {}",
                    IMAGE_RECIPE.placeholder,
                    style.rounded().then_some(IMAGE_RECIPE.rounded)
                );
                let _ = cx // proa-perf-ok: trusted static markup emitted through the low-level writer
                    .out
                    .extend(
                        b"<div data-image-placeholder data-slot=\"image-placeholder\" class=\"",
                    );
                placeholder_classes.render_attr_value(cx)?;
                let _ = cx.out.extend(b"\" style=\"background-image:url('"); // proa-perf-ok: trusted static markup emitted through the low-level writer
                css_url_escape_into(placeholder, cx);
                let _ = cx.out.extend(b"');background-size:cover");
                if let (Some(w), Some(h)) = (self.width, self.height) {
                    let _ = cx.out.extend(b";aspect-ratio:");
                    write_u32(w, cx);
                    let _ = cx.out.extend(b"/");
                    write_u32(h, cx);
                }
                let _ = cx.out.extend(b"\">");
            }
            let _ = cx // proa-perf-ok: trusted static markup emitted through the low-level writer
                .out
                .extend(b"<img data-image data-slot=\"image\" data-fit=\"");
            let _ = cx.out.extend(style.fit().as_str().as_bytes());
            let _ = cx.out.extend(b"\" data-fill=\""); // proa-perf-ok: trusted static markup emitted through the low-level writer
            let _ = cx.out.extend(if style.fill() {
                &b"true"[..]
            } else {
                &b"false"[..]
            });
            let _ = cx.out.extend(b"\" data-rounded=\""); // proa-perf-ok: trusted static markup emitted through the low-level writer
            let _ = cx.out.extend(if style.rounded() {
                &b"true"[..]
            } else {
                &b"false"[..]
            });
            let _ = cx.out.extend(b"\"");
            let _ = cx.out.extend(b" src=\"");
            attr_escape_into(self.src, cx);
            let _ = cx.out.extend(b"\"");
            let _ = cx.out.extend(b" alt=\"");
            attr_escape_into(self.alt, cx);
            let _ = cx.out.extend(b"\"");
            let _ = cx.out.extend(b" class=\""); // proa-perf-ok: trusted static markup emitted through the low-level writer
            classes.render_attr_value(cx)?;
            let _ = cx.out.extend(b"\"");
            if !self.fill {
                if let Some(w) = self.width {
                    let _ = cx.out.extend(b" width=\"");
                    write_u32(w, cx);
                    let _ = cx.out.extend(b"\"");
                }
                if let Some(h) = self.height {
                    let _ = cx.out.extend(b" height=\"");
                    write_u32(h, cx);
                    let _ = cx.out.extend(b"\"");
                }
            }
            if let Some(srcset) = self.srcset {
                let _ = cx.out.extend(b" srcset=\"");
                attr_escape_into(srcset, cx);
                let _ = cx.out.extend(b"\"");
            }
            if let Some(sizes) = self.sizes {
                let _ = cx.out.extend(b" sizes=\"");
                attr_escape_into(sizes, cx);
                let _ = cx.out.extend(b"\"");
            }
            let _ = cx.out.extend(b" loading=\"");
            let _ = cx.out.extend(self.loading.as_str().as_bytes());
            let _ = cx.out.extend(b"\"");
            let _ = cx.out.extend(b" decoding=\"async\"");
            if self.priority != ImagePriority::Auto {
                let _ = cx.out.extend(b" fetchpriority=\"");
                let _ = cx.out.extend(self.priority.as_str().as_bytes());
                let _ = cx.out.extend(b"\"");
            }
            let _ = cx.out.extend(b" />");
            if self.placeholder.is_some() {
                let _ = cx.out.extend(b"</div>"); // proa-perf-ok: trusted static markup emitted through the low-level writer
            }
            Ok(())
        })
    }
}
fn write_u32<B: WriteBuf, L: proa_core::DataLoader>(n: u32, cx: &mut WebContext<L, B>) {
    let mut buf = [0u8; 10];
    let s = format_u32(n, &mut buf);
    let _ = cx.out.extend(s);
}
fn format_u32(mut n: u32, buf: &mut [u8; 10]) -> &[u8] {
    if n == 0 {
        buf[9] = b'0';
        return &buf[9..];
    }
    let mut i = 10;
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    &buf[i..]
}
fn attr_escape_into<B: WriteBuf, L: proa_core::DataLoader>(text: &str, cx: &mut WebContext<L, B>) {
    for ch in text.bytes() {
        match ch {
            b'<' => {
                let _ = cx.out.extend(b"&lt;");
            }
            b'>' => {
                let _ = cx.out.extend(b"&gt;");
            }
            b'&' => {
                let _ = cx.out.extend(b"&amp;");
            }
            b'"' => {
                let _ = cx.out.extend(b"&quot;");
            }
            _ => {
                let _ = cx.out.push(ch); // proa-perf-ok: pre-existing pattern grandfathered at framework pin fd7517be; tracked for burn-down
            }
        }
    }
}
/// Escape a URL for embedding in a CSS `url('...')` function inside a
/// double-quoted HTML `style` attribute. Handles both layers: CSS single-quote
/// / backslash escapes, and HTML `&` / `"` / `<` / `>` entity escapes.
/// Newline and control characters are dropped (they are never valid in CSS URL
/// tokens and are the primary vector for style-attribute breakout).
fn css_url_escape_into<B: WriteBuf, L: proa_core::DataLoader>(
    text: &str,
    cx: &mut WebContext<L, B>,
) {
    for ch in text.bytes() {
        match ch {
            0..=0x1f | 0x7f => {}
            b'\\' => {
                let _ = cx.out.extend(b"\\\\");
            }
            b'\'' => {
                let _ = cx.out.extend(b"\\'");
            }
            b'&' => {
                let _ = cx.out.extend(b"&amp;");
            }
            b'"' => {
                let _ = cx.out.extend(b"&quot;");
            }
            b'<' => {
                let _ = cx.out.extend(b"&lt;");
            }
            b'>' => {
                let _ = cx.out.extend(b"&gt;");
            }
            _ => {
                let _ = cx.out.push(ch); // proa-perf-ok: pre-existing pattern grandfathered at framework pin fd7517be; tracked for burn-down
            }
        }
    }
}
