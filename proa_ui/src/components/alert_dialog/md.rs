// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};
use proa_macros::md_sync;
use proa_macros::omit;

/// AlertDialog renders its content as a plain block — the trigger is interaction chrome and is dropped.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, Trigger, Content: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for AlertDialog<Trigger, Content>
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

/// AlertDialogTrigger is interaction chrome — renders nothing.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C> MdRenderSync<Loader, B>
    for AlertDialogTrigger<C>
{
    fn render_md(self, _cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        omit!(
            self.children,
            "Interaction labels are omitted from the static document."
        );
        Ok(())
    }
}

/// AlertDialogContent renders its children as a plain block.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for AlertDialogContent<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        cx.md_block_boundary()?;
        self.children.render_md(cx)?;
        cx.md_block_boundary()?;
        Ok(())
    }
}

/// AlertDialogHeader is a passthrough — its children (title, description) handle formatting.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for AlertDialogHeader<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        self.children.render_md(cx)?;
        Ok(())
    }
}

/// AlertDialogTitle renders as a bold line: `**title**`.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for AlertDialogTitle<C>
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

/// AlertDialogDescription renders as a plain paragraph.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for AlertDialogDescription<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        cx.md_block_boundary()?;
        self.children.render_md(cx)?;
        cx.md_block_boundary()?;
        Ok(())
    }
}

/// AlertDialogFooter is a passthrough for its children (action/cancel labels).
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for AlertDialogFooter<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        cx.md_block_boundary()?;
        self.children.render_md(cx)?;
        cx.md_block_boundary()?;
        Ok(())
    }
}

/// AlertDialogAction renders as just its label text (one per line in block context).
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for AlertDialogAction<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        let prev = cx.md_set_inline(true);
        let result = self.children.render_md(cx);
        cx.md_set_inline(prev);
        result?;
        if !cx.md_inline() {
            md_sync! { "" "\n" }.render_md(cx)?;
        }
        Ok(())
    }
}

/// AlertDialogCancel renders as just its label text (one per line in block context).
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for AlertDialogCancel<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        let prev = cx.md_set_inline(true);
        let result = self.children.render_md(cx);
        cx.md_set_inline(prev);
        result?;
        if !cx.md_inline() {
            md_sync! { "" "\n" }.render_md(cx)?;
        }
        Ok(())
    }
}
