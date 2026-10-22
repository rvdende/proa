// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator,
};
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};
use proa_macros::md_sync;
use proa_macros::omit;

/// Breadcrumb is a passthrough.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for Breadcrumb<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        omit!(
            "browser-control-label",
            "Browser navigation or action label has no Markdown control."
        );
        self.children.render_md(cx)?;
        cx.md_blank_line()?;
        Ok(())
    }
}

/// BreadcrumbList is a passthrough.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for BreadcrumbList<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        self.children.render_md(cx)?;
        Ok(())
    }
}

/// BreadcrumbItem is a passthrough.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for BreadcrumbItem<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        self.children.render_md(cx)?;
        Ok(())
    }
}

/// BreadcrumbLink renders as `[text](url)`.
#[proa_macros::content(
    skip,
    reason = "Markdown delegates validated URL and child-label formatting to the context link writer"
)]
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for BreadcrumbLink<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        if let Some(href) = self.href {
            cx.md_link_with(href, |cx| self.children.render_md(cx))?;
        } else {
            self.children.render_md(cx)?;
        }
        Ok(())
    }
}

/// BreadcrumbPage renders as plain text (current page, no link).
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for BreadcrumbPage<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        self.children.render_md(cx)?;
        Ok(())
    }
}

/// BreadcrumbSeparator renders as ` / `.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C> MdRenderSync<Loader, B>
    for BreadcrumbSeparator<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        omit!(
            self.children,
            "Custom browser separator visuals are replaced by a slash in Markdown."
        );
        omit!(
            "separator-glyph",
            "The separator SVG is replaced by a slash in Markdown."
        );
        md_sync! { " / " }.render_md(cx)
    }
}

/// BreadcrumbEllipsis renders as `...`.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf> MdRenderSync<Loader, B> for BreadcrumbEllipsis {
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        omit!(
            "ellipsis-glyph",
            "Decorative browser SVG is replaced by the static text representation."
        );
        omit!(
            "browser-control-label",
            "Browser navigation or action label has no Markdown control."
        );
        md_sync! { "..." }.render_md(cx)
    }
}
