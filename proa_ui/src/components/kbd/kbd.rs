// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Kbd component implementation
//!
//! Displays keyboard input or keyboard shortcuts.
//! Based on shadcn/ui styling.
use super::recipe::{KbdRecipeProps, KBD_RECIPE};
use proa_core::{WebContext, WebRenderSync, WriteBuf};
use proa_macros::html_sync;
/// Kbd component - displays a keyboard key
///
/// # Example
///
/// ```ignore
/// use crate::components::Kbd;
/// use proa_macros::html_sync;
///
/// html_sync! {
///     {Kbd {
///         children: Some("Ctrl"),
///         ..Default::default()
///     }}
/// }
/// ```
pub struct Kbd<C = ()> {
    /// Additional classes
    pub class: Option<&'static str>,
    /// Key content (text like "Ctrl", "Esc" or symbols like "⌘")
    pub children: Option<C>,
}
impl Kbd<()> {
    pub const fn new() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<C> Default for Kbd<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for Kbd<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = KBD_RECIPE.resolve(KbdRecipeProps);
        html_sync! {
            <kbd data-kbd data-slot="kbd" class={proa_macros::text!("{} {}", style, self.class)}>
                {self.children}
            </kbd>
        }
        .render(cx)
    }
}
/// KbdGroup component - groups multiple keyboard keys
///
/// # Example
///
/// ```ignore
/// use crate::components::{Kbd, KbdGroup};
/// use proa_macros::html_sync;
///
/// html_sync! {
///     {KbdGroup {
///         children: Some(html_sync! {
///             {Kbd { children: Some("Ctrl"), ..Default::default() }}
///             "+"
///             {Kbd { children: Some("K"), ..Default::default() }}
///         }),
///         ..Default::default()
///     }}
/// }
/// ```
pub struct KbdGroup<C = ()> {
    /// Additional classes
    pub class: Option<&'static str>,
    /// Group content (usually multiple Kbd components)
    pub children: Option<C>,
}
impl<C> Default for KbdGroup<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for KbdGroup<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <kbd
                data-slot="kbd-group"
                class={proa_macros::text!("{} {}", KBD_RECIPE.group, self.class)}
            >
                {self.children}
            </kbd>
        }
        .render(cx)
    }
}
