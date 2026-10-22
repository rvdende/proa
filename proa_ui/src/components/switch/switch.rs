// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Switch component implementation
//!
//! A toggle control that switches between checked and unchecked states.
//! Styled as a sliding toggle, similar to iOS-style switches.
use super::recipe::{SwitchRecipeProps, SwitchSize, SWITCH_RECIPE};
use super::SWITCH_ANATOMY;
use proa_core::{WebContext, WebRenderSync, WriteBuf};
use proa_macros::html_sync;
/// Switch component - a sliding toggle control
///
/// # Example
///
/// ```ignore
/// use crate::components::{Switch, SwitchSize};
/// use proa_macros::html_sync;
///
/// html_sync! {
///     <div class="flex items-center gap-2">
///         {Switch {
///             id: Some("airplane-mode"),
///             ..Switch::new()
///         }}
///         <label for="airplane-mode">"Airplane Mode"</label>
///     </div>
/// }
/// ```
pub struct Switch {
    /// Size variant
    pub size: SwitchSize,
    /// Whether the switch is checked (on)
    pub checked: bool,
    /// Whether the switch is disabled
    pub disabled: bool,
    /// The name attribute for form submission
    pub name: Option<&'static str>,
    /// The id attribute
    pub id: Option<&'static str>,
    /// Additional CSS classes
    pub class: Option<&'static str>,
    /// ARIA label for accessibility
    pub aria_label: Option<&'static str>,
}
impl Switch {
    /// Create a new switch with default settings
    pub const fn new() -> Self {
        Self {
            size: SwitchSize::Default,
            checked: false,
            disabled: false,
            name: None,
            id: None,
            class: None,
            aria_label: None,
        }
    }
    /// Create a checked switch
    pub const fn checked() -> Self {
        Self {
            size: SwitchSize::Default,
            checked: true,
            disabled: false,
            name: None,
            id: None,
            class: None,
            aria_label: None,
        }
    }
}
impl Default for Switch {
    fn default() -> Self {
        Self::new()
    }
}
impl<B: WriteBuf, L: ::proa_core::DataLoader> WebRenderSync<L, B> for Switch {
    #[inline(always)]
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = SWITCH_RECIPE.resolve(SwitchRecipeProps {
            size: Some(self.size),
        });
        let checked_state = crate::components::_proa::CheckedState::from_checked(self.checked);
        let form_state = crate::components::_proa::FormControlState {
            disabled: self.disabled,
            ..crate::components::_proa::FormControlState::enabled()
        };
        html_sync! {
            <button
                type="button"
                role="switch"
                aria-checked={checked_state.aria_checked()}
                aria-disabled={form_state.aria_disabled()}
                aria-label={self.aria_label}
                data-state={checked_state}
                data-disabled={form_state.data_disabled()}
                data-size={style.size().as_str()}
                data-switch
                data-scope={SWITCH_ANATOMY.scope()}
                data-part="root"
                data-slot="switch"
                id={self.id}
                name={self.name}
                disabled={self.disabled}
                class={proa_macros::text!("{} {}", style, self.class)}
            >
                <span
                    data-switch-thumb
                    data-scope={SWITCH_ANATOMY.scope()}
                    data-part="thumb"
                    data-slot="switch-thumb"
                    data-state={checked_state}
                    class={SWITCH_RECIPE.thumb}
                ></span>
            </button>
        }
        .render(cx)
    }
}
