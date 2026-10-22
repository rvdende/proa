// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Skeleton component - animated loading placeholder

mod md;
pub mod recipe;
mod skeleton;

pub use recipe::{
    skeleton_classes, skeleton_recipe, SkeletonRecipe, SkeletonRecipeProps, SkeletonStyle,
    SKELETON_RECIPE,
};
pub use skeleton::Skeleton;
