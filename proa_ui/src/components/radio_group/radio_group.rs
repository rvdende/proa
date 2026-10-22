// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! RadioGroup component implementation
//!
//! A set of radio buttons where only one can be selected at a time.
//! Supports form integration with proa_ir for reactive state management.
use super::recipe::{
    RadioGroupOrientation, RadioGroupRecipeProps, RadioGroupSize, RADIO_GROUP_RECIPE,
};
use super::RADIO_GROUP_ANATOMY;
use crate::components::_proa::free_web::{binding_to_js_attr, JsonAttr};

use proa_core::{WebContext, WebRenderSync, WriteBuf};
use proa_ir::{FieldRegistration, ReactiveAttr};
use proa_macros::html_sync;
/// RadioGroup container component
///
/// Wraps a set of [`RadioGroupItem`] components and owns group-level state.
///
/// The root renders as a disabled-capable `fieldset`, so `disabled` is
/// inherited by every native radio input without each item repeating it.
/// Native radio `name` attributes still have to be present on each item; use
/// [`super::ControlledRadioGroup`] when the root must construct and own the
/// complete option collection.
///
/// # Example
///
/// ```ignore
/// use crate::components::{RadioGroup, RadioGroupItem};
/// use proa_macros::html_sync;
///
/// html_sync! {
///     {RadioGroup::new(
///         "plan",
///         html_sync! {
///             <div class="flex items-center space-x-2">
///                 {RadioGroupItem { id: Some("r1"), ..RadioGroupItem::new("free", "plan") }}
///                 <label for="r1">"Free"</label>
///             </div>
///             <div class="flex items-center space-x-2">
///                 {RadioGroupItem { id: Some("r2"), ..RadioGroupItem::new("pro", "plan") }}
///                 <label for="r2">"Pro"</label>
///             </div>
///         }
///     )}
/// }.render(cx);
/// ```
pub struct RadioGroup<C> {
    /// The name attribute for all radio inputs in the group
    pub name: &'static str,
    /// The currently selected value (for SSR)
    pub value: Option<&'static str>,
    /// Whether the entire group is disabled
    pub disabled: ReactiveAttr<bool>,
    /// Whether the group is required
    pub required: bool,
    /// Form registration data (from form.register_*)
    pub registration: Option<FieldRegistration>,
    /// Orientation: vertical (default) or horizontal
    pub orientation: RadioGroupOrientation,
    /// Additional CSS classes
    pub class: Option<&'static str>,
    /// ARIA label for the group
    pub aria_label: Option<&'static str>,
    /// Children (RadioGroupItem components)
    pub children: C,
}
impl<C> RadioGroup<C> {
    /// Create a new radio group with the given name
    pub const fn new(name: &'static str, children: C) -> Self {
        Self {
            name,
            value: None,
            disabled: ReactiveAttr::static_value(false),
            required: false,
            registration: None,
            orientation: RadioGroupOrientation::Vertical,
            class: None,
            aria_label: None,
            children,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for RadioGroup<C>
{
    #[inline(always)]
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = RADIO_GROUP_RECIPE.resolve(RadioGroupRecipeProps {
            orientation: Some(self.orientation),
            size: None,
        });
        let data_field = if let Some(reg) = self.registration.as_ref() {
            Some(reg.data_field)
        } else {
            None
        };
        let name_attr = if let Some(reg) = self.registration.as_ref() {
            reg.name
        } else {
            self.name
        };
        let rules_attr = JsonAttr(if let Some(reg) = self.registration.as_ref() {
            Some(reg.rules.as_slice())
        } else {
            None
        });
        let validate_on = registration_validate_on(self.registration.as_ref());
        let is_disabled = self.disabled.default;
        let disabled_binding_string = binding_to_js_attr(&self.disabled.binding);
        let disabled_binding = disabled_binding_string.as_deref();
        html_sync! {
            <fieldset
                data-radio-group
                role="radiogroup"
                class={proa_macros::text!("{} {}", style, self.class)}
                data-scope={RADIO_GROUP_ANATOMY.scope()}
                data-part="root"
                data-slot="radio-group"
                data-name={name_attr}
                data-value={self.value}
                data-field={data_field}
                data-rules={rules_attr}
                data-validate-on={validate_on}
                data-orientation={style.orientation().as_str()}
                data-size={style.size().as_str()}
                data-bind-disabled={disabled_binding}
                disabled={is_disabled}
                aria-label={self.aria_label}
                aria-orientation={style.orientation().as_str()}
                aria-required={self.required}
            >
                {self.children}
            </fieldset>
        }
        .render(cx)
    }
}
/// RadioGroupItem component - individual radio button
///
/// Must be used within a [`RadioGroup`]. Because this is a native radio input,
/// its `name` remains explicit and should match the group name.
///
/// # Example
///
/// ```ignore
/// use crate::components::RadioGroupItem;
///
/// RadioGroupItem {
///     id: Some("option-1"),
///     checked: true,
///     ..RadioGroupItem::new("option1", "plan")
/// }
/// ```
pub struct RadioGroupItem {
    /// The value of this radio item
    pub value: &'static str,
    /// The name attribute (should match the group name)
    pub name: &'static str,
    /// Whether this item is checked (for SSR)
    pub checked: bool,
    /// Whether this item is disabled
    pub disabled: ReactiveAttr<bool>,
    /// Size variant
    pub size: Option<RadioGroupSize>,
    /// Additional CSS classes
    pub class: Option<&'static str>,
    /// ID for the input element
    pub id: Option<&'static str>,
    /// ARIA label
    pub aria_label: Option<&'static str>,
    /// ARIA describedby
    pub aria_describedby: Option<&'static str>,
    /// ARIA invalid state
    pub aria_invalid: ReactiveAttr<bool>,
}
impl RadioGroupItem {
    /// Create a new radio item with the given value
    pub const fn new(value: &'static str, name: &'static str) -> Self {
        Self {
            value,
            name,
            checked: false,
            disabled: ReactiveAttr::static_value(false),
            size: None,
            class: None,
            id: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: ReactiveAttr::static_value(false),
        }
    }
    /// Create a checked radio item
    pub const fn checked(value: &'static str, name: &'static str) -> Self {
        Self {
            value,
            name,
            checked: true,
            disabled: ReactiveAttr::static_value(false),
            size: None,
            class: None,
            id: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: ReactiveAttr::static_value(false),
        }
    }
}
impl<B: WriteBuf, L: ::proa_core::DataLoader> WebRenderSync<L, B> for RadioGroupItem {
    #[inline(always)]
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = RADIO_GROUP_RECIPE.resolve_item(self.size);
        let is_disabled = self.disabled.default;
        let is_aria_invalid = self.aria_invalid.default;
        let disabled_binding_string = binding_to_js_attr(&self.disabled.binding);
        let aria_invalid_binding_string = binding_to_js_attr(&self.aria_invalid.binding);
        let disabled_binding = disabled_binding_string.as_deref();
        let aria_invalid_binding = aria_invalid_binding_string.as_deref();
        html_sync! {
            <input
                type="radio"
                class={proa_macros::text!("{} {}", style, self.class)}
                id={self.id}
                name={self.name}
                value={self.value}
                checked={self.checked}
                disabled={is_disabled}
                aria-label={self.aria_label}
                aria-describedby={self.aria_describedby}
                aria-invalid={is_aria_invalid}
                data-scope={RADIO_GROUP_ANATOMY.scope()}
                data-part="item"
                data-slot="radio-group-item"
                data-size={style.size().as_str()}
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
