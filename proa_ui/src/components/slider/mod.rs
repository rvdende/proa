// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Native-input-first Slider component.
//!
//! Slider keeps the browser's range input as the accessible and form-owning
//! control, then exposes stable compound parts for styling and render targets.

use crate::components::_proa::Anatomy;

mod md;
mod recipe;
mod slider;

/// Stable DOM parts shared by static and controlled Slider renderers.
pub const SLIDER_ANATOMY: Anatomy = Anatomy::new(
    "slider",
    &[
        "root",
        "label",
        "value-text",
        "control",
        "track",
        "range",
        "thumb",
        "hidden-input",
        "marker-group",
        "marker",
    ],
);

pub use recipe::{
    slider_recipe, SliderOrientation, SliderRecipe, SliderRecipeProps, SliderStyle, SLIDER_RECIPE,
};
#[doc(hidden)]
pub use slider::{resolve_slider_values, ResolvedSliderValues, SliderPositionStyle};
pub use slider::{Slider, SliderMarker};
