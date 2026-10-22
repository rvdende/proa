// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Sheet component - a side-sliding dialog panel
//!
//! Extends the Dialog behavior with slide-from-edge positioning.
//! Reuses the Dialog JS handler via `data-dialog-*` attributes.

use crate::components::_proa::Anatomy;

mod md;
pub mod recipe;
mod sheet;

/// Static anatomy contract for Sheet's public compound parts.
pub const SHEET_ANATOMY: Anatomy = Anatomy::new(
    "sheet",
    &[
        "root",
        "trigger",
        "portal",
        "overlay",
        "content",
        "close",
        "close-label",
        "header",
        "title",
        "description",
        "footer",
    ],
);

pub use recipe::{
    sheet_content_classes, sheet_recipe, SheetRecipe, SheetRecipeProps, SheetSide, SheetStyle,
    SHEET_RECIPE,
};
pub use sheet::{
    Sheet, SheetClose, SheetContent, SheetDescription, SheetFooter, SheetHeader, SheetTitle,
    SheetTrigger,
};
