// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::module_inception)]
pub mod button_group;
mod md;
pub mod recipe;

pub use button_group::{ButtonGroup, ButtonGroupSeparator, ButtonGroupText};
pub use recipe::{
    button_group_recipe, ButtonGroupOrientation, ButtonGroupRecipe, ButtonGroupRecipeProps,
    ResolvedButtonGroupRecipe, BUTTON_GROUP_RECIPE,
};

// Re-export metadata export function (only when feature is enabled)
