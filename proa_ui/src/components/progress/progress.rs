// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Progress component implementation
//!
//! Renders a progress bar showing task completion percentage.
//! Pure CSS component with no JS hydration needed.
use super::recipe::{ProgressRecipeProps, ProgressSize, PROGRESS_RECIPE};
use crate::components::_proa::Anatomy;
use proa_core::{WebContext, WebRenderSync, WriteBuf};
use proa_macros::{html_sync, text};

/// Stable DOM contract shared by linear and circular progress renderers.
pub const PROGRESS_ANATOMY: Anatomy =
    Anatomy::new("progress", &["root", "track", "indicator", "value-text"]);
/// Progress component - displays a progress bar indicating completion percentage
///
/// # Example
///
/// ```ignore
/// use crate::components::Progress;
/// use proa_macros::html_sync;
///
/// html_sync! {
///     <div>
///         {Progress {
///             value: 33,
///             ..Progress::new()
///         }}
///     </div>
/// }.render(cx);
/// ```
pub struct Progress {
    /// Progress value from 0 to 100
    pub value: u8,
    /// Additional CSS classes for the track
    pub class: Option<&'static str>,
    /// HTML id attribute
    pub id: Option<&'static str>,
    /// Accessible label for screen readers
    pub aria_label: Option<&'static str>,
}
impl Progress {
    /// Create a new progress bar with default settings (0%)
    pub const fn new() -> Self {
        Self {
            value: 0,
            class: None,
            id: None,
            aria_label: None,
        }
    }
}
impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
}
impl<B: WriteBuf, L: ::proa_core::DataLoader> WebRenderSync<L, B> for Progress {
    #[inline(always)]
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = PROGRESS_RECIPE.resolve(ProgressRecipeProps);
        let value = self.value.min(100);
        let translate = 100u8.saturating_sub(value);
        html_sync! {
            <div
                _a11y_ok
                data-progress
                data-scope={PROGRESS_ANATOMY.scope()}
                data-part="root"
                data-slot="progress"
                role="progressbar"
                aria-valuenow={value}
                aria-valuemin="0"
                aria-valuemax="100"
                class={text!("{} {}", style, self.class)}
                id={self.id}
                aria-label={self.aria_label}
            >
                <div
                    data-progress-indicator
                    data-scope={PROGRESS_ANATOMY.scope()}
                    data-part="indicator"
                    data-slot="progress-indicator"
                    class={PROGRESS_RECIPE.indicator}
                    style={proa_core::style_ok(text!("transform: translateX(-{}%)", translate))}
                ></div>
            </div>
        }
        .render(cx)
    }
}

/// Circular progress indicator rendered as server-side SVG.
pub struct CircularProgress {
    /// Progress value from 0 to 100.
    pub value: u8,
    pub size: Option<ProgressSize>,
    pub stroke_width: u8,
    pub class: Option<&'static str>,
    pub id: Option<&'static str>,
    pub aria_label: Option<&'static str>,
}

impl CircularProgress {
    pub const fn new() -> Self {
        Self {
            value: 0,
            size: None,
            stroke_width: 3,
            class: None,
            id: None,
            aria_label: None,
        }
    }
}

impl Default for CircularProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl<B: WriteBuf, L: ::proa_core::DataLoader> WebRenderSync<L, B> for CircularProgress {
    #[inline(always)]
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let value = self.value.min(100);
        let dash_offset = 100u8.saturating_sub(value);
        let style = PROGRESS_RECIPE.resolve_circular(self.size);

        html_sync! {
            <svg
                _a11y_ok
                data-progress
                data-progress-circular
                data-scope={PROGRESS_ANATOMY.scope()}
                data-part="root"
                data-slot="progress-circular"
                data-size={style.size().as_str()}
                id={self.id}
                role="progressbar"
                aria-label={self.aria_label}
                aria-valuenow={value}
                aria-valuemin="0"
                aria-valuemax="100"
                viewBox="0 0 36 36"
                class={text!("{} {}", style, self.class)}
            >
                <circle
                    data-scope={PROGRESS_ANATOMY.scope()}
                    data-part="track"
                    data-slot="progress-circular-track"
                    cx="18"
                    cy="18"
                    r="15.9155"
                    pathLength="100"
                    stroke-width={self.stroke_width}
                    class={PROGRESS_RECIPE.circular_track}
                />
                <circle
                    data-progress-indicator
                    data-scope={PROGRESS_ANATOMY.scope()}
                    data-part="indicator"
                    data-slot="progress-circular-indicator"
                    cx="18"
                    cy="18"
                    r="15.9155"
                    pathLength="100"
                    stroke-linecap="round"
                    stroke-dasharray="100"
                    stroke-dashoffset={dash_offset}
                    stroke-width={self.stroke_width}
                    class={PROGRESS_RECIPE.circular_indicator}
                />
            </svg>
        }
        .render(cx)
    }
}
