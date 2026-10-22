// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Accordion component module
//!
//! A vertically stacked set of interactive headings that reveal content sections.
//!
//! Two variants are available:
//! - `Accordion` - Full-featured with JS hydration (single/multiple mode, keyboard nav)
//! - `AccordionCSSOnly` - Zero-JS using native `<details>`/`<summary>` elements

use crate::components::_proa::Anatomy;

#[allow(clippy::module_inception)]
mod accordion;
mod css_only;
mod md;
mod recipe;

/// Stable DOM parts shared by the delegated-runtime and CSS-only accordions.
pub const ACCORDION_ANATOMY: Anatomy = Anatomy::new(
    "accordion",
    &[
        "root",
        "item",
        "heading",
        "trigger",
        "input",
        "content",
        "content-inner",
        "content-body",
    ],
);

pub use accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
pub use css_only::{AccordionCSSOnly, AccordionCSSOnlyItem, AccordionCSSOnlyMode};
pub use recipe::{
    accordion_content_classes, accordion_content_inner_classes, accordion_item_classes,
    accordion_recipe, accordion_trigger_classes, AccordionRecipe, AccordionRecipeProps,
    AccordionStyle, AccordionType, ACCORDION_RECIPE,
};
