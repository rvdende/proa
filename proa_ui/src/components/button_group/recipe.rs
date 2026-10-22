// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Button Group recipe — typed root appearance and named slot classes.
//!
//! The root resolves orientation once. Descendant slots read their static
//! classes directly from [`BUTTON_GROUP_RECIPE`], and the separator responds
//! to the root's semantic `data-orientation` through a named Tailwind group.

use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

mod base {
    use crate::tw_join;

    pub const BUTTON_GROUP_ROOT: &str = tw_join!(
        "group/button-group",
        "flex",
        "w-fit",
        "items-stretch",
        "[&>*]:focus-visible:z-10",
        "[&>*]:focus-visible:relative",
        "[&>[data-slot=select-trigger]:not([class*='w-'])]:w-fit",
        "[&>input]:flex-1",
        "has-[select[aria-hidden=true]:last-child]:[&>[data-slot=select-trigger]:last-of-type]:rounded-r-md",
        "has-[>[data-slot=button-group]]:gap-2"
    );

    pub const BUTTON_GROUP_TEXT: &str = tw_join!(
        "bg-muted",
        "text-muted-foreground",
        "flex",
        "items-center",
        "gap-2",
        "rounded-md",
        "border",
        "border-input",
        "px-3",
        "text-sm",
        "font-medium",
        "[&_svg]:pointer-events-none",
        "[&_svg:not([class*='size-'])]:size-4"
    );

    /// A separator is vertical in a horizontal group and horizontal in a
    /// vertical group. The named-group selectors make that relationship
    /// authoritative at the root instead of requiring every separator to
    /// repeat an orientation prop.
    pub const BUTTON_GROUP_SEPARATOR: &str = tw_join!(
        "bg-input",
        "relative",
        "!m-0",
        "self-stretch",
        "w-px",
        "h-auto",
        "group-data-[orientation=horizontal]/button-group:w-px",
        "group-data-[orientation=horizontal]/button-group:h-auto",
        "group-data-[orientation=vertical]/button-group:w-auto",
        "group-data-[orientation=vertical]/button-group:h-px"
    );
}

mod orientations {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, proa_macros::ProaVariant)]
    #[variant(name = "orientation", path = "crate::components::_proa::variant_spec")]
    pub enum ButtonGroupOrientation {
        #[default]
        Horizontal,
        Vertical,
    }

    impl ButtonGroupOrientation {
        pub const fn classes(self) -> &'static str {
            match self {
                Self::Horizontal => "[&>*:not(:first-child)]:rounded-l-none [&>*:not(:first-child)]:border-l-0 [&>*:not(:last-child)]:rounded-r-none",
                Self::Vertical => "flex-col [&>*:not(:first-child)]:rounded-t-none [&>*:not(:first-child)]:border-t-0 [&>*:not(:last-child)]:rounded-b-none",
            }
        }

        pub const fn data_value(self) -> &'static str {
            match self {
                Self::Horizontal => "horizontal",
                Self::Vertical => "vertical",
            }
        }
    }
}

pub use orientations::ButtonGroupOrientation;

/// Optional root inputs accepted by [`ButtonGroupRecipe::resolve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ButtonGroupRecipeProps {
    pub orientation: Option<ButtonGroupOrientation>,
}

/// Static named slots for the Button Group compound component.
#[derive(Debug, Clone, Copy)]
pub struct ButtonGroupRecipe {
    pub root: &'static str,
    pub text: &'static str,
    pub separator: &'static str,
    orientation_default: ButtonGroupOrientation,
}

impl ButtonGroupRecipe {
    /// Resolve the effective root orientation without allocating class storage.
    pub const fn resolve(
        &'static self,
        props: ButtonGroupRecipeProps,
    ) -> ResolvedButtonGroupRecipe {
        let orientation = match props.orientation {
            Some(orientation) => orientation,
            None => self.orientation_default,
        };

        ResolvedButtonGroupRecipe {
            recipe: self,
            orientation,
        }
    }
}

/// Fully resolved, streamable Button Group root style.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct ResolvedButtonGroupRecipe {
    recipe: &'static ButtonGroupRecipe,
    orientation: ButtonGroupOrientation,
}

impl ResolvedButtonGroupRecipe {
    pub const fn orientation(self) -> ButtonGroupOrientation {
        self.orientation
    }
}

impl AttrValue for ResolvedButtonGroupRecipe {
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

/// Global, zero-initialization Button Group recipe.
pub const BUTTON_GROUP_RECIPE: ButtonGroupRecipe = ButtonGroupRecipe {
    root: base::BUTTON_GROUP_ROOT,
    text: base::BUTTON_GROUP_TEXT,
    separator: base::BUTTON_GROUP_SEPARATOR,
    orientation_default: ButtonGroupOrientation::Horizontal,
};

pub const fn button_group_recipe() -> &'static ButtonGroupRecipe {
    &BUTTON_GROUP_RECIPE
}

/// Compatibility resolver for callers of the former helper.
///
/// New render code should call [`ButtonGroupRecipe::resolve`] directly.
pub const fn button_group_classes(
    orientation: ButtonGroupOrientation,
) -> ResolvedButtonGroupRecipe {
    BUTTON_GROUP_RECIPE.resolve(ButtonGroupRecipeProps {
        orientation: Some(orientation),
    })
}

/// Compatibility accessors for the former standalone class constants.
pub const fn button_group_text_classes() -> &'static str {
    BUTTON_GROUP_RECIPE.text
}

pub const fn button_group_separator_classes() -> &'static str {
    BUTTON_GROUP_RECIPE.separator
}

impl crate::components::_proa::variant_spec::ComponentSpec for ButtonGroupRecipe {
    const NAME: &'static str = "ButtonGroup";

    fn base_classes(&self) -> &'static str {
        self.root
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        vec![crate::components::_proa::variant_spec::variant_metadata::<
            ButtonGroupOrientation,
        >()]
    }
}
