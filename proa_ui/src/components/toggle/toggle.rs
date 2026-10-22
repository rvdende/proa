// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Toggle component implementation
//!
//! A two-state button that can be either on or off.
//! Based on Radix UI Toggle with shadcn/ui styling.
use super::recipe::{ToggleRecipeProps, ToggleSize, ToggleVariant, TOGGLE_RECIPE};
use proa_core::{WebContext, WebRenderSync, WriteBuf};
use proa_macros::html_sync;
/// Toggle component - a two-state button
///
/// # Example
///
/// ```ignore
/// use crate::components::{Toggle, ToggleVariant, ToggleSize};
/// use proa_macros::html_sync;
///
/// html_sync! {
///     {Toggle {
///         pressed: false,
///         children: Some("B"),
///         ..Default::default()
///     }}
/// }
/// ```
pub struct Toggle<C = ()> {
    /// Whether the toggle is pressed (on)
    pub pressed: bool,
    /// Visual variant
    pub variant: ToggleVariant,
    /// Toggle size
    pub size: ToggleSize,
    /// Whether the toggle is disabled
    pub disabled: bool,
    /// ARIA label for accessibility
    pub aria_label: Option<&'static str>,
    /// Additional classes
    pub class: Option<&'static str>,
    /// Toggle content (usually an icon or text)
    pub children: Option<C>,
}
impl Toggle<()> {
    pub const fn new() -> Self {
        Self {
            pressed: false,
            variant: ToggleVariant::Default,
            size: ToggleSize::Default,
            disabled: false,
            aria_label: None,
            class: None,
            children: None,
        }
    }
}
impl<C> Default for Toggle<C> {
    fn default() -> Self {
        Self {
            pressed: false,
            variant: ToggleVariant::Default,
            size: ToggleSize::Default,
            disabled: false,
            aria_label: None,
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for Toggle<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = TOGGLE_RECIPE.resolve(ToggleRecipeProps {
            variant: Some(self.variant),
            size: Some(self.size),
        });
        let state = if self.pressed { "on" } else { "off" };
        let aria_pressed = if self.pressed { "true" } else { "false" };
        let disabled = if self.disabled { "true" } else { "false" };
        html_sync! {
            <button
                type="button"
                data-toggle
                data-slot="toggle"
                data-variant={style.variant().as_str()}
                data-size={style.size().as_str()}
                data-state={state}
                data-disabled={disabled}
                aria-pressed={aria_pressed}
                aria-label={self.aria_label}
                disabled={self.disabled}
                class={proa_macros::text!("{} {}", style, self.class)}
            >
                {self.children}
            </button>
        }
        .render(cx)
    }
}
