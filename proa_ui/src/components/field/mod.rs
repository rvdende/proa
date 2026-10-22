// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Field component module
//!
//! Composable form field components for building accessible forms.
//!
//! Based on shadcn/ui styling.

use crate::components::_proa::Anatomy;

#[allow(clippy::module_inception)]
mod field;
mod md;
mod recipe;

/// Stable DOM parts shared by the Field and Fieldset family.
pub const FIELD_ANATOMY: Anatomy = Anatomy::new(
    "field",
    &[
        "root",
        "fieldset",
        "legend",
        "group",
        "label",
        "content",
        "title",
        "description",
        "error",
        "separator",
        "separator-rule",
        "separator-content",
        "separator-text",
    ],
);

pub use field::{
    Field, FieldContent, FieldDescription, FieldError, FieldGroup, FieldLabel, FieldLegend,
    FieldSeparator, FieldSet, FieldTitle,
};
pub use recipe::{
    field_recipe, FieldLegendVariant, FieldOrientation, FieldRecipe, FieldRecipeProps, FieldStyle,
    FIELD_RECIPE,
};
