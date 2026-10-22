// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Avatar recipe — typed root axes and named image/fallback slots.
//!
//! A nested fallback follows the root Avatar's `data-size` through a named
//! group. Its legacy local size still provides sensible standalone rendering,
//! while the root remains authoritative when the parts are composed.

use crate::tw_join;
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

mod base {
    use super::*;

    pub const ROOT: &str = tw_join!(
        "group/avatar",
        "relative",
        "flex",
        "shrink-0",
        "overflow-hidden"
    );
    pub const IMAGE: &str = tw_join!("aspect-square", "h-full", "w-full", "object-cover");
    pub const FALLBACK: &str = tw_join!(
        "flex",
        "h-full",
        "w-full",
        "items-center",
        "justify-center",
        "bg-muted",
        "text-muted-foreground",
        "font-medium",
        "group-data-[size=xs]/avatar:text-xs",
        "group-data-[size=sm]/avatar:text-xs",
        "group-data-[size=md]/avatar:text-sm",
        "group-data-[size=lg]/avatar:text-base",
        "group-data-[size=xl]/avatar:text-lg",
        "group-data-[size=xxl]/avatar:text-2xl"
    );
}

/// Size variants for the avatar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, proa_macros::ProaVariant)]
#[variant(name = "size", path = "crate::components::_proa::variant_spec")]
pub enum AvatarSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
    Xxl,
}

impl AvatarSize {
    pub const fn classes(self) -> &'static str {
        match self {
            Self::Xs => "size-6",
            Self::Sm => "size-8",
            Self::Md => "size-10",
            Self::Lg => "size-12",
            Self::Xl => "size-16",
            Self::Xxl => "size-24",
        }
    }

    pub const fn text_classes(self) -> &'static str {
        match self {
            Self::Xs | Self::Sm => "text-xs",
            Self::Md => "text-sm",
            Self::Lg => "text-base",
            Self::Xl => "text-lg",
            Self::Xxl => "text-2xl",
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Xs => "xs",
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Xl => "xl",
            Self::Xxl => "xxl",
        }
    }
}

/// Shape variants for the avatar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, proa_macros::ProaVariant)]
#[variant(name = "shape", path = "crate::components::_proa::variant_spec")]
pub enum AvatarShape {
    #[default]
    Circle,
    Square,
}

impl AvatarShape {
    pub const fn classes(self) -> &'static str {
        match self {
            Self::Circle => "rounded-full",
            Self::Square => "rounded-lg",
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Circle => "circle",
            Self::Square => "square",
        }
    }
}

/// Optional inputs accepted by [`AvatarRecipe::resolve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AvatarRecipeProps {
    pub size: Option<AvatarSize>,
    pub shape: Option<AvatarShape>,
}

/// Static named classes and defaults for Avatar.
#[derive(Debug, Clone, Copy)]
pub struct AvatarRecipe {
    pub root: &'static str,
    pub image: &'static str,
    pub fallback: &'static str,
    size_default: AvatarSize,
    shape_default: AvatarShape,
}

impl AvatarRecipe {
    pub const fn resolve(&'static self, props: AvatarRecipeProps) -> AvatarStyle {
        let size = match props.size {
            Some(size) => size,
            None => self.size_default,
        };
        let shape = match props.shape {
            Some(shape) => shape,
            None => self.shape_default,
        };
        AvatarStyle {
            recipe: self,
            size,
            shape,
        }
    }

    /// Resolve a standalone fallback's local text size.
    ///
    /// When nested, the root's named-group selectors have greater specificity
    /// and make the root `data-size` authoritative.
    pub const fn resolve_fallback(&'static self, size: AvatarSize) -> AvatarFallbackStyle {
        AvatarFallbackStyle { recipe: self, size }
    }
}

/// Fully resolved, streamable Avatar root style.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct AvatarStyle {
    recipe: &'static AvatarRecipe,
    size: AvatarSize,
    shape: AvatarShape,
}

impl AvatarStyle {
    pub const fn size(self) -> AvatarSize {
        self.size
    }

    pub const fn shape(self) -> AvatarShape {
        self.shape
    }
}

impl AttrValue for AvatarStyle {
    #[inline(always)]
    fn should_render(&self) -> bool {
        true
    }

    #[inline(always)]
    fn render_attr_value<B: WriteBuf, L: DataLoader>(
        &self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), WriteError> {
        cx.out.extend_static(self.recipe.root.as_bytes())?;
        cx.out.extend_static(b" ")?;
        cx.out.extend_static(self.size.classes().as_bytes())?;
        cx.out.extend_static(b" ")?;
        cx.out.extend_static(self.shape.classes().as_bytes())
    }
}

/// Streamable fallback slot plus its standalone size fallback.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct AvatarFallbackStyle {
    recipe: &'static AvatarRecipe,
    size: AvatarSize,
}

impl AttrValue for AvatarFallbackStyle {
    #[inline(always)]
    fn should_render(&self) -> bool {
        true
    }

    #[inline(always)]
    fn render_attr_value<B: WriteBuf, L: DataLoader>(
        &self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), WriteError> {
        cx.out.extend_static(self.recipe.fallback.as_bytes())?;
        cx.out.extend_static(b" ")?;
        cx.out.extend_static(self.size.text_classes().as_bytes())
    }
}

pub const AVATAR_RECIPE: AvatarRecipe = AvatarRecipe {
    root: base::ROOT,
    image: base::IMAGE,
    fallback: base::FALLBACK,
    size_default: AvatarSize::Md,
    shape_default: AvatarShape::Circle,
};

pub const fn avatar_recipe() -> &'static AvatarRecipe {
    &AVATAR_RECIPE
}

/// Compatibility delegates for the former helpers.
pub const fn avatar_classes(size: AvatarSize, shape: AvatarShape) -> AvatarStyle {
    AVATAR_RECIPE.resolve(AvatarRecipeProps {
        size: Some(size),
        shape: Some(shape),
    })
}

pub const fn avatar_image_classes() -> &'static str {
    AVATAR_RECIPE.image
}

pub const fn avatar_fallback_base_classes() -> &'static str {
    AVATAR_RECIPE.fallback
}

pub const fn avatar_fallback_classes(size: AvatarSize) -> AvatarFallbackStyle {
    AVATAR_RECIPE.resolve_fallback(size)
}

impl crate::components::_proa::variant_spec::ComponentSpec for AvatarRecipe {
    const NAME: &'static str = "Avatar";

    fn base_classes(&self) -> &'static str {
        self.root
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        vec![
            crate::components::_proa::variant_spec::variant_metadata::<AvatarSize>(),
            crate::components::_proa::variant_spec::variant_metadata::<AvatarShape>(),
        ]
    }

    fn anatomy(&self) -> Option<crate::components::_proa::Anatomy> {
        Some(super::AVATAR_ANATOMY)
    }
}
