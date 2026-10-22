// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Field recipe — typed root orientation and named compound slots.

use crate::tw_join;
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

/// Field layout orientation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, proa_macros::ProaVariant)]
#[variant(name = "orientation", path = "crate::components::_proa::variant_spec")]
pub enum FieldOrientation {
    #[default]
    Vertical,
    Horizontal,
    Responsive,
}

impl FieldOrientation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
            Self::Responsive => "responsive",
        }
    }

    pub const fn classes(self) -> &'static str {
        match self {
            Self::Vertical => FIELD_VERTICAL,
            Self::Horizontal => FIELD_HORIZONTAL,
            Self::Responsive => FIELD_RESPONSIVE,
        }
    }
}

/// FieldLegend text treatment.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum FieldLegendVariant {
    #[default]
    Legend,
    Label,
}

impl FieldLegendVariant {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Legend => "legend",
            Self::Label => "label",
        }
    }

    pub const fn classes(self) -> &'static str {
        match self {
            Self::Legend => FIELD_LEGEND_LEGEND,
            Self::Label => FIELD_LEGEND_LABEL,
        }
    }
}

/// Optional author inputs accepted by [`FieldRecipe::resolve`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FieldRecipeProps {
    pub orientation: Option<FieldOrientation>,
}

/// Named classes for every public and internal styled Field part.
#[derive(Debug, Clone, Copy)]
pub struct FieldRecipe {
    pub root: &'static str,
    pub fieldset: &'static str,
    pub legend: &'static str,
    pub group: &'static str,
    pub label: &'static str,
    pub content: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub error: &'static str,
    pub separator: &'static str,
    pub separator_rule: &'static str,
    pub separator_content: &'static str,
    pub separator_text: &'static str,
    orientation_default: FieldOrientation,
}

impl FieldRecipe {
    pub const fn resolve(&self, props: FieldRecipeProps) -> FieldStyle {
        let orientation = match props.orientation {
            Some(orientation) => orientation,
            None => self.orientation_default,
        };
        FieldStyle {
            root: self.root,
            orientation,
        }
    }
}

/// Fully resolved, zero-allocation Field root style.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldStyle {
    root: &'static str,
    orientation: FieldOrientation,
}

impl FieldStyle {
    pub const fn orientation(self) -> FieldOrientation {
        self.orientation
    }
}

impl AttrValue for FieldStyle {
    #[inline(always)]
    fn should_render(&self) -> bool {
        true
    }

    #[inline(always)]
    fn render_attr_value<B: WriteBuf, L: DataLoader>(
        &self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), WriteError> {
        cx.out.extend_static(self.root.as_bytes())?;
        cx.out.extend_static(b" ")?;
        cx.out.extend_static(self.orientation.classes().as_bytes())
    }
}

pub const FIELD_ROOT: &str = tw_join!(
    "group/field",
    "flex",
    "gap-2",
    "data-[invalid=true]:[&_[data-slot=field-error]]:block"
);
pub const FIELD_VERTICAL: &str = tw_join!("flex-col", "w-full");
pub const FIELD_HORIZONTAL: &str = tw_join!(
    "flex-row",
    "items-start",
    "[&>[data-slot=label]]:flex-1",
    "[&>[data-slot=label]]:pt-2"
);
pub const FIELD_RESPONSIVE: &str = tw_join!(
    "flex-col",
    "w-full",
    "@md/field-group:flex-row",
    "@md/field-group:items-start",
    "@md/field-group:[&>[data-slot=label]]:flex-1",
    "@md/field-group:[&>[data-slot=label]]:pt-2"
);

pub const FIELDSET: &str = tw_join!(
    "flex",
    "flex-col",
    "gap-6",
    "has-[[data-slot=field-group]]:gap-4"
);
pub const FIELD_LEGEND: &str = "text-foreground";
pub const FIELD_LEGEND_LEGEND: &str = tw_join!("text-base", "font-medium");
pub const FIELD_LEGEND_LABEL: &str = tw_join!("text-sm", "font-medium", "leading-none");
pub const FIELD_GROUP: &str = tw_join!(
    "@container/field-group",
    "flex",
    "flex-col",
    "gap-3",
    "has-[[data-slot=checkbox]]:gap-3",
    "has-[[data-slot=radio]]:gap-3"
);
pub const FIELD_LABEL: &str = tw_join!(
    "flex",
    "items-center",
    "gap-2",
    "text-sm",
    "leading-none",
    "font-medium",
    "text-foreground",
    "select-none",
    "data-[disabled=true]:pointer-events-none",
    "data-[disabled=true]:opacity-50",
    "data-[checked=true]:rounded-md",
    "data-[checked=true]:bg-accent",
    "data-[checked=true]:border",
    "data-[checked=true]:border-accent"
);
pub const FIELD_CONTENT: &str = tw_join!(
    "flex",
    "flex-col",
    "gap-2",
    "leading-5",
    "@md/field-group:leading-normal"
);
pub const FIELD_TITLE: &str = tw_join!(
    "text-sm",
    "font-medium",
    "leading-none",
    "text-foreground",
    "group-data-[disabled=true]/field:opacity-50"
);
pub const FIELD_DESCRIPTION: &str = tw_join!(
    "text-sm",
    "text-muted-foreground",
    "[&_a]:underline",
    "[&_a]:underline-offset-4",
    "[&_a]:hover:text-foreground",
    "@md/field-group:text-balance"
);
pub const FIELD_ERROR: &str = tw_join!(
    "hidden",
    "text-sm",
    "font-medium",
    "text-destructive",
    "[&_ul]:list-disc",
    "[&_ul]:list-inside"
);
pub const FIELD_SEPARATOR: &str = tw_join!("relative", "text-center", "text-sm");
pub const FIELD_SEPARATOR_RULE: &str = tw_join!("border-t", "border-border");
pub const FIELD_SEPARATOR_CONTENT: &str = tw_join!(
    "absolute",
    "inset-0",
    "flex",
    "items-center",
    "justify-center"
);
pub const FIELD_SEPARATOR_TEXT: &str = tw_join!("bg-background", "px-2", "text-muted-foreground");

/// Global named-slot Field recipe.
pub const FIELD_RECIPE: FieldRecipe = FieldRecipe {
    root: FIELD_ROOT,
    fieldset: FIELDSET,
    legend: FIELD_LEGEND,
    group: FIELD_GROUP,
    label: FIELD_LABEL,
    content: FIELD_CONTENT,
    title: FIELD_TITLE,
    description: FIELD_DESCRIPTION,
    error: FIELD_ERROR,
    separator: FIELD_SEPARATOR,
    separator_rule: FIELD_SEPARATOR_RULE,
    separator_content: FIELD_SEPARATOR_CONTENT,
    separator_text: FIELD_SEPARATOR_TEXT,
    orientation_default: FieldOrientation::Vertical,
};

pub const fn field_recipe() -> &'static FieldRecipe {
    &FIELD_RECIPE
}

impl crate::components::_proa::variant_spec::ComponentSpec for FieldRecipe {
    const NAME: &'static str = "Field";

    fn base_classes(&self) -> &'static str {
        self.root
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        vec![crate::components::_proa::variant_spec::variant_metadata::<FieldOrientation>()]
    }

    fn anatomy(&self) -> Option<crate::components::_proa::Anatomy> {
        Some(super::FIELD_ANATOMY)
    }
}
