// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Progress recipe — static named slots for the track and indicator.
//!
//! Progress has no visual axes today, but it is still a multipart component:
//! the public component owns the track while the indicator is a distinct
//! styled part. Keeping both slots in one recipe prevents their styling from
//! drifting apart and leaves a typed extension point for future axes.

use crate::tw_join;
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

mod base {
    use super::*;

    pub const PROGRESS_TRACK: &str = tw_join!(
        "relative",
        "h-2",
        "w-full",
        "overflow-hidden",
        "rounded-full",
        "bg-primary/20"
    );

    pub const PROGRESS_INDICATOR: &str = tw_join!(
        "h-full",
        "w-full",
        "flex-1",
        "rounded-full",
        "bg-primary",
        "transition-all"
    );

    pub const CIRCULAR_ROOT: &str =
        tw_join!("inline-flex", "shrink-0", "text-primary", "-rotate-90");

    pub const CIRCULAR_TRACK: &str = tw_join!("fill-none", "stroke-current", "opacity-20");

    pub const CIRCULAR_INDICATOR: &str = tw_join!(
        "fill-none",
        "stroke-current",
        "transition-[stroke-dashoffset]",
        "duration-300"
    );
}

/// Diameter variants for [`super::CircularProgress`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, proa_macros::ProaVariant)]
#[variant(name = "size", path = "crate::components::_proa::variant_spec")]
pub enum ProgressSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl ProgressSize {
    pub const fn classes(self) -> &'static str {
        match self {
            Self::Sm => "size-6",
            Self::Md => "size-10",
            Self::Lg => "size-16",
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }
}

/// Optional inputs accepted by [`ProgressRecipe::resolve`].
///
/// This is intentionally empty until Progress gains a genuine visual axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ProgressRecipeProps;

/// Static named slots for the Progress compound component.
#[derive(Debug, Clone, Copy)]
pub struct ProgressRecipe {
    pub root: &'static str,
    pub indicator: &'static str,
    pub circular_root: &'static str,
    pub circular_track: &'static str,
    pub circular_indicator: &'static str,
}

impl ProgressRecipe {
    /// Resolve the track style without allocating class storage.
    pub const fn resolve(&'static self, _props: ProgressRecipeProps) -> ProgressStyle {
        ProgressStyle { recipe: self }
    }

    pub const fn resolve_circular(
        &'static self,
        size: Option<ProgressSize>,
    ) -> CircularProgressStyle {
        CircularProgressStyle {
            recipe: self,
            size: match size {
                Some(size) => size,
                None => ProgressSize::Md,
            },
        }
    }
}

/// Fully resolved, streamable Progress track style.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct ProgressStyle {
    recipe: &'static ProgressRecipe,
}

impl AttrValue for ProgressStyle {
    #[inline(always)]
    fn should_render(&self) -> bool {
        true
    }

    #[inline(always)]
    fn render_attr_value<B: WriteBuf, L: DataLoader>(
        &self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), WriteError> {
        cx.out.extend_static(self.recipe.root.as_bytes())
    }
}

/// Fully resolved, streamable circular progress root style.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct CircularProgressStyle {
    recipe: &'static ProgressRecipe,
    size: ProgressSize,
}

impl CircularProgressStyle {
    pub const fn size(self) -> ProgressSize {
        self.size
    }
}

impl AttrValue for CircularProgressStyle {
    #[inline(always)]
    fn should_render(&self) -> bool {
        true
    }

    #[inline(always)]
    fn render_attr_value<B: WriteBuf, L: DataLoader>(
        &self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), WriteError> {
        cx.out.extend_static(self.recipe.circular_root.as_bytes())?;
        cx.out.extend_static(b" ")?;
        cx.out.extend_static(self.size.classes().as_bytes())
    }
}

/// Global, zero-initialization Progress recipe.
pub const PROGRESS_RECIPE: ProgressRecipe = ProgressRecipe {
    root: base::PROGRESS_TRACK,
    indicator: base::PROGRESS_INDICATOR,
    circular_root: base::CIRCULAR_ROOT,
    circular_track: base::CIRCULAR_TRACK,
    circular_indicator: base::CIRCULAR_INDICATOR,
};

pub const fn progress_recipe() -> &'static ProgressRecipe {
    &PROGRESS_RECIPE
}

impl crate::components::_proa::variant_spec::ComponentSpec for ProgressRecipe {
    const NAME: &'static str = "Progress";

    fn base_classes(&self) -> &'static str {
        self.root
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        Vec::new()
    }

    fn anatomy(&self) -> Option<crate::components::_proa::Anatomy> {
        Some(super::PROGRESS_ANATOMY)
    }
}

/// Compatibility resolver for the former track helper.
pub const fn progress_track_classes() -> ProgressStyle {
    PROGRESS_RECIPE.resolve(ProgressRecipeProps)
}

/// Compatibility accessor for the former indicator helper.
pub const fn progress_indicator_classes() -> &'static str {
    PROGRESS_RECIPE.indicator
}
