// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};
use proa_macros::md_sync;

/// Collapsible is a passthrough — trigger and content handle formatting.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for Collapsible<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        self.children.render_md(cx)?;
        cx.md_block_boundary()?;
        Ok(())
    }
}

/// CollapsibleTrigger renders its label as a bold line.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for CollapsibleTrigger<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        cx.md_block_boundary()?;
        let prev = cx.md_set_inline(true);
        md_sync! { "**" { self.children } "**" }.render_md(cx)?;
        cx.md_set_inline(prev);
        cx.md_block_boundary()?;
        Ok(())
    }
}

/// CollapsibleContent always renders its children in markdown (no collapse state).
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for CollapsibleContent<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        cx.md_block_boundary()?;
        self.children.render_md(cx)?;
        cx.md_block_boundary()?;
        Ok(())
    }
}
