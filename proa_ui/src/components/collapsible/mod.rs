// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::module_inception)]
//! Collapsible component - an interactive disclosure widget
//!
//! The collapsible component allows toggling visibility of content.
//! JavaScript handles the show/hide behavior.

use crate::components::_proa::Anatomy;

mod collapsible;
mod md;
mod recipe;

/// Static anatomy contract shared by SSR and the delegated runtime.
pub const COLLAPSIBLE_ANATOMY: Anatomy =
    Anatomy::new("collapsible", &["root", "trigger", "content"]);

pub use collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
pub use recipe::{
    collapsible_classes, collapsible_content_classes, collapsible_recipe,
    collapsible_trigger_classes, CollapsibleRecipe, CollapsibleRecipeProps, CollapsibleStyle,
    COLLAPSIBLE_RECIPE,
};
