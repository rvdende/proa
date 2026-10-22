// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::module_inception)]
//! Breadcrumb component - navigation hierarchy display

mod breadcrumb;
mod md;
mod recipe;

/// Stable scope/part contract shared by every Breadcrumb renderer.
pub const BREADCRUMB_ANATOMY: crate::components::_proa::Anatomy = crate::components::_proa::Anatomy::new(
    "breadcrumb",
    &[
        "root",
        "list",
        "item",
        "link",
        "page",
        "separator",
        "ellipsis",
        "ellipsis-label",
    ],
);

pub use breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator,
};

pub use recipe::{
    breadcrumb_classes, breadcrumb_ellipsis_classes, breadcrumb_item_classes,
    breadcrumb_link_classes, breadcrumb_list_classes, breadcrumb_page_classes, breadcrumb_recipe,
    breadcrumb_separator_classes, BreadcrumbRecipe, BreadcrumbRecipeProps, BreadcrumbStyle,
    BREADCRUMB_RECIPE,
};
