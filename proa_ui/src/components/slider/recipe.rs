// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Slider recipe — typed root orientation and stable compound-part slots.

use crate::tw_join;
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

/// Slider layout orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, proa_macros::ProaVariant)]
#[variant(name = "orientation", path = "crate::components::_proa::variant_spec")]
pub enum SliderOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl SliderOrientation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    pub const fn classes(self) -> &'static str {
        match self {
            Self::Horizontal => "w-full",
            Self::Vertical => "w-fit",
        }
    }
}

/// Optional root axes accepted by [`SliderRecipe::resolve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SliderRecipeProps {
    pub orientation: Option<SliderOrientation>,
}

/// Static classes for every styled Slider part.
#[derive(Debug, Clone, Copy)]
pub struct SliderRecipe {
    pub root: &'static str,
    pub label: &'static str,
    pub value_text: &'static str,
    pub control: &'static str,
    pub track: &'static str,
    pub range: &'static str,
    pub thumb: &'static str,
    pub hidden_input: &'static str,
    pub marker_group: &'static str,
    pub marker: &'static str,
    orientation_default: SliderOrientation,
}

impl SliderRecipe {
    pub const fn resolve(&'static self, props: SliderRecipeProps) -> SliderStyle {
        SliderStyle {
            recipe: self,
            orientation: match props.orientation {
                Some(orientation) => orientation,
                None => self.orientation_default,
            },
        }
    }
}

/// Fully resolved, allocation-free Slider root style.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct SliderStyle {
    recipe: &'static SliderRecipe,
    orientation: SliderOrientation,
}

impl SliderStyle {
    pub const fn orientation(self) -> SliderOrientation {
        self.orientation
    }
}

impl AttrValue for SliderStyle {
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
        cx.out.extend_static(self.orientation.classes().as_bytes())
    }
}

const SLIDER_ROOT: &str = tw_join!(
    "group/slider",
    "text-foreground",
    "data-[disabled=true]:opacity-50",
    "data-[read-only=true]:opacity-75"
);

const SLIDER_LABEL: &str = tw_join!(
    "grid",
    "grid-cols-[minmax(0,1fr)_auto]",
    "items-center",
    "gap-x-3",
    "gap-y-2",
    "text-sm",
    "font-medium"
);

const SLIDER_VALUE_TEXT: &str = tw_join!(
    "text-sm",
    "font-medium",
    "tabular-nums",
    "text-muted-foreground"
);

const SLIDER_CONTROL: &str = tw_join!(
    "relative",
    "col-span-2",
    "block",
    "touch-none",
    "select-none",
    "group-data-[orientation=horizontal]/slider:h-5",
    "group-data-[orientation=horizontal]/slider:w-full",
    "group-data-[orientation=vertical]/slider:h-48",
    "group-data-[orientation=vertical]/slider:w-5",
    "group-data-[orientation=vertical]/slider:justify-self-center"
);

const SLIDER_TRACK: &str = tw_join!(
    "pointer-events-none",
    "absolute",
    "rounded-full",
    "bg-primary/20",
    "group-data-[orientation=horizontal]/slider:left-0",
    "group-data-[orientation=horizontal]/slider:top-1/2",
    "group-data-[orientation=horizontal]/slider:h-1.5",
    "group-data-[orientation=horizontal]/slider:w-full",
    "group-data-[orientation=horizontal]/slider:-translate-y-1/2",
    "group-data-[orientation=vertical]/slider:bottom-0",
    "group-data-[orientation=vertical]/slider:left-1/2",
    "group-data-[orientation=vertical]/slider:h-full",
    "group-data-[orientation=vertical]/slider:w-1.5",
    "group-data-[orientation=vertical]/slider:-translate-x-1/2"
);

const SLIDER_RANGE: &str = tw_join!(
    "absolute",
    "rounded-full",
    "bg-primary",
    "group-data-[orientation=horizontal]/slider:left-0",
    "group-data-[orientation=horizontal]/slider:top-0",
    "group-data-[orientation=horizontal]/slider:h-full",
    "group-data-[orientation=horizontal]/slider:[width:calc(var(--slider-position)*1%)]",
    "group-data-[orientation=vertical]/slider:bottom-0",
    "group-data-[orientation=vertical]/slider:left-0",
    "group-data-[orientation=vertical]/slider:w-full",
    "group-data-[orientation=vertical]/slider:[height:calc(var(--slider-position)*1%)]"
);

