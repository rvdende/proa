// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Alert recipe - type-safe styling system for Alert component
//!
//! Provides variant styles and base styling for the Alert component.

use crate::tw_join;
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

/// Base styles for the alert
mod base {
    use super::*;

    pub const ALERT_BASE: &str = tw_join!(
        // Layout
        "relative",
        "w-full",
        // Shape
        "rounded-lg",
        "border",
        // Spacing
        "px-4",
        "py-3",
        // Typography
        "text-sm",
        // SVG styling
        "[&>svg]:size-4",
        "[&>svg]:text-current"
    );
}

/// Variant styles for the alert
mod variants {
    use super::*;

    pub const ALERT_DEFAULT: &str = tw_join!("bg-card", "text-card-foreground");

    pub const ALERT_DESTRUCTIVE: &str = tw_join!(
        "text-destructive",
        "bg-destructive/5",
        "border-destructive/20"
    );
}

/// Variant options for the alert
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, proa_macros::ProaVariant)]
#[variant(name = "variant", path = "crate::components::_proa::variant_spec")]
pub enum AlertVariant {
    /// Default alert style
    #[default]
    Default,
    /// Destructive/error alert style
    Destructive,
}

impl AlertVariant {
    /// Get the Tailwind classes for this variant
    pub const fn classes(self) -> &'static str {
        match self {
            AlertVariant::Default => variants::ALERT_DEFAULT,
            AlertVariant::Destructive => variants::ALERT_DESTRUCTIVE,
        }
    }

    /// Get the data attribute value
    pub const fn as_str(self) -> &'static str {
        match self {
            AlertVariant::Default => "default",
            AlertVariant::Destructive => "destructive",
        }
    }
}

/// Optional root inputs accepted by [`AlertRecipe::resolve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AlertRecipeProps {
    pub variant: Option<AlertVariant>,
}

/// Static named classes for every Alert slot.
#[derive(Debug, Clone, Copy)]
pub struct AlertRecipe {
    pub root: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    variant_default: AlertVariant,
}

impl AlertRecipe {
    pub const fn resolve(&'static self, props: AlertRecipeProps) -> AlertStyle {
        let variant = match props.variant {
            Some(variant) => variant,
            None => self.variant_default,
        };
        AlertStyle {
            recipe: self,
            variant,
        }
    }
}

/// Fully resolved, streamable Alert root style.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct AlertStyle {
    recipe: &'static AlertRecipe,
    variant: AlertVariant,
}

impl AlertStyle {
    pub const fn variant(self) -> AlertVariant {
        self.variant
    }
}

impl AttrValue for AlertStyle {
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
        cx.out.extend_static(self.variant.classes().as_bytes())
    }
}

/// AlertTitle slot classes.
pub const ALERT_TITLE_CLASSES: &str =
    tw_join!("mb-1", "font-medium", "leading-none", "tracking-tight");

/// AlertDescription slot classes.
pub const ALERT_DESCRIPTION_CLASSES: &str = tw_join!("text-sm", "[&_p]:leading-relaxed");

/// Global alert recipe instance.
pub const ALERT_RECIPE: AlertRecipe = AlertRecipe {
    root: base::ALERT_BASE,
    title: ALERT_TITLE_CLASSES,
    description: ALERT_DESCRIPTION_CLASSES,
    variant_default: AlertVariant::Default,
};

/// Get the global alert recipe
pub const fn alert_recipe() -> &'static AlertRecipe {
    &ALERT_RECIPE
}

/// Compatibility resolver for callers of the former helper.
pub const fn alert_classes(variant: AlertVariant) -> AlertStyle {
    ALERT_RECIPE.resolve(AlertRecipeProps {
        variant: Some(variant),
    })
}

impl crate::components::_proa::variant_spec::ComponentSpec for AlertRecipe {
    const NAME: &'static str = "Alert";

    fn base_classes(&self) -> &'static str {
        self.root
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        vec![crate::components::_proa::variant_spec::variant_metadata::<AlertVariant>()]
    }
}
