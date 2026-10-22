// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::module_inception)]
//! Select component - form dropdown selection field
//!
//! The Select component provides a styled dropdown selection with support for
//! various sizes, placeholder text, and form integration.

use crate::components::_proa::Anatomy;

mod md;
mod recipe;
mod select;

/// Stable DOM parts shared by static, Markdown, and rsjs Select targets.
pub const SELECT_ANATOMY: Anatomy = Anatomy::new("select", &["root", "option"]);

// Re-export select types

pub use select::{Select, SelectOption};

// Re-export recipe types
pub use recipe::{
    select_classes, select_recipe, SelectRecipe, SelectRecipeProps, SelectSize, SelectStyle,
    SELECT_RECIPE,
};
