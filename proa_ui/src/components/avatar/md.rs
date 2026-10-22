// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{Avatar, AvatarFallback, AvatarImage};
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};
use proa_macros::md_sync;
use proa_macros::omit;

/// Avatar renders its image when present, otherwise its fallback text.
impl<
        Loader: ::proa_core::DataLoader,
        B: WriteBuf,
        Image: MdRenderSync<Loader, B>,
        Fallback: MdRenderSync<Loader, B>,
    > MdRenderSync<Loader, B> for Avatar<Image, Fallback>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        omit!(
            self.fallback,
            "Markdown shows fallback content only when there is no image."
        );
        if let Some(image) = self.image {
            image.render_md(cx)
        } else {
            self.fallback.render_md(cx)
        }
    }
}

/// AvatarImage renders as a standard Markdown image: `![alt](src)`.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf> MdRenderSync<Loader, B> for AvatarImage {
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        md_sync! { "![" }.render_md(cx)?;
        proa_macros::content_part!(
            "image-alt",
            proa_macros::md_sync! { { self.alt } }.render_md(cx)
        )?;
        md_sync! { "](" }.render_md(cx)?;
        proa_macros::content_part!("image-src", cx.write_md_url(self.src))?;
        md_sync! { ")" }.render_md(cx)
    }
}

/// AvatarFallback renders its children inline (typically initials).
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for AvatarFallback<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        let prev = cx.md_set_inline(true);
        let result = self.children.render_md(cx);
        cx.md_set_inline(prev);
        result
    }
}
