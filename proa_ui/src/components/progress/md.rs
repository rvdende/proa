// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{CircularProgress, Progress};
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};
use proa_macros::md_sync;

/// Progress renders as a "Progress: 45%" paragraph.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf> MdRenderSync<Loader, B> for Progress {
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        cx.md_block_boundary()?;
        if let Some(label) = self.aria_label {
            md_sync! { { label } ": " }.render_md(cx)?;
        } else {
            md_sync! { "Progress: " }.render_md(cx)?;
        }
        md_sync! { { self.value } "%" }.render_md(cx)?;
        cx.md_block_boundary()?;
        Ok(())
    }
}

impl<Loader: ::proa_core::DataLoader, B: WriteBuf> MdRenderSync<Loader, B> for CircularProgress {
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        cx.md_block_boundary()?;
        if let Some(label) = self.aria_label {
            md_sync! { { label } ": " }.render_md(cx)?;
        } else {
            md_sync! { "Progress: " }.render_md(cx)?;
        }
        md_sync! { { proa_macros::content_part!("progress-percentage", self.value.min(100)) } "%" }
            .render_md(cx)?;
        cx.md_block_boundary()?;
        Ok(())
    }
}
