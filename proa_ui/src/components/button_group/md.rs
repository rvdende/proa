// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{ButtonGroup, ButtonGroupSeparator, ButtonGroupText};
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};
use proa_macros::md_sync;

/// ButtonGroup renders its children inline — the grouping chrome is meaningless
/// in markdown. In block context the labels form a standalone line, so it is
/// blank-line separated from surrounding blocks; inline the boundaries no-op.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for ButtonGroup<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        cx.md_block_boundary()?;
        let prev = cx.md_set_inline(true);
        self.children.render_md(cx)?;
        cx.md_set_inline(prev);
        cx.md_block_boundary()?;
        Ok(())
    }
}

/// ButtonGroupText renders its children as inline text.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for ButtonGroupText<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        let prev = cx.md_set_inline(true);
        self.children.render_md(cx)?;
        cx.md_set_inline(prev);
        Ok(())
    }
}

/// ButtonGroupSeparator renders as a single space so adjacent labels stay apart.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf> MdRenderSync<Loader, B>
    for ButtonGroupSeparator
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        md_sync! { " " }.render_md(cx)
    }
}
