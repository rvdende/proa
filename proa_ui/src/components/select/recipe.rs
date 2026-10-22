// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Select recipe - type-safe styling system for Select component
//!
//! Provides size variants and base styling for the Select component.

use crate::tw_join;
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

/// Base styles for the select
mod base {
    use super::*;

    pub const SELECT_BASE: &str = tw_join!(
        // Layout
        "flex w-full items-center justify-between",
        // Typography
        "text-base md:text-sm",
        // Colors
        "bg-transparent",
        "text-foreground",
        // Border (shadcn select style)
        "border border-input",
        "rounded-md",
        // Shadow
        "shadow-xs",
        // Focus visible (shadcn style)
        "focus-visible:outline-hidden",
        "focus-visible:border-ring",
        "focus-visible:ring-ring/50",
        "focus-visible:ring-[3px]",
        // Disabled
        "disabled:cursor-not-allowed",
        "disabled:opacity-50",
        // Cursor
        "cursor-pointer",
        // Selection
        "selection:bg-primary",
        "selection:text-primary-foreground",
        // Aria invalid (error state)
        "aria-invalid:border-destructive",
        "aria-invalid:ring-destructive/20",
        "dark:aria-invalid:ring-destructive/40",
        // Appearance - custom arrow
        "appearance-none",
        // Background image for dropdown arrow
        "bg-no-repeat",
        "bg-[length:16px_16px]",
        "bg-[position:right_0.5rem_center]",
        "[background-image:url('data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%2224%22%20height%3D%2224%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22none%22%20stroke%3D%22%236b7280%22%20stroke-width%3D%222%22%20stroke-linecap%3D%22round%22%20stroke-linejoin%3D%22round%22%3E%3Cpath%20d%3D%22m6%209%206%206%206-6%22%2F%3E%3C%2Fsvg%3E')]",
        // Padding to account for arrow
        "pr-8"
    );
}

/// Size variants for the select
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, proa_macros::ProaVariant)]
#[variant(name = "size", path = "crate::components::_proa::variant_spec")]
pub enum SelectSize {
    Sm,
    #[default]
    Default,
    Lg,
}

impl SelectSize {
    /// Get the Tailwind classes for this size
    pub const fn classes(self) -> &'static str {
        match self {
            SelectSize::Sm => "h-8 px-2.5 py-1.5 text-xs",
            SelectSize::Default => "h-9 px-3 py-1",
            SelectSize::Lg => "h-10 px-4 py-2 text-base",
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Default => "default",
            Self::Lg => "lg",
        }
    }
}

/// Optional inputs accepted by [`SelectRecipe::resolve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SelectRecipeProps {
    pub size: Option<SelectSize>,
}

/// Fully typed select recipe with compile-time defaults.
#[derive(Debug, Clone, Copy)]
pub struct SelectRecipe {
    base: &'static str,
    size_default: SelectSize,
}

/// Global select recipe instance
pub const SELECT_RECIPE: SelectRecipe = SelectRecipe {
    base: base::SELECT_BASE,
    size_default: SelectSize::Default,
};

impl SelectRecipe {
    pub const fn resolve(&'static self, props: SelectRecipeProps) -> SelectStyle {
        let size = match props.size {
            Some(size) => size,
            None => self.size_default,
        };
        SelectStyle { recipe: self, size }
    }
}

/// Fully resolved, streamable Select style.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct SelectStyle {
    recipe: &'static SelectRecipe,
    size: SelectSize,
}

impl SelectStyle {
    pub const fn size(self) -> SelectSize {
        self.size
    }
}

impl AttrValue for SelectStyle {
    #[inline(always)]
    fn should_render(&self) -> bool {
        true
    }

    #[inline(always)]
    fn render_attr_value<B: WriteBuf, L: DataLoader>(
        &self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), WriteError> {
        cx.out.extend_static(self.recipe.base.as_bytes())?;
        cx.out.extend_static(b" ")?;
        cx.out.extend_static(self.size.classes().as_bytes())
    }
}

/// Get the global select recipe
pub const fn select_recipe() -> &'static SelectRecipe {
    &SELECT_RECIPE
}

/// Compatibility resolver for callers of the former helper.
pub const fn select_classes(size: SelectSize) -> SelectStyle {
    SELECT_RECIPE.resolve(SelectRecipeProps { size: Some(size) })
}

impl crate::components::_proa::variant_spec::ComponentSpec for SelectRecipe {
    const NAME: &'static str = "Select";

    fn base_classes(&self) -> &'static str {
        self.base
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        vec![crate::components::_proa::variant_spec::variant_metadata::<SelectSize>()]
    }

    fn anatomy(&self) -> Option<crate::components::_proa::Anatomy> {
        Some(super::SELECT_ANATOMY)
    }
}
