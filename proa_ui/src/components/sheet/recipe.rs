// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Sheet recipe — typed content appearance and named compound-part slots.

use crate::tw_join;
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

/// Which edge the sheet slides in from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, proa_macros::ProaVariant)]
#[variant(name = "side", path = "crate::components::_proa::variant_spec")]
pub enum SheetSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

impl SheetSide {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }

    /// Compatibility alias for the historical `data-side` helper.
    pub const fn data_value(self) -> &'static str {
        self.as_str()
    }

    pub const fn classes(self) -> &'static str {
        match self {
            Self::Top => SHEET_SIDE_TOP,
            Self::Right => SHEET_SIDE_RIGHT,
            Self::Bottom => SHEET_SIDE_BOTTOM,
            Self::Left => SHEET_SIDE_LEFT,
        }
    }
}

/// Optional content appearance accepted by [`SheetRecipe::resolve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SheetRecipeProps {
    pub side: Option<SheetSide>,
}

/// Static classes for every Sheet compound part.
#[derive(Debug, Clone, Copy)]
pub struct SheetRecipe {
    pub root: &'static str,
    pub trigger: &'static str,
    pub overlay: &'static str,
    pub content: &'static str,
    pub header: &'static str,
    pub footer: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub close: &'static str,
    pub close_label: &'static str,
    side_default: SheetSide,
}

impl SheetRecipe {
    /// Resolve the content side through the single canonical recipe path.
    pub const fn resolve(&'static self, props: SheetRecipeProps) -> SheetStyle {
        let side = match props.side {
            Some(side) => side,
            None => self.side_default,
        };

        SheetStyle { recipe: self, side }
    }
}

/// Fully resolved, allocation-free Sheet content style.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct SheetStyle {
    recipe: &'static SheetRecipe,
    side: SheetSide,
}

impl SheetStyle {
    pub const fn side(self) -> SheetSide {
        self.side
    }
}

impl AttrValue for SheetStyle {
    #[inline(always)]
    fn should_render(&self) -> bool {
        true
    }

    #[inline(always)]
    fn render_attr_value<B: WriteBuf, L: DataLoader>(
        &self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), WriteError> {
        cx.out.extend_static(self.recipe.content.as_bytes())?;
        cx.out.extend_static(b" ")?;
        cx.out.extend_static(self.side.classes().as_bytes())
    }
}

const SHEET_ROOT: &str = "relative";

const SHEET_TRIGGER: &str = tw_join!(
    "inline-flex",
    "items-center",
    "justify-center",
    "gap-2",
    "whitespace-nowrap",
    "text-sm",
    "font-medium",
    "transition-all",
    "cursor-pointer",
    "rounded-md",
    "h-9",
    "px-4",
    "py-2",
    "border",
    "border-input",
    "bg-background",
    "text-foreground",
    "shadow-xs",
    "hover:bg-accent",
    "hover:text-accent-foreground"
);

const SHEET_OVERLAY: &str = tw_join!(
    "fixed",
    "inset-0",
    "z-50",
    "bg-black/50",
    "data-[state=open]:animate-in",
    "data-[state=closed]:animate-out",
    "data-[state=closed]:fade-out-0",
    "data-[state=open]:fade-in-0"
);

const SHEET_CONTENT: &str = tw_join!(
    "fixed",
    "z-50",
    "flex",
    "flex-col",
    "gap-4",
    "bg-background",
    "p-6",
    "shadow-lg"
);

const SHEET_SIDE_RIGHT: &str = tw_join!(
    "inset-y-0",
    "right-0",
    "h-full",
    "w-3/4",
    "border-l",
    "border-border",
    "sm:max-w-sm"
);

const SHEET_SIDE_LEFT: &str = tw_join!(
    "inset-y-0",
    "left-0",
    "h-full",
    "w-3/4",
    "border-r",
    "border-border",
    "sm:max-w-sm"
);

const SHEET_SIDE_TOP: &str = tw_join!("inset-x-0", "top-0", "border-b", "border-border");

const SHEET_SIDE_BOTTOM: &str = tw_join!("inset-x-0", "bottom-0", "border-t", "border-border");

const SHEET_HEADER: &str = tw_join!("flex", "flex-col", "gap-1.5");

const SHEET_FOOTER: &str = tw_join!("mt-auto", "flex", "flex-col", "gap-2");

const SHEET_TITLE: &str = tw_join!("text-lg", "font-semibold", "text-foreground");

const SHEET_DESCRIPTION: &str = tw_join!("text-sm", "text-muted-foreground");

const SHEET_CLOSE: &str = tw_join!(
    "absolute",
    "right-4",
    "top-4",
    "rounded-sm",
    "opacity-70",
    "ring-offset-background",
    "transition-opacity",
    "hover:opacity-100",
    "focus:outline-hidden",
    "focus:ring-2",
    "focus:ring-ring",
    "focus:ring-offset-2",
    "disabled:pointer-events-none"
);

/// Global, zero-initialization Sheet recipe.
pub const SHEET_RECIPE: SheetRecipe = SheetRecipe {
    root: SHEET_ROOT,
    trigger: SHEET_TRIGGER,
    overlay: SHEET_OVERLAY,
    content: SHEET_CONTENT,
    header: SHEET_HEADER,
    footer: SHEET_FOOTER,
    title: SHEET_TITLE,
    description: SHEET_DESCRIPTION,
    close: SHEET_CLOSE,
    close_label: "sr-only",
    side_default: SheetSide::Right,
};

pub const fn sheet_recipe() -> &'static SheetRecipe {
    &SHEET_RECIPE
}

/// Compatibility resolver for callers that need the content classes.
pub const fn sheet_content_classes(side: SheetSide) -> SheetStyle {
    SHEET_RECIPE.resolve(SheetRecipeProps { side: Some(side) })
}

pub const fn sheet_overlay_classes() -> &'static str {
    SHEET_RECIPE.overlay
}

pub const fn sheet_content_base() -> &'static str {
    SHEET_RECIPE.content
}

pub const fn sheet_header_classes() -> &'static str {
    SHEET_RECIPE.header
}

pub const fn sheet_footer_classes() -> &'static str {
    SHEET_RECIPE.footer
}

pub const fn sheet_title_classes() -> &'static str {
    SHEET_RECIPE.title
}

pub const fn sheet_description_classes() -> &'static str {
    SHEET_RECIPE.description
}

pub const fn sheet_close_classes() -> &'static str {
    SHEET_RECIPE.close
}

impl crate::components::_proa::variant_spec::ComponentSpec for SheetRecipe {
    const NAME: &'static str = "Sheet";

    fn base_classes(&self) -> &'static str {
        self.content
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        vec![crate::components::_proa::variant_spec::variant_metadata::<SheetSide>()]
    }

    fn anatomy(&self) -> Option<crate::components::_proa::Anatomy> {
        Some(super::SHEET_ANATOMY)
    }
}
