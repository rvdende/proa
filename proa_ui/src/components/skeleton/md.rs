// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Skeleton;
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};

/// Skeleton is visual-only — renders nothing in markdown.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf> MdRenderSync<Loader, B> for Skeleton {
    fn render_md(self, _cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        Ok(())
    }
}
