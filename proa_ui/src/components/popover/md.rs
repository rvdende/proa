// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{Popover, PopoverClose, PopoverContent, PopoverTrigger};
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};
use proa_macros::omit;

/// Popover renders its trigger inline, then its content as a following block.
impl<
        Loader: ::proa_core::DataLoader,
        B: WriteBuf,
        Trigger: MdRenderSync<Loader, B>,
        Content: MdRenderSync<Loader, B>,
    > MdRenderSync<Loader, B> for Popover<Trigger, Content>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        cx.md_block_boundary()?;
        let prev = cx.md_set_inline(true);
        self.trigger.render_md(cx)?;
        cx.md_set_inline(prev);
        cx.md_block_boundary()?;
        self.content.render_md(cx)?;
        cx.md_block_boundary()?;
        Ok(())
    }
}

/// PopoverTrigger renders as just its label text — the button chrome is meaningless in markdown.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for PopoverTrigger<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        let prev = cx.md_set_inline(true);
        self.children.render_md(cx)?;
        cx.md_set_inline(prev);
        Ok(())
    }
}

/// PopoverContent renders its children as a plain block.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for PopoverContent<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        cx.md_block_boundary()?;
        self.children.render_md(cx)?;
        cx.md_block_boundary()?;
        Ok(())
    }
}

/// PopoverClose is interaction chrome — renders nothing.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C> MdRenderSync<Loader, B> for PopoverClose<C> {
    fn render_md(self, _cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        omit!(
            self.children,
            "Interaction labels are omitted from the static document."
        );
        Ok(())
    }
}
