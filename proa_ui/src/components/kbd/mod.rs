// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Kbd component module
//!
//! Displays keyboard input or keyboard shortcuts.
//!
//! Based on shadcn/ui styling.

#[allow(clippy::module_inception)]
mod kbd;
mod md;
mod recipe;

pub use kbd::{Kbd, KbdGroup};
pub use recipe::{
    kbd_base_classes, kbd_group_classes, kbd_recipe, KbdRecipe, KbdRecipeProps, KbdStyle,
    KBD_RECIPE,
};
