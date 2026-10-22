// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Collapsible component implementations
//!
//! SSR markup for collapsible disclosure widgets. The JavaScript hydration
//! handles the expand/collapse behavior.
use super::recipe::{CollapsibleRecipeProps, COLLAPSIBLE_RECIPE};
use super::COLLAPSIBLE_ANATOMY;
use crate::components::_proa::DisclosureState;
use proa_core::{WebContext, WebRenderSync, WriteBuf};
use proa_macros::html_sync;
/// Collapsible root component - wraps trigger and content
///
/// Renders a container div with data attributes for JS hydration.
/// The `open` field controls the initial state.
///
/// # Example
///
/// ```ignore
/// use crate::components::{Collapsible, CollapsibleTrigger, CollapsibleContent};
/// use proa_macros::html_sync;
///
/// html_sync! {
///     {Collapsible {
///         open: false,
///         children: Some(html_sync! {
///             {CollapsibleTrigger {
///                 children: Some("Toggle"),
///                 ..Default::default()
///             }}
///             {CollapsibleContent {
///                 children: Some("Hidden content"),
///                 ..Default::default()
///             }}
///         }),
///         ..Default::default()
///     }}
/// }
/// ```
pub struct Collapsible<C = ()> {
    /// Initial open/closed state
    pub open: bool,
    /// Whether the collapsible is disabled
    pub disabled: bool,
    /// Additional CSS classes
    pub class: Option<&'static str>,
    /// Optional id attribute
    pub id: Option<&'static str>,
    /// Child elements (trigger + content)
    pub children: Option<C>,
}
impl Collapsible<()> {
    pub const fn new() -> Self {
        Self {
            open: false,
            disabled: false,
            class: None,
            id: None,
            children: None,
        }
    }
}
impl<C> Default for Collapsible<C> {
    fn default() -> Self {
        Self {
            open: false,
            disabled: false,
            class: None,
            id: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for Collapsible<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = COLLAPSIBLE_RECIPE.resolve(CollapsibleRecipeProps);
        let state = DisclosureState::from_open(self.open);
        let disabled = if self.disabled { "true" } else { "false" };
        let initial_display = if state.hidden() { "none" } else { "block" };
        html_sync! {
            <div
                data-collapsible
                data-collapsible-root
                data-scope={COLLAPSIBLE_ANATOMY.scope()}
                data-part="root"
                data-slot="collapsible"
                data-state={state}
                data-disabled={disabled}
                data-collapsible-id={self.id}
                id={self.id}
                class={proa_macros::text!("{} {}", style, self.class)}
                style={proa_core::style_ok(proa_macros::text!(
                    "--proa-collapsible-display: {};",
                    initial_display
                ))}
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
/// CollapsibleTrigger - button that toggles the collapsible content
///
/// Renders a `<button>` element with `data-collapsible-trigger` for JS hydration.
pub struct CollapsibleTrigger<C = ()> {
    /// Additional CSS classes
    pub class: Option<&'static str>,
    /// Trigger content (usually text or an icon)
    pub children: Option<C>,
}
impl<C> Default for CollapsibleTrigger<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for CollapsibleTrigger<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <button
                type="button"
                data-collapsible-trigger
                data-scope={COLLAPSIBLE_ANATOMY.scope()}
                data-part="trigger"
                data-slot="collapsible-trigger"
                class={proa_macros::text!("{} {}", COLLAPSIBLE_RECIPE.trigger, self.class)}
            >
                {self.children}
            </button>
        }
        .render(cx)
    }
}
/// CollapsibleContent - the content area that is shown/hidden
///
/// The root owns the initial open state. Its inherited display variable makes
/// the first SSR frame truthful; the delegated runtime then projects state and
/// ARIA attributes onto this part.
pub struct CollapsibleContent<C = ()> {
    /// Additional CSS classes
    pub class: Option<&'static str>,
    /// The collapsible content
    pub children: Option<C>,
}
impl<C> Default for CollapsibleContent<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for CollapsibleContent<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <div
                data-collapsible-content
                data-scope={COLLAPSIBLE_ANATOMY.scope()}
                data-part="content"
                data-slot="collapsible-content"
                class={proa_macros::text!("{} {}", COLLAPSIBLE_RECIPE.content, self.class)}
                style="display: var(--proa-collapsible-display, none);"
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
