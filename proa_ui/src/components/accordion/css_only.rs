// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND (ISC AND MIT)

//! CSS-only Accordion component
//!
//! A zero-JavaScript accordion using the checkbox/radio input hack.
//! Works without JavaScript, supports both single and multiple modes.
//!
//! - Multiple mode: Uses checkboxes, multiple items can be open
//! - Single mode: Uses radio inputs, only one item can be open at a time
use super::recipe::{AccordionRecipeProps, ACCORDION_RECIPE};
use super::ACCORDION_ANATOMY;
use proa_core::{WebContext, WebRenderSync, WriteBuf};
use proa_macros::html_sync;
/// Accordion mode for CSS-only accordion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccordionCSSOnlyMode {
    /// Multiple items can be open (uses checkboxes)
    #[default]
    Multiple,
    /// Only one item can be open at a time (uses radio inputs)
    Single,
}
/// CSS-only Accordion root container
///
/// Uses the checkbox/radio input hack for pure CSS expand/collapse.
/// - `Multiple` mode: checkboxes allow multiple items open
/// - `Single` mode: radio inputs ensure only one item open at a time
pub struct AccordionCSSOnly<C = ()> {
    /// Accordion mode - single or multiple
    pub mode: AccordionCSSOnlyMode,
    /// Unique name for radio group (required for single mode)
    /// Items will use this to link their radio inputs
    pub name: &'static str,
    /// Additional classes
    pub class: Option<&'static str>,
    /// Accordion items
    pub children: Option<C>,
}
impl<C> Default for AccordionCSSOnly<C> {
    fn default() -> Self {
        Self {
            mode: AccordionCSSOnlyMode::Multiple,
            name: "accordion",
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for AccordionCSSOnly<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = ACCORDION_RECIPE.resolve(AccordionRecipeProps);
        let mode_str = match self.mode {
            AccordionCSSOnlyMode::Multiple => "multiple",
            AccordionCSSOnlyMode::Single => "single",
        };
        html_sync! {
            <div
                class={proa_macros::text!("{} {}", style, self.class)}
                data-accordion-css-only
                data-scope={ACCORDION_ANATOMY.scope()}
                data-part="root"
                data-slot="accordion-css-only"
                data-mode={mode_str}
                data-name={self.name}
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
/// CSS-only Accordion Item using checkbox/radio input hack
pub struct AccordionCSSOnlyItem<T = (), C = ()> {
    /// Unique ID for this item (used for input id and label for)
    pub id: &'static str,
    /// Group name - must match parent AccordionCSSOnly name for single mode
    pub name: &'static str,
    /// Whether to use radio (single mode) or checkbox (multiple mode)
    pub single_mode: bool,
    /// Whether this item is open by default
    pub open: bool,
    /// Whether this item is disabled
    pub disabled: bool,
    /// Additional classes
    pub class: Option<&'static str>,
    /// Trigger content (the clickable label)
    pub trigger: Option<T>,
    /// Content to show when expanded
    pub children: Option<C>,
}
impl<T, C> Default for AccordionCSSOnlyItem<T, C> {
    fn default() -> Self {
        Self {
            id: "",
            name: "accordion",
            single_mode: false,
            open: false,
            disabled: false,
            class: None,
            trigger: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, T: WebRenderSync<L, B>, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader>
    WebRenderSync<L, B> for AccordionCSSOnlyItem<T, C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let disabled_classes = self
            .disabled
            .then_some(ACCORDION_RECIPE.css_only_item_disabled);
        let input_type = if self.single_mode {
            "radio"
        } else {
            "checkbox"
        };
        if self.open {
            html_sync! {
                <div
                    data-scope={ACCORDION_ANATOMY.scope()}
                    data-part="item"
                    data-slot="accordion-css-only-item"
                    class={proa_macros::text!("{} {} {}", ACCORDION_RECIPE.css_only_item, disabled_classes, self.class)}
                >
                    <input
                        type={input_type}
                        name={self.name}
                        id={self.id}
                        data-scope={ACCORDION_ANATOMY.scope()}
                        data-part="input"
                        data-slot="accordion-css-only-input"
                        aria-controls={proa_macros::text!("{}-content", self.id)}
                        class={ACCORDION_RECIPE.css_only_input}
                        checked
                        disabled={self.disabled}
                    />
                    <label
                        id={proa_macros::text!("{}-trigger", self.id)}
                        for={self.id}
                        data-scope={ACCORDION_ANATOMY.scope()}
                        data-part="trigger"
                        data-slot="accordion-css-only-trigger"
                        class={ACCORDION_RECIPE.css_only_trigger}
                    >
                        {self.trigger}
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
                            class={ACCORDION_RECIPE.css_only_chevron}
                            aria-hidden="true"
                        >
                            <path d="m6 9 6 6 6-6" />
                        </svg>
                    </label>
                    <div
                        id={proa_macros::text!("{}-content", self.id)}
                        data-scope={ACCORDION_ANATOMY.scope()}
                        data-part="content"
                        data-slot="accordion-css-only-content"
                        role="region"
                        aria-labelledby={proa_macros::text!("{}-trigger", self.id)}
                        class={ACCORDION_RECIPE.css_only_content}
                    >
                        <div
                            data-scope={ACCORDION_ANATOMY.scope()}
                            data-part="content-inner"
                            data-slot="accordion-css-only-content-inner"
                            class={ACCORDION_RECIPE.css_only_content_inner}
                        >
                            <div
                                data-scope={ACCORDION_ANATOMY.scope()}
                                data-part="content-body"
                                data-slot="accordion-css-only-content-body"
                                class={ACCORDION_RECIPE.css_only_content_body}
                            >
                                {self.children}
                            </div>
                        </div>
                    </div>
                </div>
            }
            .render(cx)
        } else {
            html_sync! {
                <div
                    data-scope={ACCORDION_ANATOMY.scope()}
                    data-part="item"
                    data-slot="accordion-css-only-item"
                    class={proa_macros::text!("{} {} {}", ACCORDION_RECIPE.css_only_item, disabled_classes, self.class)}
                >
                    <input
                        type={input_type}
                        name={self.name}
                        id={self.id}
                        data-scope={ACCORDION_ANATOMY.scope()}
                        data-part="input"
                        data-slot="accordion-css-only-input"
                        aria-controls={proa_macros::text!("{}-content", self.id)}
                        class={ACCORDION_RECIPE.css_only_input}
                        disabled={self.disabled}
                    />
                    <label
                        id={proa_macros::text!("{}-trigger", self.id)}
                        for={self.id}
                        data-scope={ACCORDION_ANATOMY.scope()}
                        data-part="trigger"
                        data-slot="accordion-css-only-trigger"
                        class={ACCORDION_RECIPE.css_only_trigger}
                    >
                        {self.trigger}
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
                            class={ACCORDION_RECIPE.css_only_chevron}
                            aria-hidden="true"
                        >
                            <path d="m6 9 6 6 6-6" />
                        </svg>
                    </label>
                    <div
                        id={proa_macros::text!("{}-content", self.id)}
                        data-scope={ACCORDION_ANATOMY.scope()}
                        data-part="content"
                        data-slot="accordion-css-only-content"
                        role="region"
                        aria-labelledby={proa_macros::text!("{}-trigger", self.id)}
                        class={ACCORDION_RECIPE.css_only_content}
                    >
                        <div
                            data-scope={ACCORDION_ANATOMY.scope()}
                            data-part="content-inner"
                            data-slot="accordion-css-only-content-inner"
                            class={ACCORDION_RECIPE.css_only_content_inner}
                        >
                            <div
                                data-scope={ACCORDION_ANATOMY.scope()}
                                data-part="content-body"
                                data-slot="accordion-css-only-content-body"
                                class={ACCORDION_RECIPE.css_only_content_body}
                            >
                                {self.children}
                            </div>
                        </div>
                    </div>
                </div>
            }
            .render(cx)
        }
    }
}
