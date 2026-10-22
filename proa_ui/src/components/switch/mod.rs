// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::module_inception)]
//! Switch component - a sliding toggle control
//!
//! The Switch component provides a toggle that switches between checked and
//! unchecked states, styled as a sliding control similar to iOS-style switches.

use crate::components::_proa::Anatomy;

mod md;
mod recipe;
mod switch;

/// Stable DOM parts exposed by Switch.
pub const SWITCH_ANATOMY: Anatomy = Anatomy::new("switch", &["root", "thumb"]);

// Re-export switch types
pub use switch::Switch;

// Re-export recipe types
pub use recipe::{
    switch_recipe, switch_thumb_classes, switch_track_classes, SwitchRecipe, SwitchRecipeProps,
    SwitchSize, SwitchStyle, SWITCH_RECIPE,
};

// Re-export metadata export function (only when feature is enabled)
