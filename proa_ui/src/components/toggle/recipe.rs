// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Toggle recipe - styling system for toggle component
//!
//! Provides Tailwind classes for toggle styling following shadcn patterns.

use crate::tw_join;
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

/// Toggle variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, proa_macros::ProaVariant)]
#[variant(name = "variant", path = "crate::components::_proa::variant_spec")]
pub enum ToggleVariant {
    /// Default transparent toggle
    #[default]
    Default,
    /// Outlined toggle with border
    Outline,
}

impl ToggleVariant {
    pub const fn as_str(self) -> &'static str {
        match self {
            ToggleVariant::Default => "default",
            ToggleVariant::Outline => "outline",
        }
    }

    pub const fn classes(self) -> &'static str {
        toggle_variant_classes(self)
    }
}

/// Toggle size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, proa_macros::ProaVariant)]
#[variant(name = "size", path = "crate::components::_proa::variant_spec")]
pub enum ToggleSize {
    /// Small toggle
    Sm,
    /// Default medium toggle
    #[default]
    Default,
    /// Large toggle
    Lg,
}

impl ToggleSize {
    pub const fn as_str(self) -> &'static str {
        match self {
            ToggleSize::Sm => "sm",
            ToggleSize::Default => "default",
            ToggleSize::Lg => "lg",
        }
    }

    pub const fn classes(self) -> &'static str {
        toggle_size_classes(self)
    }
}

/// Base toggle styles
pub const TOGGLE_BASE: &str = tw_join!(
    "inline-flex",
    "items-center",
    "justify-center",
    "gap-2",
    "rounded-md",
    "text-sm",
    "font-medium",
    "transition-colors",
    "hover:bg-muted",
    "hover:text-muted-foreground",
    "disabled:pointer-events-none",
    "disabled:opacity-50",
    "data-[state=on]:bg-accent",
    "data-[state=on]:text-accent-foreground",
    "focus-visible:outline-hidden",
    "focus-visible:ring-2",
    "focus-visible:ring-ring",
    "focus-visible:ring-offset-2",
    "ring-offset-background"
);

/// Default variant classes
pub const TOGGLE_VARIANT_DEFAULT: &str = "bg-transparent";

/// Outline variant classes
pub const TOGGLE_VARIANT_OUTLINE: &str = tw_join!(
    "border",
    "border-input",
    "bg-transparent",
    "shadow-sm",
    "hover:bg-accent",
    "hover:text-accent-foreground"
);

/// Small size classes
pub const TOGGLE_SIZE_SM: &str = tw_join!("h-8", "px-1.5", "min-w-8");

/// Default size classes
pub const TOGGLE_SIZE_DEFAULT: &str = tw_join!("h-9", "px-2", "min-w-9");

/// Large size classes
pub const TOGGLE_SIZE_LG: &str = tw_join!("h-10", "px-2.5", "min-w-10");

/// Get toggle base classes
pub const fn toggle_base_classes() -> &'static str {
    TOGGLE_BASE
}

/// Get variant classes
pub const fn toggle_variant_classes(variant: ToggleVariant) -> &'static str {
    match variant {
        ToggleVariant::Default => TOGGLE_VARIANT_DEFAULT,
        ToggleVariant::Outline => TOGGLE_VARIANT_OUTLINE,
    }
}

/// Get size classes
pub const fn toggle_size_classes(size: ToggleSize) -> &'static str {
    match size {
        ToggleSize::Sm => TOGGLE_SIZE_SM,
        ToggleSize::Default => TOGGLE_SIZE_DEFAULT,
        ToggleSize::Lg => TOGGLE_SIZE_LG,
    }
}

/// Optional inputs accepted by [`ToggleRecipe::resolve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ToggleRecipeProps {
    pub variant: Option<ToggleVariant>,
    pub size: Option<ToggleSize>,
}

/// Compile-time Toggle recipe.
#[derive(Debug, Clone, Copy)]
pub struct ToggleRecipe {
    root: &'static str,
    variant_default: ToggleVariant,
    size_default: ToggleSize,
}

impl ToggleRecipe {
    pub const fn resolve(&'static self, props: ToggleRecipeProps) -> ToggleStyle {
        let variant = match props.variant {
            Some(variant) => variant,
            None => self.variant_default,
        };
        let size = match props.size {
            Some(size) => size,
            None => self.size_default,
        };

        ToggleStyle {
            recipe: self,
            variant,
            size,
        }
    }
}

/// Fully resolved, streamable Toggle style.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct ToggleStyle {
    recipe: &'static ToggleRecipe,
    variant: ToggleVariant,
    size: ToggleSize,
}

impl ToggleStyle {
    pub const fn variant(self) -> ToggleVariant {
        self.variant
    }

    pub const fn size(self) -> ToggleSize {
        self.size
    }
}

impl AttrValue for ToggleStyle {
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
        cx.out.extend_static(self.variant.classes().as_bytes())?;
        cx.out.extend_static(b" ")?;
        cx.out.extend_static(self.size.classes().as_bytes())
    }
}

pub const TOGGLE_RECIPE: ToggleRecipe = ToggleRecipe {
    root: toggle_base_classes(),
    variant_default: ToggleVariant::Default,
    size_default: ToggleSize::Default,
};

pub const fn toggle_recipe() -> &'static ToggleRecipe {
    &TOGGLE_RECIPE
}

impl crate::components::_proa::variant_spec::ComponentSpec for ToggleRecipe {
    const NAME: &'static str = "Toggle";

    fn base_classes(&self) -> &'static str {
        self.root
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        vec![
            crate::components::_proa::variant_spec::variant_metadata::<ToggleVariant>(),
            crate::components::_proa::variant_spec::variant_metadata::<ToggleSize>(),
        ]
    }
}
