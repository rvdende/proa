// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND (ISC AND MIT)

//! Accordion component implementations
//!
//! SSR markup for accordions. The JavaScript hydration handles the
//! expand/collapse behavior and animations.
use super::recipe::{AccordionRecipeProps, AccordionType, ACCORDION_RECIPE};
use super::ACCORDION_ANATOMY;
use proa_core::{WebContext, WebRenderSync, WriteBuf};
use proa_macros::html_sync;
/// Accordion root component - manages the accordion state
///
/// Supports two modes:
/// - `Single`: Only one item can be open at a time
/// - `Multiple`: Multiple items can be open simultaneously
pub struct Accordion<C = ()> {
    /// Accordion type (single or multiple)
    pub accordion_type: AccordionType,
    /// Whether items can be collapsed in single mode
    pub collapsible: bool,
    /// Default open item(s) - comma-separated values for multiple
    pub default_value: Option<&'static str>,
    /// Additional classes
    pub class: Option<&'static str>,
    /// Accordion items
    pub children: Option<C>,
}
impl Accordion<()> {
    pub const fn new() -> Self {
        Self {
            accordion_type: AccordionType::Single,
            collapsible: false,
            default_value: None,
            class: None,
            children: None,
        }
    }
}
impl<C> Default for Accordion<C> {
    fn default() -> Self {
        Self {
            accordion_type: AccordionType::Single,
            collapsible: false,
            default_value: None,
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for Accordion<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = ACCORDION_RECIPE.resolve(AccordionRecipeProps);
        let collapsible = if self.collapsible { "true" } else { "false" };
        html_sync! {
            <div
                data-accordion-root
                data-scope={ACCORDION_ANATOMY.scope()}
                data-part="root"
                data-slot="accordion"
                data-type={self.accordion_type.as_str()}
                data-collapsible={collapsible}
                data-value={self.default_value}
                class={proa_macros::text!("{} {}", style, self.class)}
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
/// AccordionItem - individual expandable section
pub struct AccordionItem<C = ()> {
    /// Unique value for this item
    pub value: &'static str,
    /// Whether this item is disabled
    pub disabled: bool,
    /// Additional classes
    pub class: Option<&'static str>,
    /// Item children (trigger + content)
    pub children: Option<C>,
}
impl<C> Default for AccordionItem<C> {
    fn default() -> Self {
        Self {
            value: "",
            disabled: false,
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for AccordionItem<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let disabled = if self.disabled { "true" } else { "false" };
        html_sync! {
            <div
                data-accordion-item
                data-scope={ACCORDION_ANATOMY.scope()}
                data-part="item"
                data-slot="accordion-item"
                data-value={self.value}
                data-state="closed"
                data-disabled={disabled}
                class={proa_macros::text!("{} {}", ACCORDION_RECIPE.item, self.class)}
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
/// AccordionTrigger - clickable header that toggles the content
pub struct AccordionTrigger<C = ()> {
    /// Additional classes
    pub class: Option<&'static str>,
    /// Trigger content (usually text)
    pub children: Option<C>,
}
impl<C> Default for AccordionTrigger<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for AccordionTrigger<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <h3
                data-scope={ACCORDION_ANATOMY.scope()}
                data-part="heading"
                data-slot="accordion-heading"
                class={ACCORDION_RECIPE.heading}
            >
                <button
                    type="button"
                    data-accordion-trigger
                    data-scope={ACCORDION_ANATOMY.scope()}
                    data-part="trigger"
                    data-slot="accordion-trigger"
                    aria-expanded="false"
                    class={proa_macros::text!("{} {}", ACCORDION_RECIPE.trigger, self.class)}
                >
                    {self.children}
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="16"
                        height="16"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class={ACCORDION_RECIPE.chevron}
                        aria-hidden="true"
                    >
                        <path d="m6 9 6 6 6-6" />
                    </svg>
                </button>
            </h3>
        }
        .render(cx)
    }
}
/// AccordionContent - the expandable content area
pub struct AccordionContent<C = ()> {
    /// Additional classes
    pub class: Option<&'static str>,
    /// Content to show when expanded
    pub children: Option<C>,
}
impl<C> Default for AccordionContent<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for AccordionContent<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <div
                data-accordion-content
                data-scope={ACCORDION_ANATOMY.scope()}
                data-part="content"
                data-slot="accordion-content"
                data-state="closed"
                role="region"
                hidden
                class={proa_macros::text!("{} {}", ACCORDION_RECIPE.content, self.class)}
            >
                <div
                    data-scope={ACCORDION_ANATOMY.scope()}
                    data-part="content-inner"
                    data-slot="accordion-content-inner"
                    class={ACCORDION_RECIPE.content_inner}
                >
                    {self.children}
                </div>
            </div>
        }
        .render(cx)
    }
}
