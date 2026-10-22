// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Markdown rendering for the radio group component.

use super::{RadioGroup, RadioGroupItem};
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};
use proa_macros::md_sync;
use proa_macros::omit;

/// RadioGroup renders its items as a bullet list followed by a blank line.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for RadioGroup<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        if let Some(label) = self.aria_label {
            cx.md_block_boundary()?;
            md_sync! { "**" { label } "**\n\n" }.render_md(cx)?;
        }

        cx.md_block_boundary()?;
        self.children.render_md(cx)?;
        cx.md_block_boundary()?;
        Ok(())
    }
}

/// RadioGroupItem renders as a list item with the selected option marked:
/// `- (•) value` when checked, `- ( ) value` otherwise.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf> MdRenderSync<Loader, B> for RadioGroupItem {
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        omit!(self.value, "Markdown displays the accessible name when present; the submission value is only its fallback.");
        omit!(self.aria_label, "Markdown uses the accessible name as a label with component-specific fallbacks and formatting.");
        let label = if let Some(label) = self.aria_label {
            label
        } else {
            self.value
        };
        // Fresh line, not a blank-line boundary: consecutive items in a
        // group must stay adjacent or the list would be split.
        cx.md_line_boundary()?;
        proa_macros::content_part!("markdown-list-layout", cx.write_md_list_indent())?;
        if self.checked {
            md_sync! { "- (•) " { label } "\n" }.render_md(cx)
        } else {
            md_sync! { "- ( ) " { label } "\n" }.render_md(cx)
        }
    }
}
