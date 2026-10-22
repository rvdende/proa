// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Skeleton component implementation
use super::recipe::{SkeletonRecipeProps, SKELETON_RECIPE};
use proa_core::{WebContext, WebRenderSync, WriteBuf};
use proa_macros::html_sync;
/// Skeleton - an animated loading placeholder
///
/// Renders a `<div>` with a pulse animation and muted background.
/// Size and shape are controlled via the `class` prop.
///
/// # Example
/// ```ignore
/// Skeleton { class: Some("h-4 w-[250px]"), id: None }
/// Skeleton { class: Some("h-12 w-12 rounded-full"), id: None }
/// ```
#[derive(Default)]
pub struct Skeleton {
    pub class: Option<&'static str>,
    pub id: Option<&'static str>,
}
impl<B: WriteBuf, L: ::proa_core::DataLoader> WebRenderSync<L, B> for Skeleton {
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = SKELETON_RECIPE.resolve(SkeletonRecipeProps);
        html_sync! {
            <div
                data-skeleton
                data-slot="skeleton"
                class={proa_macros::text!("{} {}", style, self.class)}
                id={self.id}
            ></div>
        }
        .render(cx)
    }
}
