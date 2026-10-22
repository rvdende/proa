// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Server-rendered native Slider.

use super::{SliderOrientation, SliderRecipeProps, SLIDER_ANATOMY, SLIDER_RECIPE};
use crate::components::{FormControlIds, FormControlState, PartId};
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WebRenderSync, WriteBuf, WriteError};
use proa_macros::html_sync;

/// A labeled position marker rendered along a Slider track.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SliderMarker<'a> {
    pub value: f64,
    pub label: Option<&'a str>,
}

impl SliderMarker<'_> {
    pub const fn new(value: f64) -> Self {
        Self { value, label: None }
    }

    pub const fn labeled(value: f64, label: &'static str) -> SliderMarker<'static> {
        SliderMarker {
            value,
            label: Some(label),
        }
    }
}

/// A native-input-first single-value Slider.
///
/// The range input remains the browser-owned accessible and form control.
/// The surrounding parts expose a stable anatomy for styling and codegen.
pub struct Slider<'a> {
    pub label: &'a str,
    pub value_text: Option<&'a str>,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub value: f64,
    pub orientation: SliderOrientation,
    pub state: FormControlState,
    pub markers: &'a [SliderMarker<'a>],
    pub id: Option<&'a str>,
    pub name: Option<&'a str>,
    pub aria_describedby: Option<&'a str>,
    pub class: Option<&'a str>,
}

impl<'a> Slider<'a> {
    pub const fn new(label: &'a str) -> Self {
        Self {
            label,
            value_text: None,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            value: 0.0,
            orientation: SliderOrientation::Horizontal,
            state: FormControlState::enabled(),
            markers: &[],
            id: None,
            name: None,
            aria_describedby: None,
            class: None,
        }
    }

    pub const fn with_id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }
}

impl Default for Slider<'static> {
    fn default() -> Self {
        Self::new("Value")
    }
}

