// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::module_inception)]
//! Alert component - displays a callout for user attention
//!
//! Supports Default and Destructive variants.

mod alert;
mod md;
mod recipe;

pub use alert::{Alert, AlertDescription, AlertTitle};
pub use recipe::{
    alert_classes, alert_recipe, AlertRecipe, AlertRecipeProps, AlertStyle, AlertVariant,
    ALERT_RECIPE,
};
