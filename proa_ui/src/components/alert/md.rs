// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{Alert, AlertDescription, AlertTitle, AlertVariant};
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};
use proa_macros::md_sync;

/// Alert renders as a blockquote with a bold prefix indicating the variant.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for Alert<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        cx.md_block_boundary()?;
        if self.variant == AlertVariant::Default {
            md_sync! { "> **Note:** " }.render_md(cx)?;
        } else {
            md_sync! { "> **Warning:** " }.render_md(cx)?;
        }
        let previous_inline = cx.md_set_inline(true);
        let result = self.children.render_md(cx);
        cx.md_set_inline(previous_inline);
        result?;
        md_sync! { "" "\n" }.render_md(cx)?;
        cx.md_block_boundary()
    }
}

/// AlertTitle renders its children as inline text (consumed by Alert's blockquote).
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for AlertTitle<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        self.children.render_md(cx)
    }
}

/// AlertDescription renders its children as inline text.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for AlertDescription<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        if !cx.md_inline() {
            md_sync! { " " }.render_md(cx)?;
        }
        self.children.render_md(cx)
    }
}
