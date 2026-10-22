// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Popover recipe — semantic root placement and named slot classes.

use crate::components::_proa::theme::rounded;
use crate::tw_join;
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

/// Popover placement options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, proa_macros::ProaVariant)]
#[variant(name = "side", path = "crate::components::_proa::variant_spec")]
pub enum PopoverSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

impl PopoverSide {
    pub const fn classes(self) -> &'static str {
        ""
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

/// Popover alignment relative to its trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, proa_macros::ProaVariant)]
#[variant(name = "align", path = "crate::components::_proa::variant_spec")]
pub enum PopoverAlign {
    #[default]
    Center,
    Start,
    End,
}

impl PopoverAlign {
    pub const fn classes(self) -> &'static str {
        ""
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Center => "center",
            Self::Start => "start",
            Self::End => "end",
        }
    }
}

/// Optional root inputs accepted by [`PopoverRecipe::resolve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PopoverRecipeProps {
    pub side: Option<PopoverSide>,
    pub align: Option<PopoverAlign>,
}

/// Static named classes for every Popover part.
#[derive(Debug, Clone, Copy)]
pub struct PopoverRecipe {
    pub root: &'static str,
    pub trigger: &'static str,
    pub content: &'static str,
    pub close: &'static str,
    side_default: PopoverSide,
    align_default: PopoverAlign,
}

impl PopoverRecipe {
    pub const fn resolve(&'static self, props: PopoverRecipeProps) -> PopoverStyle {
        let side = match props.side {
            Some(side) => side,
            None => self.side_default,
        };
        let align = match props.align {
            Some(align) => align,
            None => self.align_default,
        };

        PopoverStyle {
            recipe: self,
            side,
            align,
        }
    }
}

/// Effective root placement and streamable root classes.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct PopoverStyle {
    recipe: &'static PopoverRecipe,
    side: PopoverSide,
    align: PopoverAlign,
}

impl PopoverStyle {
    pub const fn side(self) -> PopoverSide {
        self.side
    }

    pub const fn align(self) -> PopoverAlign {
        self.align
    }
}

impl AttrValue for PopoverStyle {
    #[inline(always)]
    fn should_render(&self) -> bool {
        true
    }

    #[inline(always)]
    fn render_attr_value<B: WriteBuf, L: DataLoader>(
        &self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), WriteError> {
        cx.out.extend_static(self.recipe.root.as_bytes())
    }
}

const POPOVER_ROOT: &str = tw_join!("relative", "inline-block");

const POPOVER_TRIGGER: &str = tw_join!(
    "inline-flex",
    "items-center",
    "justify-center",
    "gap-2",
    "whitespace-nowrap",
    "rounded-md",
    "border",
    "border-input",
    "bg-background",
    "h-9",
    "px-4",
    "py-2",
    "text-sm",
    "font-medium",
    "text-foreground",
    "shadow-xs",
    "transition-all",
    "cursor-pointer",
    "hover:bg-accent",
    "hover:text-accent-foreground"
);

const POPOVER_CONTENT: &str = tw_join!(
    "hidden",
    "absolute",
    "z-50",
    "w-72",
    "border",
    "border-border",
    "bg-popover",
    "text-popover-foreground",
    "p-4",
    "shadow-md",
    "outline-hidden",
    rounded::MD
);

/// Close has no intrinsic visual treatment; it remains a named slot so
/// authors can target it consistently without the recipe owning markup.
const POPOVER_CLOSE: &str = "";

pub const POPOVER_RECIPE: PopoverRecipe = PopoverRecipe {
    root: POPOVER_ROOT,
    trigger: POPOVER_TRIGGER,
    content: POPOVER_CONTENT,
    close: POPOVER_CLOSE,
    side_default: PopoverSide::Bottom,
    align_default: PopoverAlign::Center,
};

pub const fn popover_recipe() -> &'static PopoverRecipe {
    &POPOVER_RECIPE
}

impl crate::components::_proa::variant_spec::ComponentSpec for PopoverRecipe {
    const NAME: &'static str = "Popover";

    fn base_classes(&self) -> &'static str {
        self.root
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        vec![
            crate::components::_proa::variant_spec::variant_metadata::<PopoverSide>(),
            crate::components::_proa::variant_spec::variant_metadata::<PopoverAlign>(),
        ]
    }

    fn anatomy(&self) -> Option<crate::components::_proa::Anatomy> {
        Some(super::POPOVER_ANATOMY)
    }
}
