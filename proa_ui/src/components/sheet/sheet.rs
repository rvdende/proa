// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND (ISC AND MIT)

//! Sheet component implementations
//!
//! A Sheet is a side-sliding panel (like a Dialog that slides from an edge).
//! It reuses the Dialog JS handler by emitting `data-dialog-*` attributes,
//! so open/close, overlay clicks, focus trap, and body scroll lock all work
//! automatically without additional JavaScript.
use super::recipe::{SheetRecipeProps, SheetSide, SHEET_RECIPE};
use super::SHEET_ANATOMY;
use proa_core::{StaticRaw, WebContext, WebRenderSync, WriteBuf, WriteError};
use proa_macros::html_sync;

const CLOSE_ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>"#;
/// Sheet root component
///
/// Renders a container with `data-dialog-root` so the existing Dialog JS handler
/// manages all interactive behavior (open/close, focus trap, scroll lock).
pub struct Sheet<Trigger = (), Content = ()> {
    pub default_open: bool,
    pub id: Option<&'static str>,
    pub trigger: Option<Trigger>,
    pub content: Option<Content>,
}
impl<Trigger, Content> Default for Sheet<Trigger, Content> {
    fn default() -> Self {
        Self {
            default_open: false,
            id: None,
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
    > WebRenderSync<L, B> for Sheet<Trigger, Content>
{
    fn render(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        let state = crate::components::_proa::OverlayState::new(self.default_open, true);
        let initial_display = if state.hidden() { "none" } else { "block" };
        html_sync! {
            <div
                id={self.id}
                data-dialog-root
                data-sheet
                data-scope={SHEET_ANATOMY.scope()}
                data-part="root"
                data-slot="sheet"
                data-state={state.data_state()}
                data-dialog-id={self.id}
                class={SHEET_RECIPE.root}
                style={proa_core::style_ok(proa_macros::text!("--proa-overlay-display: {};", initial_display))}
            >
                {self.trigger}
                {self.content}
            </div>
        }
        .render(cx)
    }
}
/// SheetTrigger - the element that opens the sheet
pub struct SheetTrigger<C = ()> {
    pub class: Option<&'static str>,
    pub children: Option<C>,
}
impl<C> Default for SheetTrigger<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for SheetTrigger<C>
{
    fn render(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        html_sync! {
            <button
                type="button"
                data-dialog-trigger
                data-sheet-trigger
                data-scope={SHEET_ANATOMY.scope()}
                data-part="trigger"
                data-slot="sheet-trigger"
                aria-haspopup="dialog"
                class={proa_macros::text!("{} {}", SHEET_RECIPE.trigger, self.class)}
            >
                {self.children}
            </button>
        }
        .render(cx)
    }
}
/// SheetContent - the sliding panel container
pub struct SheetContent<C = ()> {
    pub side: SheetSide,
    pub class: Option<&'static str>,
    pub children: Option<C>,
    pub show_close: bool,
}
impl<C> Default for SheetContent<C> {
    fn default() -> Self {
        Self {
            side: SheetSide::Right,
            class: None,
            children: None,
            show_close: true,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for SheetContent<C>
{
    fn render(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        let style = SHEET_RECIPE.resolve(SheetRecipeProps {
            side: Some(self.side),
        });
        let side = style.side().data_value();
        html_sync! {
            <div
                data-dialog-portal
                data-scope={SHEET_ANATOMY.scope()}
                data-part="portal"
                data-slot="sheet-portal"
                style="display: var(--proa-overlay-display, none);"
            >
                <div
                    data-dialog-overlay
                    data-sheet-overlay
                    data-scope={SHEET_ANATOMY.scope()}
                    data-part="overlay"
                    data-slot="sheet-overlay"
                    class={SHEET_RECIPE.overlay}
                    aria-hidden="true"
                ></div>
                <div
                    data-sheet-content
                    data-scope={SHEET_ANATOMY.scope()}
                    data-part="content"
                    data-slot="sheet-content"
                    data-side={side}
                    class={proa_macros::text!("{} {}", style, self.class)}
                    role="dialog"
                    aria-modal="true"
                    tabindex="-1"
                >
                    {self.children}
                    {proa_macros::content_part!("close-button", self.show_close.then(|| html_sync! {
                            <button type="button" data-dialog-close data-sheet-close data-scope={SHEET_ANATOMY.scope()} data-part="close" data-slot="sheet-close" class={SHEET_RECIPE.close}>
                                <span data-scope={SHEET_ANATOMY.scope()} data-part="close-label" data-slot="sheet-close-label" class={SHEET_RECIPE.close_label}>
                                    "Close"
                                </span>
                                {StaticRaw(CLOSE_ICON)}
                            </button>
                        }))}
                </div>
            </div>
        }
            .render(cx)
    }
}
/// SheetHeader - container for title and description
pub struct SheetHeader<C = ()> {
    pub class: Option<&'static str>,
    pub children: Option<C>,
}
impl<C> Default for SheetHeader<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for SheetHeader<C>
{
    fn render(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        html_sync! {
            <div
                data-sheet-header
                data-scope={SHEET_ANATOMY.scope()}
                data-part="header"
                data-slot="sheet-header"
                class={proa_macros::text!("{} {}", SHEET_RECIPE.header, self.class)}
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
/// SheetTitle - accessible title
pub struct SheetTitle<C = ()> {
    pub class: Option<&'static str>,
    pub children: Option<C>,
}
impl<C> Default for SheetTitle<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for SheetTitle<C>
{
    fn render(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        html_sync! {
            <h2
                data-sheet-title
                data-scope={SHEET_ANATOMY.scope()}
                data-part="title"
                data-slot="sheet-title"
                class={proa_macros::text!("{} {}", SHEET_RECIPE.title, self.class)}
            >
                {self.children}
            </h2>
        }
        .render(cx)
    }
}
/// SheetDescription - optional description text
pub struct SheetDescription<C = ()> {
    pub class: Option<&'static str>,
    pub children: Option<C>,
}
impl<C> Default for SheetDescription<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for SheetDescription<C>
{
    fn render(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        html_sync! {
            <p
                data-sheet-description
                data-scope={SHEET_ANATOMY.scope()}
                data-part="description"
                data-slot="sheet-description"
                class={proa_macros::text!("{} {}", SHEET_RECIPE.description, self.class)}
            >
                {self.children}
            </p>
        }
        .render(cx)
    }
}
/// SheetFooter - container for action buttons
pub struct SheetFooter<C = ()> {
    pub class: Option<&'static str>,
    pub children: Option<C>,
}
impl<C> Default for SheetFooter<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for SheetFooter<C>
{
    fn render(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        html_sync! {
            <div
                data-sheet-footer
                data-scope={SHEET_ANATOMY.scope()}
                data-part="footer"
                data-slot="sheet-footer"
                class={proa_macros::text!("{} {}", SHEET_RECIPE.footer, self.class)}
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
/// SheetClose - element that closes the sheet when clicked
pub struct SheetClose<C = ()> {
    pub class: Option<&'static str>,
    pub children: Option<C>,
}
impl<C> Default for SheetClose<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for SheetClose<C>
{
    fn render(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        html_sync! {
            <span
                data-dialog-close
                data-sheet-close
                data-scope={SHEET_ANATOMY.scope()}
                data-part="close"
                data-slot="sheet-close"
                class={proa_macros::text!("{} {}", SHEET_RECIPE.close, self.class)}
            >
                {self.children}
            </span>
        }
        .render(cx)
    }
}
