// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Image component with lazy loading, responsive srcset, and CLS prevention.

#[allow(clippy::module_inception)]
mod image;
mod md;
mod recipe;

pub use image::{Image, ImageLoading, ImagePriority};
pub use recipe::{
    image_base, image_recipe, ImageFit, ImageRecipe, ImageRecipeProps, ImageStyle, IMAGE_RECIPE,
};
