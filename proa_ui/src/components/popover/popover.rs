// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Popover component implementations
//!
//! SSR markup for popovers. The TypeScript hydration handles positioning,
//! open/close state, and click-outside detection.
use super::recipe::{PopoverAlign, PopoverRecipeProps, PopoverSide, POPOVER_RECIPE};
use super::POPOVER_ANATOMY;
use crate::components::_proa::OverlayState;
use proa_core::{WebContext, WebRenderSync, WriteBuf};
use proa_macros::html_sync;
/// Popover root component - manages the popover state
pub struct Popover<Trigger = (), Content = ()> {
    /// Whether the popover starts open
    pub default_open: bool,
    /// Preferred side for the popover
    pub side: PopoverSide,
    /// Alignment relative to trigger
    pub align: PopoverAlign,
    /// The trigger element
    pub trigger: Option<Trigger>,
    /// The popover content
    pub content: Option<Content>,
}
impl Popover<(), ()> {
    pub const fn new() -> Self {
        Self {
            default_open: false,
            side: PopoverSide::Bottom,
            align: PopoverAlign::Center,
            trigger: None,
            content: None,
        }
    }
}
impl<Trigger, Content> Default for Popover<Trigger, Content> {
    fn default() -> Self {
        Self {
            default_open: false,
            side: PopoverSide::Bottom,
            align: PopoverAlign::Center,
            trigger: None,
            content: None,
        }
    }
}
impl<
        B: WriteBuf,
        Trigger: WebRenderSync<L, B>,
        Content: WebRenderSync<L, B>,
        L: ::proa_core::DataLoader,
    > WebRenderSync<L, B> for Popover<Trigger, Content>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = POPOVER_RECIPE.resolve(PopoverRecipeProps {
            side: Some(self.side),
            align: Some(self.align),
        });
        let state = OverlayState::new(self.default_open, false);
        let initial_display = if state.hidden() { "none" } else { "block" };
        html_sync! {
            <div
                data-popover-root
                data-scope={POPOVER_ANATOMY.scope()}
                data-part="root"
                data-slot="popover"
                data-state={state.data_state()}
                data-side={style.side().as_str()}
                data-align={style.align().as_str()}
                class={style}
                style={proa_core::style_ok(proa_macros::text!(
                    "--proa-overlay-display: {};",
                    initial_display
                ))}
            >
                {self.trigger}
                {self.content}
            </div>
        }
        .render(cx)
    }
}
/// PopoverTrigger - the element that opens the popover
pub struct PopoverTrigger<C = ()> {
    pub class: Option<&'static str>,
    pub children: Option<C>,
}
impl<C> Default for PopoverTrigger<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for PopoverTrigger<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <button
                type="button"
                data-popover-trigger
                data-scope={POPOVER_ANATOMY.scope()}
                data-part="trigger"
                data-slot="popover-trigger"
                aria-haspopup="dialog"
                class={proa_macros::text!("{} {}", POPOVER_RECIPE.trigger, self.class)}
            >
                {self.children}
            </button>
        }
        .render(cx)
    }
}
/// PopoverContent - the popover container
pub struct PopoverContent<C = ()> {
    pub class: Option<&'static str>,
    pub children: Option<C>,
}
impl<C> Default for PopoverContent<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for PopoverContent<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <div
                data-popover-content
                data-scope={POPOVER_ANATOMY.scope()}
                data-part="content"
                data-slot="popover-content"
                role="dialog"
                tabindex="-1"
                class={proa_macros::text!("{} {}", POPOVER_RECIPE.content, self.class)}
                style="display: var(--proa-overlay-display, none); position: absolute;"
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
/// PopoverClose - button that closes the popover
pub struct PopoverClose<C = ()> {
    pub class: Option<&'static str>,
    pub children: Option<C>,
}
impl<C> Default for PopoverClose<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for PopoverClose<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <button
                type="button"
                data-popover-close
                data-scope={POPOVER_ANATOMY.scope()}
                data-part="close"
                data-slot="popover-close"
                class={proa_macros::text!("{} {}", POPOVER_RECIPE.close, self.class)}
            >
                {self.children}
            </button>
        }
        .render(cx)
    }
}