impl<B: WriteBuf, L: DataLoader> WebRenderSync<L, B> for Slider<'_> {
    fn render(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        let values = resolve_slider_values(self.min, self.max, self.step, self.value);
        let style = SLIDER_RECIPE.resolve(SliderRecipeProps {
            orientation: Some(self.orientation),
        });
        let ids = FormControlIds::new(self.id);
        let control_id = ids.control();
        let label_id = ids.label();
        let value_text_id = PartId::new(self.id, "value-text");
        let range_name = if self.state.read_only {
            None
        } else {
            self.name
        };
        let effective_disabled = self.state.disabled || self.state.read_only;
        let aria_invalid = if self.state.invalid {
            Some("true")
        } else {
            None
        };
        let aria_read_only = if self.state.read_only {
            Some("true")
        } else {
            None
        };
        let aria_required = if self.state.required {
            Some("true")
        } else {
            None
        };
        let data_disabled = if self.state.disabled { "true" } else { "false" };
        let data_invalid = if self.state.invalid { "true" } else { "false" };
        let data_read_only = if self.state.read_only {
            "true"
        } else {
            "false"
        };
        let data_required = if self.state.required { "true" } else { "false" };

        html_sync! {
            <div
                data-slider
                data-scope={SLIDER_ANATOMY.scope()}
                data-part="root"
                data-slot="slider"
                data-orientation={style.orientation().as_str()}
                data-disabled={data_disabled}
                data-invalid={data_invalid}
                data-read-only={data_read_only}
                data-required={data_required}
                id={self.id}
                class={proa_macros::text!("{} {}", style, self.class)}
            >
                <label
                    data-scope={SLIDER_ANATOMY.scope()}
                    data-part="label"
                    data-slot="slider-label"
                    id={label_id}
                    for={control_id}
                    class={SLIDER_RECIPE.label}
                >
                    <span>{self.label}</span>
                    <output
                        data-scope={SLIDER_ANATOMY.scope()}
                        data-part="value-text"
                        data-slot="slider-value-text"
                        id={value_text_id}
                        for={control_id}
                        class={SLIDER_RECIPE.value_text}
                    >
                        {SliderValueDisplay { value: values.value, text: self.value_text }}
                    </output>
                    <span
                        data-scope={SLIDER_ANATOMY.scope()}
                        data-part="control"
                        data-slot="slider-control"
                        data-position={values.position}
                        class={SLIDER_RECIPE.control}
                        style={proa_core::style_ok(SliderPositionStyle::value(values.position))}
                    >
                        <span
                            data-scope={SLIDER_ANATOMY.scope()}
                            data-part="track"
                            data-slot="slider-track"
                            aria-hidden="true"
                            class={SLIDER_RECIPE.track}
                        >
                            <span
                                data-scope={SLIDER_ANATOMY.scope()}
                                data-part="range"
                                data-slot="slider-range"
                                class={SLIDER_RECIPE.range}
                            ></span>
                        </span>
                        <span
                            data-scope={SLIDER_ANATOMY.scope()}
                            data-part="thumb"
                            data-slot="slider-thumb"
                            aria-hidden="true"
                            class={SLIDER_RECIPE.thumb}
                        ></span>
                        <span
                            data-scope={SLIDER_ANATOMY.scope()}
                            data-part="marker-group"
                            data-slot="slider-marker-group"
                            aria-hidden="true"
                            class={SLIDER_RECIPE.marker_group}
                        >
                            {for marker in self.markers {
                                let marker_position = values.position_for(marker.value);
                                html_sync! {
                                    <span
                                        data-scope={SLIDER_ANATOMY.scope()}
                                        data-part="marker"
                                        data-slot="slider-marker"
                                        data-value={marker.value}
                                        data-position={marker_position}
                                        class={SLIDER_RECIPE.marker}
                                        style={proa_core::style_ok(SliderPositionStyle::marker(marker_position))}
                                    >
                                        {SliderValueDisplay { value: marker.value, text: marker.label }}
                                    </span>
                                }
                            }}
                        </span>
                        <input
                            type="range"
                            data-slider-input
                            data-scope={SLIDER_ANATOMY.scope()}
                            data-part="hidden-input"
                            data-slot="slider-hidden-input"
                            data-orientation={style.orientation().as_str()}
                            data-disabled={data_disabled}
                            data-invalid={data_invalid}
                            data-read-only={data_read_only}
                            id={control_id}
                            name={range_name}
                            min={values.min}
                            max={values.max}
                            step={values.step}
                            value={values.value}
                            disabled={effective_disabled}
                            readonly={self.state.read_only}
                            required={self.state.required}
                            aria-labelledby={label_id}
                            aria-describedby={self.aria_describedby}
                            aria-orientation={style.orientation().as_str()}
                            aria-valuemin={values.min}
                            aria-valuemax={values.max}
                            aria-valuenow={values.value}
                            aria-valuetext={self.value_text}
                            aria-invalid={aria_invalid}
                            aria-readonly={aria_read_only}
                            aria-required={aria_required}
                            class={SLIDER_RECIPE.hidden_input}
                        />
                    </span>
                    <input
                        type="hidden"
                        data-slider-readonly-value
                        name={self.name}
                        value={values.value}
                        disabled={!self.state.read_only || self.state.disabled}
                    />
                </label>
            </div>
        }
        .render(cx)
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct ResolvedSliderValues {
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub value: f64,
    pub position: f64,
}

impl ResolvedSliderValues {
    pub fn position_for(self, value: f64) -> f64 {
        let value = if value.is_finite() {
            value.clamp(self.min, self.max)
        } else {
            self.min
        };
        ((value - self.min) / (self.max - self.min) * 100.0).clamp(0.0, 100.0)
    }
}

#[doc(hidden)]
pub fn resolve_slider_values(
    requested_min: f64,
    requested_max: f64,
    requested_step: f64,
    requested_value: f64,
) -> ResolvedSliderValues {
    let (min, max) = if requested_min.is_finite()
        && requested_max.is_finite()
        && requested_max > requested_min
    {
        (requested_min, requested_max)
    } else {
        (0.0, 100.0)
    };
    let step = if requested_step.is_finite() && requested_step > 0.0 {
        requested_step
    } else {
        1.0
    };
    let value = if requested_value.is_finite() {
        requested_value.clamp(min, max)
    } else {
        min
    };
    let position = ((value - min) / (max - min) * 100.0).clamp(0.0, 100.0);

    ResolvedSliderValues {
        min,
        max,
        step,
        value,
        position,
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct SliderPositionStyle {
    position: f64,
    marker: bool,
}

impl SliderPositionStyle {
    pub const fn value(position: f64) -> Self {
        Self {
            position,
            marker: false,
        }
    }

    pub const fn marker(position: f64) -> Self {
        Self {
            position,
            marker: true,
        }
    }
}

impl AttrValue for SliderPositionStyle {
    fn should_render(&self) -> bool {
        true
    }

    fn render_attr_value<Buf: WriteBuf, Loader: DataLoader>(
        &self,
        cx: &mut WebContext<Loader, Buf>,
    ) -> Result<(), WriteError> {
        if self.marker {
            cx.out.extend_static(b"--slider-marker-position:")?;
        } else {
            cx.out.extend_static(b"--slider-position:")?;
        }
        self.position.render_attr_value(cx)?;
        cx.out.extend_static(b";")
    }
}

#[derive(Debug, Clone, Copy)]
struct SliderValueDisplay<'a> {
    value: f64,
    text: Option<&'a str>,
}

impl<B: WriteBuf, L: DataLoader> WebRenderSync<L, B> for SliderValueDisplay<'_> {
    fn render(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        match self.text {
            Some(text) => text.render(cx),
            None => self.value.render(cx),
        }
    }
}
