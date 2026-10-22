// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Skeleton recipe - styling for loading placeholder

use crate::tw_join;
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

pub const SKELETON_BASE: &str = tw_join!("animate-pulse", "rounded-md", "bg-muted");

/// Optional author inputs for the Skeleton recipe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SkeletonRecipeProps;

/// Static named root for Skeleton.
#[derive(Debug, Clone, Copy)]
pub struct SkeletonRecipe {
    pub root: &'static str,
}

impl SkeletonRecipe {
    pub const fn resolve(&'static self, _props: SkeletonRecipeProps) -> SkeletonStyle {
        SkeletonStyle { recipe: self }
    }
}

/// Resolved, allocation-free Skeleton root style.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct SkeletonStyle {
    recipe: &'static SkeletonRecipe,
}

impl AttrValue for SkeletonStyle {
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

pub const SKELETON_RECIPE: SkeletonRecipe = SkeletonRecipe {
    root: SKELETON_BASE,
};

pub const fn skeleton_recipe() -> &'static SkeletonRecipe {
    &SKELETON_RECIPE
}

/// Compatibility accessor for the former standalone recipe function.
pub const fn skeleton_classes() -> &'static str {
    SKELETON_RECIPE.root
}

impl crate::components::_proa::variant_spec::ComponentSpec for SkeletonRecipe {
    const NAME: &'static str = "Skeleton";

    fn base_classes(&self) -> &'static str {
        self.root
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        Vec::new()
    }
}
