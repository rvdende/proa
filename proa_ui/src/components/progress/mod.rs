// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::module_inception)]
//! Progress component - displays a progress bar indicating completion percentage
//!
//! The Progress component shows task completion as a horizontal bar.
//! It is a pure CSS component with no JS hydration needed.

mod md;
mod progress;
mod recipe;

// Re-export progress types
pub use progress::{CircularProgress, Progress, PROGRESS_ANATOMY};

// Re-export recipe types
pub use recipe::{
    progress_indicator_classes, progress_recipe, progress_track_classes, CircularProgressStyle,
    ProgressRecipe, ProgressRecipeProps, ProgressSize, ProgressStyle, PROGRESS_RECIPE,
};
