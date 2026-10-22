// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::module_inception)]
//! Checkbox component - form checkbox with tri-state support
//!
//! The Checkbox component provides a styled checkbox with support for
//! checked, unchecked, and indeterminate states.

use crate::components::_proa::Anatomy;

mod checkbox;

mod md;
mod recipe;

/// Stable DOM root exposed by the native Checkbox control.
pub const CHECKBOX_ANATOMY: Anatomy = Anatomy::new("checkbox", &["root"]);

// Re-export checkbox types
pub use checkbox::{Checkbox, CheckboxState};

// Re-export recipe types
pub use recipe::{
    checkbox_classes, checkbox_recipe, CheckboxColor, CheckboxIcon, CheckboxRecipe,
    CheckboxRecipeProps, CheckboxSize, CheckboxStyle, CHECKBOX_RECIPE,
};