// The browser-owned range thumb is the moving visual thumb. This DOM part is
// retained as a stable anatomy/styling hook without leaving a stale duplicate
// behind when the no-JS native input moves.
const SLIDER_THUMB: &str = tw_join!(
    "pointer-events-none",
    "absolute",
    "size-4",
    "rounded-full",
    "border-2",
    "border-primary",
    "bg-background",
    "opacity-0",
    "shadow-sm",
    "group-data-[orientation=horizontal]/slider:left-[calc(var(--slider-position)*1%)]",
    "group-data-[orientation=horizontal]/slider:top-1/2",
    "group-data-[orientation=horizontal]/slider:-translate-x-1/2",
    "group-data-[orientation=horizontal]/slider:-translate-y-1/2",
    "group-data-[orientation=vertical]/slider:bottom-[calc(var(--slider-position)*1%)]",
    "group-data-[orientation=vertical]/slider:left-1/2",
    "group-data-[orientation=vertical]/slider:-translate-x-1/2",
    "group-data-[orientation=vertical]/slider:translate-y-1/2"
);

const SLIDER_HIDDEN_INPUT: &str = tw_join!(
    "absolute",
    "inset-0",
    "z-10",
    "m-0",
    "h-full",
    "w-full",
    "cursor-pointer",
    "appearance-none",
    "bg-transparent",
    "outline-hidden",
    "focus-visible:ring-2",
    "focus-visible:ring-ring",
    "focus-visible:ring-offset-2",
    "disabled:cursor-not-allowed",
    "[&::-webkit-slider-runnable-track]:h-1.5",
    "[&::-webkit-slider-runnable-track]:rounded-full",
    "[&::-webkit-slider-runnable-track]:bg-transparent",
    "[&::-webkit-slider-thumb]:mt-[-5px]",
    "[&::-webkit-slider-thumb]:size-4",
    "[&::-webkit-slider-thumb]:appearance-none",
    "[&::-webkit-slider-thumb]:rounded-full",
    "[&::-webkit-slider-thumb]:border-2",
    "[&::-webkit-slider-thumb]:border-primary",
    "[&::-webkit-slider-thumb]:bg-background",
    "[&::-webkit-slider-thumb]:shadow-sm",
    "[&::-moz-range-track]:h-1.5",
    "[&::-moz-range-track]:rounded-full",
    "[&::-moz-range-track]:bg-transparent",
    "[&::-moz-range-thumb]:size-4",
    "[&::-moz-range-thumb]:rounded-full",
    "[&::-moz-range-thumb]:border-2",
    "[&::-moz-range-thumb]:border-primary",
    "[&::-moz-range-thumb]:bg-background",
    "[&::-moz-range-thumb]:shadow-sm",
    "group-data-[orientation=vertical]/slider:[direction:rtl]",
    "group-data-[orientation=vertical]/slider:[writing-mode:vertical-lr]"
);

const SLIDER_MARKER_GROUP: &str = tw_join!(
    "pointer-events-none",
    "absolute",
    "inset-0",
    "text-[10px]",
    "font-normal",
    "text-muted-foreground"
);

const SLIDER_MARKER: &str = tw_join!(
    "absolute",
    "whitespace-nowrap",
    "group-data-[orientation=horizontal]/slider:left-[calc(var(--slider-marker-position)*1%)]",
    "group-data-[orientation=horizontal]/slider:top-full",
    "group-data-[orientation=horizontal]/slider:mt-1",
    "group-data-[orientation=horizontal]/slider:-translate-x-1/2",
    "group-data-[orientation=vertical]/slider:bottom-[calc(var(--slider-marker-position)*1%)]",
    "group-data-[orientation=vertical]/slider:right-full",
    "group-data-[orientation=vertical]/slider:mr-2",
    "group-data-[orientation=vertical]/slider:translate-y-1/2"
);

/// Global, zero-initialization Slider recipe.
pub const SLIDER_RECIPE: SliderRecipe = SliderRecipe {
    root: SLIDER_ROOT,
    label: SLIDER_LABEL,
    value_text: SLIDER_VALUE_TEXT,
    control: SLIDER_CONTROL,
    track: SLIDER_TRACK,
    range: SLIDER_RANGE,
    thumb: SLIDER_THUMB,
    hidden_input: SLIDER_HIDDEN_INPUT,
    marker_group: SLIDER_MARKER_GROUP,
    marker: SLIDER_MARKER,
    orientation_default: SliderOrientation::Horizontal,
};

pub const fn slider_recipe() -> &'static SliderRecipe {
    &SLIDER_RECIPE
}

impl crate::components::_proa::variant_spec::ComponentSpec for SliderRecipe {
    const NAME: &'static str = "Slider";

    fn base_classes(&self) -> &'static str {
        self.root
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        vec![crate::components::_proa::variant_spec::variant_metadata::<SliderOrientation>()]
    }

    fn anatomy(&self) -> Option<crate::components::_proa::Anatomy> {
        Some(super::SLIDER_ANATOMY)
    }
}
