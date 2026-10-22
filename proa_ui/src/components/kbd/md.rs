// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{Kbd, KbdGroup};
use proa_core::WebContext;
use proa_core::{MdCodeContent, MdRenderSync, WriteBuf, WriteError};

/// Kbd renders as inline code.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdCodeContent> MdRenderSync<Loader, B>
    for Kbd<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        cx.write_md_inline_code(
            self.children
                .as_ref()
                .map_or("", MdCodeContent::md_code_content),
        )
    }
}

/// KbdGroup just renders children (keys) separated by spaces.
impl<Loader: ::proa_core::DataLoader, B: WriteBuf, C: MdRenderSync<Loader, B>>
    MdRenderSync<Loader, B> for KbdGroup<C>
{
    fn render_md(self, cx: &mut WebContext<Loader, B>) -> Result<(), WriteError> {
        self.children.render_md(cx)?;
        Ok(())
    }
}
