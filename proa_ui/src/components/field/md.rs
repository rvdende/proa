// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Markdown rendering for the field components.

use super::{
    Field, FieldContent, FieldDescription, FieldError, FieldGroup, FieldLabel, FieldLegend,
    FieldSeparator, FieldSet, FieldTitle,
};
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};
use proa_macros::md_sync;

/// FieldSet is a passthrough container followed by a blank line.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for FieldSet<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        cx.md_block_boundary()?;
        self.children.render_md(cx)?;
        cx.md_block_boundary()?;
        Ok(())
    }
}

/// FieldLegend renders as bold text on its own line.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for FieldLegend<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        if let Some(children) = self.children {
            cx.md_block_boundary()?;
            let prev = cx.md_set_inline(true);
            md_sync! { "**" { children } "**" }.render_md(cx)?;
            cx.md_set_inline(prev);
            cx.md_block_boundary()?;
        }
        Ok(())
    }
}

/// FieldGroup is a passthrough container.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for FieldGroup<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        self.children.render_md(cx)?;
        Ok(())
    }
}

/// Field renders its children (label, control, help text) followed by a
/// blank line separating it from the next field.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for Field<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        cx.md_block_boundary()?;
        self.children.render_md(cx)?;
        cx.md_block_boundary()?;
        Ok(())
    }
}

/// FieldLabel renders as bold text on its own line, above the control.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for FieldLabel<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        if let Some(children) = self.children {
            cx.md_block_boundary()?;
            let prev = cx.md_set_inline(true);
            md_sync! { "**" { children } "**" }.render_md(cx)?;
            cx.md_set_inline(prev);
            cx.md_block_boundary()?;
        }
        Ok(())
    }
}

/// FieldContent is a passthrough container.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for FieldContent<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        self.children.render_md(cx)?;
        Ok(())
    }
}

/// FieldTitle renders as inline bold text.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for FieldTitle<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        if let Some(children) = self.children {
            let prev = cx.md_set_inline(true);
            md_sync! { "**" { children } "**" }.render_md(cx)?;
            cx.md_set_inline(prev);
        }
        Ok(())
    }
}

/// FieldDescription renders as italic help text.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for FieldDescription<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        if let Some(children) = self.children {
            cx.md_block_boundary()?;
            let prev = cx.md_set_inline(true);
            md_sync! { "*" { children } "*" }.render_md(cx)?;
            cx.md_set_inline(prev);
            cx.md_block_boundary()?;
        }
        Ok(())
    }
}

/// FieldError renders as a bold "Error:" prefix with italic message text.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for FieldError<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        if let Some(children) = self.children {
            cx.md_block_boundary()?;
            let prev = cx.md_set_inline(true);
            md_sync! { "**Error:** *" { children } "*" }.render_md(cx)?;
            cx.md_set_inline(prev);
            cx.md_block_boundary()?;
        }
        Ok(())
    }
}

/// FieldSeparator renders its centered content as an italic line, or a
/// horizontal rule when it has none.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for FieldSeparator<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        cx.md_block_boundary()?;
        if let Some(children) = self.children {
            let prev = cx.md_set_inline(true);
            md_sync! { "*" { children } "*" }.render_md(cx)?;
            cx.md_set_inline(prev);
            cx.md_block_boundary()?;
        } else {
            cx.md_hr()?;
        }
        Ok(())
    }
}
