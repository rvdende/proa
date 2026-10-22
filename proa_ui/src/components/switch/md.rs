// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Markdown rendering for the switch component.

use super::Switch;
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};
use proa_macros::md_sync;

/// Switch renders as `**Label:** on/off` (label from aria-label or name),
/// or just `on`/`off` when unlabeled.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf> MdRenderSync<Loader, B> for Switch {
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        let state = if self.checked { "on" } else { "off" };
        let label = if self.aria_label.is_some() {
            self.aria_label
        } else {
            self.name
        };
        if let Some(label) = label {
            // The labeled form is a block-level `**Label:** state` line.
            cx.md_block_boundary()?;
            md_sync! { "**" { label } ":** " { state } }.render_md(cx)?;
            cx.md_block_boundary()?;
        } else {
            // Unlabeled, the bare state is inline text.
            md_sync! { { state } }.render_md(cx)?;
        }
        Ok(())
    }
}
