// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Label;
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};

/// Label renders as bold text.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf> MdRenderSync<Loader, B> for Label {
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        proa_macros::md_sync! { "**" { self.text } "**" }.render_md(cx)?;
        Ok(())
    }
}
