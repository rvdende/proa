// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};

/// Accordion is a passthrough — children handle their own rendering.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for Accordion<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        self.children.render_md(cx)?;
        Ok(())
    }
}

/// AccordionItem is a passthrough.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for AccordionItem<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        self.children.render_md(cx)?;
        Ok(())
    }
}

/// AccordionTrigger renders as a heading (level 3 + offset).
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for AccordionTrigger<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        cx.md_block_boundary()?;
        cx.md_heading_with(3, |cx| self.children.render_md(cx))
    }
}

/// AccordionContent always renders in markdown (no collapse state).
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for AccordionContent<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        self.children.render_md(cx)?;
        cx.md_blank_line()?;
        Ok(())
    }
}
