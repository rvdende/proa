// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Breadcrumb recipe — static named slots for navigation hierarchy parts.

use crate::tw_join;
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

mod base {
    use super::*;

    pub const ROOT: &str = "";
    pub const LIST: &str = tw_join!(
        "flex",
        "flex-wrap",
        "items-center",
        "gap-1.5",
        "break-words",
        "text-sm",
        "text-muted-foreground",
        "sm:gap-2.5"
    );
    pub const ITEM: &str = tw_join!("inline-flex", "items-center", "gap-1.5");
    pub const LINK: &str = tw_join!("transition-colors", "hover:text-foreground");
    pub const PAGE: &str = tw_join!("font-normal", "text-foreground");
    pub const SEPARATOR: &str = tw_join!("[&>svg]:h-3.5", "[&>svg]:w-3.5");
    pub const ELLIPSIS: &str = tw_join!("flex", "h-9", "w-9", "items-center", "justify-center");
    pub const ELLIPSIS_LABEL: &str = "sr-only";
}

/// Optional inputs accepted by [`BreadcrumbRecipe::resolve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BreadcrumbRecipeProps;

/// Static named classes for every Breadcrumb part.
#[derive(Debug, Clone, Copy)]
pub struct BreadcrumbRecipe {
    pub root: &'static str,
    pub list: &'static str,
    pub item: &'static str,
    pub link: &'static str,
    pub page: &'static str,
    pub separator: &'static str,
    pub ellipsis: &'static str,
    pub ellipsis_label: &'static str,
}

impl BreadcrumbRecipe {
    pub const fn resolve(&'static self, _props: BreadcrumbRecipeProps) -> BreadcrumbStyle {
        BreadcrumbStyle { recipe: self }
    }
}

/// Fully resolved, streamable Breadcrumb root style.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct BreadcrumbStyle {
    recipe: &'static BreadcrumbRecipe,
}

impl AttrValue for BreadcrumbStyle {
    #[inline(always)]
    fn should_render(&self) -> bool {
        !self.recipe.root.is_empty()
    }

    #[inline(always)]
    fn render_attr_value<B: WriteBuf, L: DataLoader>(
        &self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), WriteError> {
        cx.out.extend_static(self.recipe.root.as_bytes())
    }
}

pub const BREADCRUMB_RECIPE: BreadcrumbRecipe = BreadcrumbRecipe {
    root: base::ROOT,
    list: base::LIST,
    item: base::ITEM,
    link: base::LINK,
    page: base::PAGE,
    separator: base::SEPARATOR,
    ellipsis: base::ELLIPSIS,
    ellipsis_label: base::ELLIPSIS_LABEL,
};

pub const fn breadcrumb_recipe() -> &'static BreadcrumbRecipe {
    &BREADCRUMB_RECIPE
}

impl crate::components::_proa::variant_spec::ComponentSpec for BreadcrumbRecipe {
    const NAME: &'static str = "Breadcrumb";

    fn base_classes(&self) -> &'static str {
        self.root
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        Vec::new()
    }

    fn anatomy(&self) -> Option<crate::components::_proa::Anatomy> {
        Some(super::BREADCRUMB_ANATOMY)
    }
}

/// Compatibility accessors for the former standalone helpers.
pub const fn breadcrumb_classes() -> &'static str {
    BREADCRUMB_RECIPE.root
}

pub const fn breadcrumb_list_classes() -> &'static str {
    BREADCRUMB_RECIPE.list
}

pub const fn breadcrumb_item_classes() -> &'static str {
    BREADCRUMB_RECIPE.item
}

pub const fn breadcrumb_link_classes() -> &'static str {
    BREADCRUMB_RECIPE.link
}

pub const fn breadcrumb_page_classes() -> &'static str {
    BREADCRUMB_RECIPE.page
}

pub const fn breadcrumb_separator_classes() -> &'static str {
    BREADCRUMB_RECIPE.separator
}

pub const fn breadcrumb_ellipsis_classes() -> &'static str {
    BREADCRUMB_RECIPE.ellipsis
}
