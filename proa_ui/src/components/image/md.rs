// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Image;
use proa_core::WebContext;
use proa_core::{MdRenderSync, WriteBuf, WriteError};
use proa_macros::md_sync;

/// Image renders as a standard markdown image: `![alt](src)`.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf> MdRenderSync<Loader, B> for Image {
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        proa_macros::content_part!("image-document", {
            md_sync! { "![" }.render_md(cx)?;
            proa_macros::content_part!(
                "image-alt",
                proa_macros::md_sync! { { self.alt } }.render_md(cx)
            )?;
            md_sync! { "](" }.render_md(cx)?;
            proa_macros::content_part!("image-src", cx.write_md_url(self.src))?;
            md_sync! { ")" }.render_md(cx)
        })
    }
}
