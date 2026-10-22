// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Field component implementation
//!
//! Composable form field components for building accessible forms.
//! Based on shadcn/ui Field component.
use super::recipe::{FieldLegendVariant, FieldOrientation, FieldRecipeProps, FIELD_RECIPE};
use super::FIELD_ANATOMY;
use proa_core::{WebContext, WebRenderSync, WriteBuf};
use proa_macros::html_sync;
/// FieldSet component - semantic container for form field groups
///
/// # Example
///
/// ```ignore
/// use crate::components::FieldSet;
/// use proa_macros::html_sync;
///
/// html_sync! {
///     {FieldSet {
///         children: Some(html_sync! {
///             // FieldLegend and Field components here
///         }),
///         ..Default::default()
///     }}
/// }
/// ```
pub struct FieldSet<C = ()> {
    /// Additional classes
    pub class: Option<&'static str>,
    /// Children content
    pub children: Option<C>,
}
impl<C> Default for FieldSet<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for FieldSet<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <fieldset
                data-scope={FIELD_ANATOMY.scope()}
                data-part="fieldset"
                data-slot="fieldset"
                class={proa_macros::text!("{} {}", FIELD_RECIPE.fieldset, self.class)}
            >
                {self.children}
            </fieldset>
        }
        .render(cx)
    }
}
/// FieldLegend component - legend for FieldSet
///
/// # Example
///
/// ```ignore
/// use crate::components::{FieldLegend, FieldLegendVariant};
///
/// html_sync! {
///     {FieldLegend {
///         variant: FieldLegendVariant::Legend,
///         children: Some("Personal Information"),
///         ..Default::default()
///     }}
/// }
/// ```
pub struct FieldLegend<C = ()> {
    /// Style variant
    pub variant: FieldLegendVariant,
    /// Additional classes
    pub class: Option<&'static str>,
    /// Children content
    pub children: Option<C>,
}
impl<C> Default for FieldLegend<C> {
    fn default() -> Self {
        Self {
            variant: FieldLegendVariant::Legend,
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for FieldLegend<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <legend
                data-scope={FIELD_ANATOMY.scope()}
                data-part="legend"
                data-slot="field-legend"
                data-variant={self.variant.as_str()}
                class={proa_macros::text!("{} {} {}", FIELD_RECIPE.legend, self.variant.classes(), self.class)}
            >
                {self.children}
            </legend>
        }
        .render(cx)
    }
}
/// FieldGroup component - container with CSS container queries
///
/// # Example
///
/// ```ignore
/// use crate::components::FieldGroup;
///
/// html_sync! {
///     {FieldGroup {
///         children: Some(html_sync! {
///             // Multiple Field components
///         }),
///         ..Default::default()
///     }}
/// }
/// ```
pub struct FieldGroup<C = ()> {
    /// Additional classes
    pub class: Option<&'static str>,
    /// Children content
    pub children: Option<C>,
}
impl<C> Default for FieldGroup<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for FieldGroup<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <div
                data-scope={FIELD_ANATOMY.scope()}
                data-part="group"
                data-slot="field-group"
                role="group"
                class={proa_macros::text!("{} {}", FIELD_RECIPE.group, self.class)}
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
/// Field component - wrapper for form field with label, input, and helpers
///
/// # Example
///
/// ```ignore
/// use crate::components::{Field, FieldOrientation};
///
/// html_sync! {
///     {Field {
///         orientation: FieldOrientation::Vertical,
///         invalid: false,
///         children: Some(html_sync! {
///             // FieldLabel, Input, FieldDescription, FieldError
///         }),
///         ..Default::default()
///     }}
/// }
/// ```
pub struct Field<C = ()> {
    /// Layout orientation
    pub orientation: FieldOrientation,
    /// Whether the field is in invalid state
    pub invalid: bool,
    /// Additional classes
    pub class: Option<&'static str>,
    /// Children content
    pub children: Option<C>,
}
impl<C> Default for Field<C> {
    fn default() -> Self {
        Self {
            orientation: FieldOrientation::Vertical,
            invalid: false,
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for Field<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = FIELD_RECIPE.resolve(FieldRecipeProps {
            orientation: Some(self.orientation),
        });
        let state = crate::components::_proa::FormControlState {
            invalid: self.invalid,
            ..crate::components::_proa::FormControlState::enabled()
        };
        html_sync! {
            <div
                data-field
                data-scope={FIELD_ANATOMY.scope()}
                data-part="root"
                data-slot="field"
                role="group"
                data-orientation={style.orientation().as_str()}
                data-invalid={state.data_invalid()}
                aria-invalid={state.aria_invalid()}
                class={proa_macros::text!("{} {}", style, self.class)}
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
/// FieldLabel component - accessible label for form controls
///
/// # Example
///
/// ```ignore
/// use crate::components::FieldLabel;
///
/// html_sync! {
///     {FieldLabel {
///         for_id: Some("email"),
///         disabled: false,
///         checked: false,
///         children: Some("Email"),
///         ..Default::default()
///     }}
/// }
/// ```
pub struct FieldLabel<C = ()> {
    /// ID of the form element this label is for
    pub for_id: Option<&'static str>,
    /// Whether the associated control is disabled
    pub disabled: bool,
    /// Whether the associated control is checked (for checkbox/radio)
    pub checked: bool,
    /// Additional classes
    pub class: Option<&'static str>,
    /// Children content
    pub children: Option<C>,
}
impl<C> Default for FieldLabel<C> {
    fn default() -> Self {
        Self {
            for_id: None,
            disabled: false,
            checked: false,
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for FieldLabel<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let disabled_attr = if self.disabled { Some("true") } else { None };
        let checked_attr = if self.checked { Some("true") } else { None };
        html_sync! {
            <label
                data-scope={FIELD_ANATOMY.scope()}
                data-part="label"
                data-slot="label"
                for={self.for_id}
                data-disabled={disabled_attr}
                data-checked={checked_attr}
                class={proa_macros::text!("{} {}", FIELD_RECIPE.label, self.class)}
            >
                {self.children}
            </label>
        }
        .render(cx)
    }
}
/// FieldContent component - flex container for control and description
///
/// # Example
///
/// ```ignore
/// use crate::components::FieldContent;
///
/// html_sync! {
///     {FieldContent {
///         children: Some(html_sync! {
///             // Input and FieldDescription
///         }),
///         ..Default::default()
///     }}
/// }
/// ```
pub struct FieldContent<C = ()> {
    /// Additional classes
    pub class: Option<&'static str>,
    /// Children content
    pub children: Option<C>,
}
impl<C> Default for FieldContent<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for FieldContent<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <div
                data-scope={FIELD_ANATOMY.scope()}
                data-part="content"
                data-slot="field-content"
                class={proa_macros::text!("{} {}", FIELD_RECIPE.content, self.class)}
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
/// FieldTitle component - semantic title for field content
///
/// # Example
///
/// ```ignore
/// use crate::components::FieldTitle;
///
/// html_sync! {
///     {FieldTitle {
///         children: Some("Newsletter Subscription"),
///         ..Default::default()
///     }}
/// }
/// ```
pub struct FieldTitle<C = ()> {
    /// Additional classes
    pub class: Option<&'static str>,
    /// Children content
    pub children: Option<C>,
}
impl<C> Default for FieldTitle<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for FieldTitle<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <span
                data-scope={FIELD_ANATOMY.scope()}
                data-part="title"
                data-slot="field-title"
                class={proa_macros::text!("{} {}", FIELD_RECIPE.title, self.class)}
            >
                {self.children}
            </span>
        }
        .render(cx)
    }
}
/// FieldDescription component - helper/description text for form fields
///
/// # Example
///
/// ```ignore
/// use crate::components::FieldDescription;
///
/// html_sync! {
///     {FieldDescription {
///         children: Some("Enter your email address"),
///         ..Default::default()
///     }}
/// }
/// ```
pub struct FieldDescription<C = ()> {
    /// Additional classes
    pub class: Option<&'static str>,
    /// Children content
    pub children: Option<C>,
}
impl<C> Default for FieldDescription<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for FieldDescription<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <p
                data-scope={FIELD_ANATOMY.scope()}
                data-part="description"
                data-slot="field-description"
                class={proa_macros::text!("{} {}", FIELD_RECIPE.description, self.class)}
            >
                {self.children}
            </p>
        }
        .render(cx)
    }
}
/// FieldError component - accessible error message container
///
/// Hidden by default, shown when parent Field has `data-invalid="true"`.
///
/// # Example
///
/// ```ignore
/// use crate::components::FieldError;
///
/// html_sync! {
///     {FieldError {
///         children: Some("This field is required"),
///         ..Default::default()
///     }}
/// }
/// ```
pub struct FieldError<C = ()> {
    /// Additional classes
    pub class: Option<&'static str>,
    /// Children content (single error message or list)
    pub children: Option<C>,
}
impl<C> Default for FieldError<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for FieldError<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <div
                data-scope={FIELD_ANATOMY.scope()}
                data-part="error"
                data-slot="field-error"
                role="alert"
                class={proa_macros::text!("{} {}", FIELD_RECIPE.error, self.class)}
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
/// FieldSeparator component - divider with optional centered content
///
/// # Example
///
/// ```ignore
/// use crate::components::FieldSeparator;
///
/// // Simple separator
/// html_sync! {
///     {FieldSeparator::<()> {
///         children: None,
///         ..Default::default()
///     }}
/// }
///
/// // Separator with text
/// html_sync! {
///     {FieldSeparator {
///         children: Some("or"),
///         ..Default::default()
///     }}
/// }
/// ```
pub struct FieldSeparator<C = ()> {
    /// Additional classes
    pub class: Option<&'static str>,
    /// Optional content to display centered over separator
    pub children: Option<C>,
}
impl<C> Default for FieldSeparator<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for FieldSeparator<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <div
                data-scope={FIELD_ANATOMY.scope()}
                data-part="separator"
                data-slot="field-separator"
                class={proa_macros::text!("{} {}", FIELD_RECIPE.separator, self.class)}
            >
                <hr
                    data-scope={FIELD_ANATOMY.scope()}
                    data-part="separator-rule"
                    data-slot="field-separator-rule"
                    class={FIELD_RECIPE.separator_rule}
                />
                <div
                    data-scope={FIELD_ANATOMY.scope()}
                    data-part="separator-content"
                    data-slot="field-separator-content"
                    class={FIELD_RECIPE.separator_content}
                >
                    <span
                        data-scope={FIELD_ANATOMY.scope()}
                        data-part="separator-text"
                        data-slot="field-separator-text"
                        class={FIELD_RECIPE.separator_text}
                    >
                        {self.children}
                    </span>
                </div>
            </div>
        }
        .render(cx)
    }
}
