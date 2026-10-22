// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Markdown rendering for the toggle component.

use super::Toggle;
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};
use proa_macros::md_sync;
use proa_macros::omit;

/// Toggle renders as a task-list item — `- [x] Label` when pressed,
/// `- [ ] Label` when not. The label comes from the children (rendered
/// inline) or the aria-label.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for Toggle<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        omit!(self.aria_label, "Markdown uses the accessible name as a label with component-specific fallbacks and formatting.");
        let mark = if self.pressed { "x" } else { " " };
        // Fresh line (not a full blank-line boundary): a lone toggle after
        // prose needs its own line, but consecutive toggles in a group must
        // stay adjacent with no blank line between them.
        cx.md_line_boundary()?;
        proa_macros::content_part!("markdown-list-layout", cx.write_md_list_indent())?;
        md_sync! { "- [" { mark } "] " }.render_md(cx)?;
        let prev = cx.md_set_inline(true);
        if let Some(children) = self.children {
            children.render_md(cx)?;
        } else if let Some(label) = self.aria_label {
            proa_macros::md_sync! { { label } }.render_md(cx)?;
        }
        cx.md_set_inline(prev);
        md_sync! { "" "\n" }.render_md(cx)
    }
}
