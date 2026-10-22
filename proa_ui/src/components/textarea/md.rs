// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Markdown rendering for the textarea component.

use super::Textarea;
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};
use proa_macros::md_sync;
use proa_macros::omit;

/// Textarea renders its value — as a fenced code block when multiline,
/// as plain text otherwise. With no value, the placeholder is shown in
/// brackets (matching Input's convention).
#[proa_macros::content(
    skip,
    reason = "Markdown presents the value or empty-control placeholder, rather than textarea metadata and interactive state"
)]
impl<Loader: ::proa_core::DataLoader, B: WriteBuf> MdRenderSync<Loader, B> for Textarea {
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        omit!(self.aria_label, "Markdown uses the accessible name as a label with component-specific fallbacks and formatting.");
        omit!(
            self.placeholder,
            "Markdown uses the placeholder only when the textarea has no current value."
        );
        if let Some(value) = self.value {
            if value
                .as_bytes()
                .iter()
                .any(|byte| matches!(byte, b'\r' | b'\n'))
            {
                cx.md_fenced_code("", value)?;
            } else {
                proa_macros::md_sync! { { value } }.render_md(cx)?;
            }
        } else if let Some(placeholder) = self.placeholder {
            md_sync! { "[" { placeholder } "]" }.render_md(cx)?;
        }
        Ok(())
    }
}
