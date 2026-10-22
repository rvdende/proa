// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{Tabs, TabsContent};
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};

/// Tabs is a passthrough — all tab content is visible in markdown.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for Tabs<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        self.children.render_md(cx)?;
        Ok(())
    }
}

/// TabsContent renders as a heading with the tab name, then content.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for TabsContent<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        cx.md_block_boundary()?;
        cx.md_heading_with(3, |cx| {
            proa_macros::md_sync! { { self.value } }.render_md(cx)
        })?;
        self.children.render_md(cx)?;
        cx.md_block_boundary()?;
        Ok(())
    }
}
