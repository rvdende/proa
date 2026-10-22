// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{
    Sheet, SheetClose, SheetContent, SheetDescription, SheetFooter, SheetHeader, SheetTitle,
    SheetTrigger,
};
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};
use proa_macros::md_sync;
use proa_macros::omit;

/// Sheet renders its content as a plain block — the trigger is interaction chrome and is dropped.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, Trigger, Content: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for Sheet<Trigger, Content>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        omit!(
            self.trigger,
            "Markdown renders the panel content without its opening control."
        );
        cx.md_block_boundary()?;
        self.content.render_md(cx)?;
        cx.md_block_boundary()?;
        Ok(())
    }
}

/// SheetTrigger is interaction chrome — renders nothing.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C> MdRenderSync<Loader, B> for SheetTrigger<C> {
    fn render_md(self, _cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        omit!(
            self.children,
            "Interaction labels are omitted from the static document."
        );
        Ok(())
    }
}

/// SheetContent renders its children as a plain block (side and close button are dropped).
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for SheetContent<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        omit!(
            "close-button",
            "Panel interaction chrome is omitted from the static document."
        );
        cx.md_block_boundary()?;
        self.children.render_md(cx)?;
        cx.md_block_boundary()?;
        Ok(())
    }
}

/// SheetHeader is a passthrough — its children (title, description) handle formatting.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for SheetHeader<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        self.children.render_md(cx)?;
        Ok(())
    }
}

/// SheetTitle renders as a bold line: `**title**`.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for SheetTitle<C>
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

/// SheetDescription renders as a plain paragraph.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for SheetDescription<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        cx.md_block_boundary()?;
        self.children.render_md(cx)?;
        cx.md_block_boundary()?;
        Ok(())
    }
}

/// SheetFooter is a passthrough for its children (action buttons render as labels).
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for SheetFooter<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        cx.md_block_boundary()?;
        self.children.render_md(cx)?;
        cx.md_block_boundary()?;
        Ok(())
    }
}

/// SheetClose is interaction chrome — renders nothing.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C> MdRenderSync<Loader, B> for SheetClose<C> {
    fn render_md(self, _cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        omit!(
            self.children,
            "Interaction labels are omitted from the static document."
        );
        Ok(())
    }
}
