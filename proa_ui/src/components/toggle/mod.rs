// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Toggle component module
//!
//! A two-state button that can be either on or off.
//!
//! Based on Radix UI Toggle with shadcn/ui styling.

mod md;
#[allow(clippy::module_inception)]
mod recipe;
mod toggle;

pub use recipe::{
    toggle_recipe, ToggleRecipe, ToggleRecipeProps, ToggleSize, ToggleStyle, ToggleVariant,
    TOGGLE_RECIPE,
};
pub use toggle::Toggle;
