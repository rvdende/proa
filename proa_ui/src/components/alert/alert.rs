// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Alert component implementation
//!
//! Renders a callout for user attention with variant styling.
use super::recipe::{AlertRecipeProps, AlertVariant, ALERT_RECIPE};
use proa_core::{WebContext, WebRenderSync, WriteBuf};
use proa_macros::html_sync;
/// Alert component - displays a callout for user attention
pub struct Alert<C = ()> {
    /// Style variant
    pub variant: AlertVariant,
    /// Additional CSS classes
    pub class: Option<&'static str>,
    /// The alert content (title, description, etc.)
    pub children: Option<C>,
}
impl Alert<()> {
    pub const fn new() -> Self {
        Self {
            variant: AlertVariant::Default,
            class: None,
            children: None,
        }
    }
}
impl<C> Default for Alert<C> {
    fn default() -> Self {
        Self {
            variant: AlertVariant::Default,
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for Alert<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = ALERT_RECIPE.resolve(AlertRecipeProps {
            variant: Some(self.variant),
        });
        html_sync! {
            <div
                data-alert
                data-slot="alert"
                data-variant={style.variant().as_str()}
                role="alert"
                class={proa_macros::text!("{} {}", style, self.class)}
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
/// AlertTitle component - renders the title within an Alert
pub struct AlertTitle<C = ()> {
    /// Additional CSS classes
    pub class: Option<&'static str>,
    /// The title content
    pub children: Option<C>,
}
impl AlertTitle<()> {
    pub const fn new() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<C> Default for AlertTitle<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for AlertTitle<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <h5
                data-alert-title
                data-slot="alert-title"
                class={proa_macros::text!("{} {}", ALERT_RECIPE.title, self.class)}
            >
                {self.children}
            </h5>
        }
        .render(cx)
    }
}
/// AlertDescription component - renders the description within an Alert
pub struct AlertDescription<C = ()> {
    /// Additional CSS classes
    pub class: Option<&'static str>,
    /// The description content
    pub children: Option<C>,
}
impl AlertDescription<()> {
    pub const fn new() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<C> Default for AlertDescription<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for AlertDescription<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <div
                data-alert-description
                data-slot="alert-description"
                class={proa_macros::text!("{} {}", ALERT_RECIPE.description, self.class)}
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
