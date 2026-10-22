// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Markdown rendering for the theme toggle component.

use super::ThemeToggle;
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};
use proa_macros::omit;

/// A theme toggle is interactive page chrome with no Markdown equivalent —
/// a document has no theme to switch — so it renders nothing, like other
/// purely presentational controls.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf> MdRenderSync<Loader, B> for ThemeToggle {
    fn render_md(self, _cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        omit!(
            self.aria_label,
            "A static document has no theme-switching control."
        );
        Ok(())
    }
}
