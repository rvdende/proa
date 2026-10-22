// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Radio Group recipe — typed root axes and named compound slots.

use crate::tw_join;
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

/// Size of each radio item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, proa_macros::ProaVariant)]
#[variant(name = "size", path = "crate::components::_proa::variant_spec")]
pub enum RadioGroupSize {
    Sm,
    #[default]
    Default,
    Lg,
}

impl RadioGroupSize {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Default => "default",
            Self::Lg => "lg",
        }
    }

    /// Compatibility size utility for callers of the pre-recipe API.
    pub const fn classes(self) -> &'static str {
        match self {
            Self::Sm => "size-3.5",
            Self::Default => "size-4",
            Self::Lg => "size-5",
        }
    }

    /// A CSS variable lets a controlled group resolve size once at its root.
    const fn variable_classes(self) -> &'static str {
        match self {
            Self::Sm => "[--radio-group-item-size:0.875rem]",
            Self::Default => "[--radio-group-item-size:1rem]",
            Self::Lg => "[--radio-group-item-size:1.25rem]",
        }
    }
}

/// Orientation of the radio group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, proa_macros::ProaVariant)]
#[variant(name = "orientation", path = "crate::components::_proa::variant_spec")]
pub enum RadioGroupOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl RadioGroupOrientation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }

    pub const fn classes(self) -> &'static str {
        match self {
            Self::Vertical => RADIO_GROUP_VERTICAL,
            Self::Horizontal => RADIO_GROUP_HORIZONTAL,
        }
    }
}

/// Optional author inputs accepted by [`RadioGroupRecipe::resolve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RadioGroupRecipeProps {
    pub orientation: Option<RadioGroupOrientation>,
    pub size: Option<RadioGroupSize>,
}

/// Named classes for the Radio Group root, item, and controlled option.
#[derive(Debug, Clone, Copy)]
pub struct RadioGroupRecipe {
    pub root: &'static str,
    pub item: &'static str,
    pub option: &'static str,
    orientation_default: RadioGroupOrientation,
    size_default: RadioGroupSize,
}

impl RadioGroupRecipe {
    pub const fn resolve(&self, props: RadioGroupRecipeProps) -> RadioGroupStyle {
        let orientation = match props.orientation {
            Some(orientation) => orientation,
            None => self.orientation_default,
        };
        let size = match props.size {
            Some(size) => size,
            None => self.size_default,
        };
        RadioGroupStyle {
            root: self.root,
            orientation,
            size,
        }
    }

    pub const fn resolve_item(&self, size: Option<RadioGroupSize>) -> RadioGroupItemStyle {
        let size = match size {
            Some(size) => size,
            None => self.size_default,
        };
        RadioGroupItemStyle {
            item: self.item,
            size,
        }
    }

    const fn legacy_group_classes(&self, orientation: RadioGroupOrientation) -> &'static str {
        orientation.classes()
    }
}

/// Fully resolved, zero-allocation Radio Group root style.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RadioGroupStyle {
    root: &'static str,
    orientation: RadioGroupOrientation,
    size: RadioGroupSize,
}

impl RadioGroupStyle {
    pub const fn orientation(self) -> RadioGroupOrientation {
        self.orientation
    }

    pub const fn size(self) -> RadioGroupSize {
        self.size
    }
}

impl AttrValue for RadioGroupStyle {
    #[inline(always)]
    fn should_render(&self) -> bool {
        true
    }

    #[inline(always)]
    fn render_attr_value<B: WriteBuf, L: DataLoader>(
        &self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), WriteError> {
        if !self.root.is_empty() {
            cx.out.extend_static(self.root.as_bytes())?;
            cx.out.extend_static(b" ")?;
        }
        cx.out
            .extend_static(self.orientation.classes().as_bytes())?;
        cx.out.extend_static(b" ")?;
        cx.out
            .extend_static(self.size.variable_classes().as_bytes())
    }
}

/// Fully resolved, zero-allocation standalone item style.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RadioGroupItemStyle {
    item: &'static str,
    size: RadioGroupSize,
}

impl RadioGroupItemStyle {
    pub const fn size(self) -> RadioGroupSize {
        self.size
    }
}

impl AttrValue for RadioGroupItemStyle {
    #[inline(always)]
    fn should_render(&self) -> bool {
        true
    }

    #[inline(always)]
    fn render_attr_value<B: WriteBuf, L: DataLoader>(
        &self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), WriteError> {
        cx.out.extend_static(self.item.as_bytes())?;
        cx.out.extend_static(b" ")?;
        cx.out
            .extend_static(self.size.variable_classes().as_bytes())
    }
}

pub const RADIO_GROUP_ROOT: &str = "";
pub const RADIO_GROUP_VERTICAL: &str = tw_join!("grid", "gap-3");
pub const RADIO_GROUP_HORIZONTAL: &str =
    tw_join!("flex", "flex-row", "flex-wrap", "items-center", "gap-4");
pub const RADIO_GROUP_ITEM: &str = tw_join!(
    "aspect-square",
    "size-[var(--radio-group-item-size)]",
    "shrink-0",
    "rounded-full",
    "border",
    "border-input",
    "shadow-xs",
    "appearance-none",
    "cursor-pointer",
    "focus-visible:outline-hidden",
    "focus-visible:ring-[3px]",
    "focus-visible:ring-ring/50",
    "focus-visible:border-ring",
    "disabled:cursor-not-allowed",
    "disabled:opacity-50",
    "aria-invalid:border-destructive",
    "aria-invalid:ring-destructive/20",
    "dark:aria-invalid:ring-destructive/40",
    "checked:border-primary",
    "checked:bg-primary",
    "bg-center",
    "bg-no-repeat"
);
pub const RADIO_GROUP_OPTION: &str = tw_join!("inline-flex", "items-center", "gap-2");

/// Global named-slot Radio Group recipe.
pub const RADIO_GROUP_RECIPE: RadioGroupRecipe = RadioGroupRecipe {
    root: RADIO_GROUP_ROOT,
    item: RADIO_GROUP_ITEM,
    option: RADIO_GROUP_OPTION,
    orientation_default: RadioGroupOrientation::Vertical,
    size_default: RadioGroupSize::Default,
};

pub const fn radio_group_recipe() -> &'static RadioGroupRecipe {
    &RADIO_GROUP_RECIPE
}

impl crate::components::_proa::variant_spec::ComponentSpec for RadioGroupRecipe {
    const NAME: &'static str = "RadioGroup";

    fn base_classes(&self) -> &'static str {
        self.root
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        vec![
            crate::components::_proa::variant_spec::variant_metadata::<RadioGroupOrientation>(),
            crate::components::_proa::variant_spec::variant_metadata::<RadioGroupSize>(),
        ]
    }

    fn anatomy(&self) -> Option<crate::components::_proa::Anatomy> {
        Some(super::RADIO_GROUP_ANATOMY)
    }
}

// Public compatibility helpers delegate to the canonical recipe.
pub const fn radio_group_classes(orientation: RadioGroupOrientation) -> &'static str {
    RADIO_GROUP_RECIPE.legacy_group_classes(orientation)
}

pub const fn radio_item_classes() -> &'static str {
    RADIO_GROUP_RECIPE.item
}

pub fn radio_item_classes_with_size(size: RadioGroupSize) -> proa_core::ClassList<3> {
    proa_core::ClassList::new()
        .add(RADIO_GROUP_RECIPE.item)
        .add(size.variable_classes())
}
