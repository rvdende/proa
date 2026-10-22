// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Select component implementation
use super::recipe::{SelectRecipeProps, SelectSize, SELECT_RECIPE};
use super::SELECT_ANATOMY;
use crate::components::_proa::free_web::{binding_to_js_attr, JsonAttr};

use proa_core::{WebContext, WebRenderSync, WriteBuf};
use proa_ir::{FieldRegistration, ReactiveAttr};
use proa_macros::html_sync;
/// A single option for the Select component

#[derive(Debug, Clone, PartialEq)]
pub struct SelectOption {
    /// The value attribute (submitted with form)
    pub value: &'static str,
    /// The display label for this option
    pub label: &'static str,
    /// Whether this option is disabled
    pub disabled: bool,
}
impl SelectOption {
    /// Create a new option
    pub const fn new(value: &'static str, label: &'static str) -> Self {
        Self {
            value,
            label,
            disabled: false,
        }
    }
    /// Create a disabled option
    pub const fn disabled(value: &'static str, label: &'static str) -> Self {
        Self {
            value,
            label,
            disabled: true,
        }
    }
}

impl crate::components::_proa::collection::ListCollectionItem for SelectOption {
    type Value = str;

    fn value(&self) -> &Self::Value {
        self.value
    }

    fn text(&self) -> &str {
        self.label
    }

    fn disabled(&self) -> bool {
        self.disabled
    }
}

/// Type-safe Select component.
///
/// A form select/dropdown field with support for various sizes and states.
/// Follows the shadcn/ui select design with Tailwind CSS styling.
///
/// # Example
///
/// ```ignore
/// use crate::components::{Select, SelectOption, SelectSize};
/// use proa_macros::html_sync;
///
/// // Basic usage
/// html_sync! {
///     <div>
///         {Select {
///             options: vec![
///                 SelectOption::new("apple", "Apple"),
///                 SelectOption::new("banana", "Banana"),
///                 SelectOption::new("orange", "Orange"),
///             ],
///             placeholder: Some("Select a fruit"),
///             ..Select::new()
///         }}
///     </div>
/// }.render(cx);
///
/// // With form registration (React Hook Form style)
/// let form = SignUpForm::form();
/// html_sync! {
///     <div>
///         {Select {
///             registration: Some(form.register_country()),
///             options: vec![
///                 SelectOption::new("us", "United States"),
///                 SelectOption::new("uk", "United Kingdom"),
///                 SelectOption::new("ca", "Canada"),
///             ],
///             placeholder: Some("Select a country"),
///             aria_invalid: form.field_country().has_error(),
///             ..Select::new()
///         }}
///     </div>
/// }.render(cx);
/// ```
pub struct Select {
    pub size: Option<SelectSize>,
    /// The options to display in the dropdown
    pub options: Vec<SelectOption>,
    /// The name attribute for form submission
    pub name: Option<&'static str>,
    /// The currently selected value (if any)
    pub value: Option<&'static str>,
    /// Placeholder text shown when no value is selected
    pub placeholder: Option<&'static str>,
    /// Whether the select is disabled
    pub disabled: ReactiveAttr<bool>,
    /// Whether the select is required
    pub required: bool,
    /// Autofocus the select on page load
    pub autofocus: bool,
    pub aria_label: Option<&'static str>,
    pub aria_describedby: Option<&'static str>,
    pub aria_invalid: ReactiveAttr<bool>,
    /// Field registration data (from form.register_*)
    pub registration: Option<FieldRegistration>,
    pub class: Option<&'static str>,
    pub id: Option<&'static str>,
}
impl Select {
    /// Create a new select with default settings
    pub const fn new() -> Self {
        Self {
            size: None,
            options: Vec::new(),
            name: None,
            value: None,
            placeholder: None,
            disabled: ReactiveAttr::static_value(false),
            required: false,
            autofocus: false,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: ReactiveAttr::static_value(false),
            registration: None,
            class: None,
            id: None,
        }
    }
}
impl Default for Select {
    fn default() -> Self {
        Self::new()
    }
}
impl<B: WriteBuf, L: ::proa_core::DataLoader> WebRenderSync<L, B> for Select {
    #[inline(always)]
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = SELECT_RECIPE.resolve(SelectRecipeProps { size: self.size });
        let is_disabled = self.disabled.default;
        let is_aria_invalid = self.aria_invalid.default;
        let state = crate::components::_proa::FormControlState {
            disabled: is_disabled,
            invalid: is_aria_invalid,
            required: self.required,
            read_only: false,
        };
        let disabled_binding_string = binding_to_js_attr(&self.disabled.binding);
        let aria_invalid_binding_string = binding_to_js_attr(&self.aria_invalid.binding);
        let disabled_binding = disabled_binding_string.as_deref();
        let aria_invalid_binding = aria_invalid_binding_string.as_deref();
        let registration = self.registration.as_ref();
        let rules_attr = JsonAttr(registration.map(|reg| reg.rules.as_slice()));
        let data_field = registration.map(|reg| reg.data_field);
        let name_attr = registration.map(|reg| reg.name).or(self.name);
        let validate_on = registration.map(|reg| match reg.validate_on {
            proa_ir::ValidationTrigger::Blur => "blur",
            proa_ir::ValidationTrigger::Input => "input",
            proa_ir::ValidationTrigger::Change => "change",
            proa_ir::ValidationTrigger::Submit => "submit",
        });
        let placeholder_is_selected = self.placeholder.is_some() && self.value.is_none();
        let current_value = self.value;
        html_sync! {
            <select
                class={proa_macros::text!("{} {}", style, self.class)}
                id={self.id}
                name={name_attr}
                disabled={is_disabled}
                required={self.required}
                autofocus={self.autofocus}
                aria-label={self.aria_label}
                aria-describedby={self.aria_describedby}
                aria-invalid={state.aria_invalid()}
                aria-disabled={state.aria_disabled()}
                aria-required={state.aria_required()}
                data-select
                data-scope={SELECT_ANATOMY.scope()}
                data-part="root"
                data-slot="select"
                data-size={style.size().as_str()}
                data-disabled={state.data_disabled()}
                data-invalid={state.data_invalid()}
                data-required={state.data_required()}
                data-field={data_field}
                data-rules={rules_attr}
                data-validate-on={validate_on}
                data-bind-disabled={disabled_binding}
                data-bind-aria-invalid={aria_invalid_binding}
            >
                {if let Some(placeholder) = self.placeholder { html_sync! {
                    <option data-scope={SELECT_ANATOMY.scope()} data-part="option" data-placeholder="true" value="" disabled selected={placeholder_is_selected}>
                        {placeholder}
                    </option>
                }.render(cx) ? ; }}
                {for opt in self.options { let is_selected = current_value == Some(opt.value) ; html_sync! {
                    <option data-scope={SELECT_ANATOMY.scope()} data-part="option" value={opt.value} disabled={opt.disabled} selected={is_selected}>
                        {opt.label}
                    </option>
                }.render(cx) ? ; }}
            </select>
        }
        .render(cx)
    }
}
