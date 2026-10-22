// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Select;
use crate::components::_proa::ListCollection;
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};
use proa_macros::md_sync;

/// Select renders its current value or placeholder text in brackets.
#[proa_macros::content(
    skip,
    reason = "Markdown presents the selected option or placeholder instead of the browser selection control"
)]
impl<Loader: ::proa_core::DataLoader, B: WriteBuf> MdRenderSync<Loader, B> for Select {
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        // Show the selected option's label, or the placeholder
        if let Some(val) = self.value {
            if let Some(opt) = ListCollection::new(&self.options).item(val) {
                md_sync! { "[" { opt.label } "]" }.render_md(cx)?;
                return Ok(());
            }
        }
        if let Some(placeholder) = self.placeholder {
            md_sync! { "[" { placeholder } "]" }.render_md(cx)?;
        }
        Ok(())
    }
}
