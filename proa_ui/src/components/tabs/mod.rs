// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Tabs component module
//!
//! A set of layered sections of content that are displayed one at a time.
//!
//! Based on Radix UI Tabs with shadcn/ui styling.

#![allow(clippy::module_inception)]
use crate::components::_proa::Anatomy;

mod md;
mod recipe;
mod tabs;

/// Stable DOM parts shared by delegated-runtime and controlled tabs.
pub const TABS_ANATOMY: Anatomy = Anatomy::new("tabs", &["root", "list", "trigger", "content"]);

pub use recipe::{
    tabs_content_classes, tabs_list_classes, tabs_recipe, tabs_trigger_classes, TabsOrientation,
    TabsRecipe, TabsRecipeProps, TabsStyle, TABS_RECIPE,
};
pub use tabs::{Tabs, TabsActivationMode, TabsContent, TabsList, TabsTrigger};
