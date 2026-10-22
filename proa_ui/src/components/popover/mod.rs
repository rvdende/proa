// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Popover component - displays content in a floating panel
//!
//! Similar to Dialog but positioned relative to a trigger element.

use crate::components::_proa::Anatomy;

mod md;
#[allow(clippy::module_inception)]
mod popover;
mod recipe;

/// Static anatomy contract shared by SSR and the delegated runtime.
pub const POPOVER_ANATOMY: Anatomy =
    Anatomy::new("popover", &["root", "trigger", "content", "close"]);

pub use popover::{Popover, PopoverClose, PopoverContent, PopoverTrigger};
pub use recipe::{
    popover_recipe, PopoverAlign, PopoverRecipe, PopoverRecipeProps, PopoverSide, PopoverStyle,
    POPOVER_RECIPE,
};
