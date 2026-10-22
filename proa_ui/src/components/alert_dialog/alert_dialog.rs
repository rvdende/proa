// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Alert Dialog component implementations
//!
//! Thin wrappers around the Dialog component with `role="alertdialog"` semantics.
//! AlertDialog is used for destructive action confirmations where the user must
//! explicitly confirm or cancel.
use super::recipe::ALERT_DIALOG_RECIPE;
use super::ALERT_DIALOG_ANATOMY;
use crate::components::_proa::OverlayState;
use proa_core::{WebContext, WebRenderSync, WriteBuf};
use proa_macros::html_sync;
/// AlertDialog root component - manages the alert dialog state
///
/// Wraps the Dialog component with `data-alert-dialog-root` for hydration.
/// The TypeScript code handles the interactive behaviors but treats this
/// as a non-dismissible dialog (no click-outside close, no escape close).
pub struct AlertDialog<Trigger = (), Content = ()> {
    /// Whether the dialog starts open (for SSR)
    pub default_open: bool,
    /// Dialog ID for linking trigger and content
    pub id: Option<&'static str>,
    /// The trigger element (button that opens dialog)
    pub trigger: Option<Trigger>,
    /// The dialog content
    pub content: Option<Content>,
}
impl AlertDialog<(), ()> {
    pub const fn new() -> Self {
        Self {
            default_open: false,
            id: None,
            trigger: None,
            content: None,
        }
    }
}
impl<Trigger, Content> Default for AlertDialog<Trigger, Content> {
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
    > WebRenderSync<L, B> for AlertDialog<Trigger, Content>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let state = OverlayState::new(self.default_open, true);
        let initial_display = if state.hidden() { "none" } else { "block" };
        html_sync! {
            <div
                data-alert-dialog-root
                data-dialog-root
                data-scope={ALERT_DIALOG_ANATOMY.scope()}
                data-part="root"
                data-slot="alert-dialog"
                data-state={state.data_state()}
                data-dialog-id={self.id}
                id={self.id}
                class={ALERT_DIALOG_RECIPE.root}
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
/// AlertDialogTrigger - the element that opens the alert dialog
///
/// Wraps DialogTrigger with `aria-haspopup="dialog"`.
pub struct AlertDialogTrigger<C = ()> {
    pub class: Option<&'static str>,
    pub children: Option<C>,
}
impl<C> Default for AlertDialogTrigger<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for AlertDialogTrigger<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <button
                type="button"
                data-dialog-trigger
                data-scope={ALERT_DIALOG_ANATOMY.scope()}
                data-part="trigger"
                data-slot="alert-dialog-trigger"
                aria-haspopup="dialog"
                class={proa_macros::text!("{} {}", ALERT_DIALOG_RECIPE.trigger, self.class)}
            >
                {self.children}
            </button>
        }
        .render(cx)
    }
}
/// AlertDialogContent - the modal content container with `role="alertdialog"`
///
/// Unlike DialogContent, this uses `role="alertdialog"` and does not render
/// a close (X) button, enforcing explicit action/cancel interactions.
pub struct AlertDialogContent<C = ()> {
    pub class: Option<&'static str>,
    pub children: Option<C>,
}
impl<C> Default for AlertDialogContent<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for AlertDialogContent<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <div
                data-dialog-portal
                data-scope={ALERT_DIALOG_ANATOMY.scope()}
                data-part="portal"
                data-slot="alert-dialog-portal"
                style="display: var(--proa-overlay-display, none);"
            >
                <div
                    data-dialog-overlay
                    data-scope={ALERT_DIALOG_ANATOMY.scope()}
                    data-part="overlay"
                    data-slot="alert-dialog-overlay"
                    class={ALERT_DIALOG_RECIPE.overlay}
                    aria-hidden="true"
                />
                <div
                    data-dialog-content
                    data-scope={ALERT_DIALOG_ANATOMY.scope()}
                    data-part="content"
                    data-slot="alert-dialog-content"
                    class={proa_macros::text!("{} {}", ALERT_DIALOG_RECIPE.content, self.class)}
                    role="alertdialog"
                    aria-modal="true"
                    tabindex="-1"
                >
                    {self.children}
                </div>
            </div>
        }
        .render(cx)
    }
}
/// AlertDialogHeader - container for title and description
pub struct AlertDialogHeader<C = ()> {
    pub class: Option<&'static str>,
    pub children: Option<C>,
}
impl<C> Default for AlertDialogHeader<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for AlertDialogHeader<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <div
                data-scope={ALERT_DIALOG_ANATOMY.scope()}
                data-part="header"
                data-slot="alert-dialog-header"
                class={proa_macros::text!("{} {}", ALERT_DIALOG_RECIPE.header, self.class)}
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
/// AlertDialogTitle - accessible alert dialog title
pub struct AlertDialogTitle<C = ()> {
    pub class: Option<&'static str>,
    pub children: Option<C>,
}
impl<C> Default for AlertDialogTitle<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for AlertDialogTitle<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <h2
                data-dialog-title
                data-scope={ALERT_DIALOG_ANATOMY.scope()}
                data-part="title"
                data-slot="alert-dialog-title"
                class={proa_macros::text!("{} {}", ALERT_DIALOG_RECIPE.title, self.class)}
            >
                {self.children}
            </h2>
        }
        .render(cx)
    }
}
/// AlertDialogDescription - description text explaining the destructive action
pub struct AlertDialogDescription<C = ()> {
    pub class: Option<&'static str>,
    pub children: Option<C>,
}
impl<C> Default for AlertDialogDescription<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for AlertDialogDescription<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <p
                data-dialog-description
                data-scope={ALERT_DIALOG_ANATOMY.scope()}
                data-part="description"
                data-slot="alert-dialog-description"
                class={proa_macros::text!("{} {}", ALERT_DIALOG_RECIPE.description, self.class)}
            >
                {self.children}
            </p>
        }
        .render(cx)
    }
}
/// AlertDialogFooter - container for action and cancel buttons
pub struct AlertDialogFooter<C = ()> {
    pub class: Option<&'static str>,
    pub children: Option<C>,
}
impl<C> Default for AlertDialogFooter<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for AlertDialogFooter<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <div
                data-scope={ALERT_DIALOG_ANATOMY.scope()}
                data-part="footer"
                data-slot="alert-dialog-footer"
                class={proa_macros::text!("{} {}", ALERT_DIALOG_RECIPE.footer, self.class)}
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
/// AlertDialogAction - destructive/primary action button
///
/// Styled with destructive colors. Includes `data-dialog-close` so clicking
/// this button closes the alert dialog via the hydration code.
pub struct AlertDialogAction<C = ()> {
    pub class: Option<&'static str>,
    pub children: Option<C>,
}
impl<C> Default for AlertDialogAction<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for AlertDialogAction<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <button
                type="button"
                data-dialog-close
                data-alert-dialog-action
                data-scope={ALERT_DIALOG_ANATOMY.scope()}
                data-part="action"
                data-slot="alert-dialog-action"
                class={proa_macros::text!("{} {}", ALERT_DIALOG_RECIPE.action, self.class)}
            >
                {self.children}
            </button>
        }
        .render(cx)
    }
}
/// AlertDialogCancel - cancel/dismiss button
///
/// Styled with outline/secondary colors. Includes `data-dialog-close` so
/// clicking this button closes the alert dialog via the hydration code.
pub struct AlertDialogCancel<C = ()> {
    pub class: Option<&'static str>,
    pub children: Option<C>,
}
impl<C> Default for AlertDialogCancel<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for AlertDialogCancel<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <button
                type="button"
                data-dialog-close
                data-alert-dialog-cancel
                data-scope={ALERT_DIALOG_ANATOMY.scope()}
                data-part="cancel"
                data-slot="alert-dialog-cancel"
                class={proa_macros::text!("{} {}", ALERT_DIALOG_RECIPE.cancel, self.class)}
            >
                {self.children}
            </button>
        }
        .render(cx)
    }
}
