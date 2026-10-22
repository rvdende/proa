// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Collapsible recipe — static named slots for disclosure parts.

use crate::tw_join;
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

const COLLAPSIBLE_ROOT: &str = "";
const COLLAPSIBLE_TRIGGER: &str = "";
const COLLAPSIBLE_CONTENT: &str = tw_join!("overflow-hidden", "data-[state=closed]:hidden");

/// Optional inputs accepted by [`CollapsibleRecipe::resolve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CollapsibleRecipeProps;

/// Static named classes for every Collapsible part.
#[derive(Debug, Clone, Copy)]
pub struct CollapsibleRecipe {
    pub root: &'static str,
    pub trigger: &'static str,
    pub content: &'static str,
}

impl CollapsibleRecipe {
    pub const fn resolve(&'static self, _props: CollapsibleRecipeProps) -> CollapsibleStyle {
        CollapsibleStyle { recipe: self }
    }
}

/// Fully resolved, streamable Collapsible root style.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct CollapsibleStyle {
    recipe: &'static CollapsibleRecipe,
}

impl AttrValue for CollapsibleStyle {
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

pub const COLLAPSIBLE_RECIPE: CollapsibleRecipe = CollapsibleRecipe {
    root: COLLAPSIBLE_ROOT,
    trigger: COLLAPSIBLE_TRIGGER,
    content: COLLAPSIBLE_CONTENT,
};

pub const fn collapsible_recipe() -> &'static CollapsibleRecipe {
    &COLLAPSIBLE_RECIPE
}

impl crate::components::_proa::variant_spec::ComponentSpec for CollapsibleRecipe {
    const NAME: &'static str = "Collapsible";

    fn base_classes(&self) -> &'static str {
        self.root
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        Vec::new()
    }

    fn anatomy(&self) -> Option<crate::components::_proa::Anatomy> {
        Some(super::COLLAPSIBLE_ANATOMY)
    }
}

/// Compatibility accessors for the former standalone helpers.
pub const fn collapsible_classes() -> &'static str {
    COLLAPSIBLE_RECIPE.root
}

pub const fn collapsible_trigger_classes() -> &'static str {
    COLLAPSIBLE_RECIPE.trigger
}

pub const fn collapsible_content_classes() -> &'static str {
    COLLAPSIBLE_RECIPE.content
}
