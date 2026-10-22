// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Markdown rendering for the checkbox component.

use super::{Checkbox, CheckboxState};
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};
use proa_macros::md_sync;
use proa_macros::omit;

/// Checkbox renders as a task-list item: `- [x]` / `- [ ]` / `- [-]` for
/// indeterminate, followed by the aria-label when one is set.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf> MdRenderSync<Loader, B> for Checkbox {
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        omit!(
            self.value,
            "The submitted checkbox value is not part of the task-list label."
        );
        // Fresh line (not a full blank-line boundary): a lone checkbox after
        // prose needs its own line, but consecutive task items must stay
        // adjacent with no blank line between them.
        cx.md_line_boundary()?;
        proa_macros::content_part!("markdown-list-layout", cx.write_md_list_indent())?;
        if self.state == CheckboxState::Checked {
            md_sync! { "- [x]" }.render_md(cx)?;
        } else if self.state == CheckboxState::Indeterminate {
            md_sync! { "- [-]" }.render_md(cx)?;
        } else {
            md_sync! { "- [ ]" }.render_md(cx)?;
        }
        if let Some(label) = self.aria_label {
            md_sync! { " " { label } }.render_md(cx)?;
        }
        md_sync! { "" "\n" }.render_md(cx)
    }
}
