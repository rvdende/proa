// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::module_inception)]
//! RadioGroup component - a set of radio buttons where only one can be selected
//!
//! The RadioGroup component provides a styled set of radio buttons with
//! support for form integration via proa_ir.

use crate::components::_proa::Anatomy;

mod md;
mod radio_group;
mod recipe;

/// Stable DOM parts shared by static and controlled radio groups.
pub const RADIO_GROUP_ANATOMY: Anatomy = Anatomy::new("radio-group", &["root", "option", "item"]);

// Re-export radio group types

pub use radio_group::{RadioGroup, RadioGroupItem};

// Re-export recipe types
pub use recipe::{
    radio_group_classes, radio_group_recipe, radio_item_classes, radio_item_classes_with_size,
    RadioGroupItemStyle, RadioGroupOrientation, RadioGroupRecipe, RadioGroupRecipeProps,
    RadioGroupSize, RadioGroupStyle, RADIO_GROUP_RECIPE,
};
