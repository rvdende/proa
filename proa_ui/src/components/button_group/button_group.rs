// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::recipe::{ButtonGroupOrientation, ButtonGroupRecipeProps, BUTTON_GROUP_RECIPE};
use proa_core::{WebContext, WebRenderSync, WriteBuf};
use proa_macros::html_sync;
pub struct ButtonGroup<C = ()> {
    pub orientation: Option<ButtonGroupOrientation>,
    pub children: Option<C>,
    pub class: Option<&'static str>,
    pub id: Option<&'static str>,
}
impl Default for ButtonGroup<()> {
    fn default() -> Self {
        Self::new()
    }
}
impl ButtonGroup<()> {
    pub const fn new() -> Self {
        Self {
            orientation: None,
            children: None,
            class: None,
            id: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for ButtonGroup<C>
{
    #[inline(always)]
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = BUTTON_GROUP_RECIPE.resolve(ButtonGroupRecipeProps {
            orientation: self.orientation,
        });
        let orientation = style.orientation();
        html_sync! {
            <div
                data-button-group
                role="group"
                data-slot="button-group"
                data-orientation={orientation.data_value()}
                class={proa_macros::text!("{} {}", style, self.class)}
                id={self.id}
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
/// ButtonGroupText - displays text/content within a button group
pub struct ButtonGroupText<C = ()> {
    pub children: Option<C>,
    pub class: Option<&'static str>,
    pub id: Option<&'static str>,
}
impl Default for ButtonGroupText<()> {
    fn default() -> Self {
        Self::new()
    }
}
impl ButtonGroupText<()> {
    pub const fn new() -> Self {
        Self {
            children: None,
            class: None,
            id: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for ButtonGroupText<C>
{
    #[inline(always)]
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <div
                data-button-group-text
                data-slot="button-group-text"
                class={proa_macros::text!("{} {}", BUTTON_GROUP_RECIPE.text, self.class)}
                id={self.id}
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
/// ButtonGroupSeparator - visual separator between button group items
pub struct ButtonGroupSeparator {
    pub orientation: Option<ButtonGroupOrientation>,
    pub class: Option<&'static str>,
    pub id: Option<&'static str>,
}
impl Default for ButtonGroupSeparator {
    fn default() -> Self {
        Self::new()
    }
}
impl ButtonGroupSeparator {
    pub const fn new() -> Self {
        Self {
            orientation: Some(ButtonGroupOrientation::Vertical),
            class: None,
            id: None,
        }
    }
}
impl<B: WriteBuf, L: ::proa_core::DataLoader> WebRenderSync<L, B> for ButtonGroupSeparator {
    #[inline(always)]
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let orientation = self.orientation.unwrap_or(ButtonGroupOrientation::Vertical);
        html_sync! {
            <div
                data-button-group-separator
                data-slot="button-group-separator"
                data-orientation={orientation.data_value()}
                role="separator"
                aria-hidden="true"
                class={proa_macros::text!("{} {}", BUTTON_GROUP_RECIPE.separator, self.class)}
                id={self.id}
            ></div>
        }
        .render(cx)
    }
}
