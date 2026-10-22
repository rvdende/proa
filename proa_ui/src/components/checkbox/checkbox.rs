// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Checkbox component implementation
#![allow(unused_must_use)]

use super::recipe::{
    CheckboxColor, CheckboxIcon, CheckboxRecipeProps, CheckboxSize, CHECKBOX_RECIPE,
};
use super::CHECKBOX_ANATOMY;
use crate::components::_proa::free_web::{binding_to_js_attr, JsonAttr};

use proa_core::{WebContext, WebRenderSync, WriteBuf};
use proa_ir::{FieldRegistration, ReactiveAttr};
use proa_macros::html_sync;
/// Checkbox-specific name for the shared three-state value.
pub type CheckboxState = crate::components::_proa::CheckedState;
/// Type-safe Checkbox component.
///
/// A form checkbox with support for checked, unchecked, and indeterminate states.
/// Follows the shadcn/ui checkbox design with Tailwind CSS styling.
///
/// # Example
///
/// ```ignore
/// use crate::components::{Checkbox, CheckboxSize, CheckboxState};
/// use proa_macros::html_sync;
///
/// // Basic usage
/// html_sync! {
///     <div class="flex items-center gap-2">
///         {Checkbox {
///             id: Some("terms"),
///             ..Checkbox::new()
///         }}
///         <label for="terms">"Accept terms and conditions"</label>
///     </div>
/// }.render(cx);
///
/// // Checked state
/// html_sync! {
///     <div>
///         {Checkbox {
///             state: CheckboxState::Checked,
///             ..Checkbox::new()
///         }}
///     </div>
/// }.render(cx);
/// ```
pub struct Checkbox {
    pub size: Option<CheckboxSize>,
    /// The color variant (default: Primary)
    pub color: Option<CheckboxColor>,
    /// The icon to display when checked (default: Check)
    pub icon: Option<CheckboxIcon>,
    /// The current state of the checkbox
    pub state: CheckboxState,
    /// The name attribute for form submission
    pub name: Option<&'static str>,
    /// The value attribute for form submission
    pub value: Option<&'static str>,
    /// Whether the checkbox is disabled
    pub disabled: ReactiveAttr<bool>,
    /// Whether the checkbox is required
    pub required: bool,
    pub aria_label: Option<&'static str>,
    pub aria_describedby: Option<&'static str>,
    pub aria_invalid: ReactiveAttr<bool>,
    /// Field registration data (from form.register_*)
    pub registration: Option<FieldRegistration>,
    pub class: Option<&'static str>,
    pub id: Option<&'static str>,
}
impl Checkbox {
    /// Create a new checkbox with default settings
    pub const fn new() -> Self {
        Self {
            size: None,
            color: None,
            icon: None,
            state: CheckboxState::Unchecked,
            name: None,
            value: None,
            disabled: ReactiveAttr::static_value(false),
            required: false,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: ReactiveAttr::static_value(false),
            registration: None,
            class: None,
            id: None,
        }
    }
    /// Create a checked checkbox
    pub const fn checked() -> Self {
        Self {
            size: None,
            color: None,
            icon: None,
            state: CheckboxState::Checked,
            name: None,
            value: None,
            disabled: ReactiveAttr::static_value(false),
            required: false,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: ReactiveAttr::static_value(false),
            registration: None,
            class: None,
            id: None,
        }
    }
}
impl Default for Checkbox {
    fn default() -> Self {
        Self::new()
    }
}
impl<B: WriteBuf, L: ::proa_core::DataLoader> WebRenderSync<L, B> for Checkbox {
    #[inline(always)]
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = CHECKBOX_RECIPE.resolve(CheckboxRecipeProps {
            size: self.size,
            color: self.color,
        });
        let icon = self.icon.unwrap_or_default();
        let icon_style = icon.style();
        let is_disabled = self.disabled.default;
        let is_aria_invalid = self.aria_invalid.default;
        let form_state = crate::components::_proa::FormControlState {
            disabled: is_disabled,
            invalid: is_aria_invalid,
            required: self.required,
            read_only: false,
        };
        let is_checked = self.state == CheckboxState::Checked;
        let is_indeterminate = self.state == CheckboxState::Indeterminate;
        let disabled_binding_string = binding_to_js_attr(&self.disabled.binding);
        let aria_invalid_binding_string = binding_to_js_attr(&self.aria_invalid.binding);
        let disabled_binding = disabled_binding_string.as_deref();
        let aria_invalid_binding = aria_invalid_binding_string.as_deref();
        let registration = self.registration.as_ref();
        let rules_attr = JsonAttr(if let Some(reg) = registration {
            Some(reg.rules.as_slice())
        } else {
            None
        });
        let data_field = if let Some(reg) = registration {
            Some(reg.data_field)
        } else {
            None
        };
        let name_attr = if let Some(reg) = registration {
            Some(reg.name)
        } else {
            self.name
        };
        let validate_on = registration_validate_on(registration);
        html_sync! {
            <input
                type="checkbox"
                class={proa_macros::text!("{} {}", style, self.class)}
                style={proa_core::style_ok(icon_style)}
                id={self.id}
                name={name_attr}
                value={self.value}
                checked={is_checked}
                disabled={is_disabled}
                required={self.required}
                aria-label={self.aria_label}
                aria-describedby={self.aria_describedby}
                aria-checked={self.state.aria_checked()}
                aria-disabled={form_state.aria_disabled()}
                aria-invalid={form_state.aria_invalid()}
                aria-required={form_state.aria_required()}
                data-checkbox
                data-scope={CHECKBOX_ANATOMY.scope()}
                data-part="root"
                data-slot="checkbox"
                data-state={self.state}
                data-disabled={form_state.data_disabled()}
                data-invalid={form_state.data_invalid()}
                data-required={form_state.data_required()}
                data-size={style.size().as_str()}
                data-color={style.color().as_str()}
                data-icon={icon.as_str()}
                data-indeterminate={is_indeterminate}
                data-field={data_field}
                data-rules={rules_attr}
                data-validate-on={validate_on}
                data-bind-disabled={disabled_binding}
                data-bind-aria-invalid={aria_invalid_binding}
            />
        }
        .render(cx)
    }
}

fn registration_validate_on(registration: Option<&FieldRegistration>) -> Option<&'static str> {
    registration.map(|reg| match reg.validate_on {
        proa_ir::ValidationTrigger::Blur => "blur",
        proa_ir::ValidationTrigger::Input => "input",
        proa_ir::ValidationTrigger::Change => "change",
        proa_ir::ValidationTrigger::Submit => "submit",
    })
}
