// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::module_inception)]
//! Avatar component - displays user profile images with fallback support
//!
//! The Avatar component provides a styled circular (or rounded) image container
//! with automatic fallback to initials or placeholder when the image fails to load.

use crate::components::_proa::Anatomy;

mod avatar;
mod md;
mod recipe;

/// Stable DOM parts exposed by Avatar.
pub const AVATAR_ANATOMY: Anatomy = Anatomy::new("avatar", &["root", "image", "fallback"]);

// Re-export avatar types
pub use avatar::{Avatar, AvatarFallback, AvatarImage};

// Re-export recipe types
pub use recipe::{
    avatar_classes, avatar_fallback_base_classes, avatar_fallback_classes, avatar_image_classes,
    avatar_recipe, AvatarFallbackStyle, AvatarRecipe, AvatarRecipeProps, AvatarShape, AvatarSize,
    AvatarStyle, AVATAR_RECIPE,
};

// Re-export metadata export function (only when feature is enabled)
