// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::slider::resolve_slider_values;
use super::Slider;
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};
use proa_macros::md_sync;

#[proa_macros::content(
    skip,
    reason = "Markdown summarizes the current slider value without reproducing slider controls"
)]
impl<Loader: ::proa_core::DataLoader, B: WriteBuf> MdRenderSync<Loader, B> for Slider<'_> {
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        let values = resolve_slider_values(self.min, self.max, self.step, self.value);
        cx.md_block_boundary()?;
        md_sync! { { self.label } ": " }.render_md(cx)?;
        match self.value_text {
            Some(value_text) => value_text.render_md(cx)?,
            None => values.value.render_md(cx)?,
        }
        cx.md_block_boundary()
    }
}
