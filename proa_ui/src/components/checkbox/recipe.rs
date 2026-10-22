// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT AND (ISC AND MIT)

//! Checkbox recipe - type-safe styling system for Checkbox component
//!
//! Provides size variants, color variants, icon variants, and base styling for the Checkbox component.

use crate::tw_join;
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

/// Base styles for the checkbox (without color-specific checked state styling)
mod base {
    use super::*;

    pub const CHECKBOX_BASE: &str = tw_join!(
        // Layout - peer for label styling
        "peer",
        // Size and shape
        "size-4",
        "shrink-0",
        "rounded-[4px]",
        // Border
        "border border-input",
        // Background
        "bg-background",
        // Shadow
        "shadow-xs",
        // Focus ring (shadcn style)
        "focus-visible:outline-hidden",
        "focus-visible:ring-[3px]",
        "focus-visible:ring-ring/50",
        "focus-visible:border-ring",
        // Disabled state
        "disabled:cursor-not-allowed",
        "disabled:opacity-50",
        // Aria invalid (error state)
        "aria-invalid:border-destructive",
        "aria-invalid:ring-destructive/20",
        "dark:aria-invalid:ring-destructive/40",
        // Appearance - hide default checkbox styling
        "appearance-none",
        // Cursor
        "cursor-pointer",
        // Background sizing for icons
        "bg-[length:100%_100%]",
        "bg-center",
        "bg-no-repeat"
    );
}

/// Color variants for the checkbox
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, proa_macros::ProaVariant)]
#[variant(name = "color", path = "crate::components::_proa::variant_spec")]
pub enum CheckboxColor {
    /// Primary color (default) - typically blue/indigo
    #[default]
    Primary,
    /// Destructive/danger color - typically red
    Destructive,
    /// Success color - typically green
    Success,
}

impl CheckboxColor {
    /// Get the Tailwind classes for this color's checked state
    pub const fn classes(self) -> &'static str {
        match self {
            CheckboxColor::Primary => "checked:bg-primary checked:border-primary",
            CheckboxColor::Destructive => "checked:bg-destructive checked:border-destructive",
            CheckboxColor::Success => "checked:bg-green-600 checked:border-green-600",
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Destructive => "destructive",
            Self::Success => "success",
        }
    }
}

/// Icon variants for the checkbox checkmark
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CheckboxIcon {
    /// Standard checkmark (default)
    #[default]
    Check,
    /// X/Cross mark
    Cross,
    /// Minus/dash (typically used for indeterminate, but can be used as checked icon)
    Minus,
    /// Circle/dot
    Dot,
}

impl CheckboxIcon {
    /// Get the CSS style string to set the --checkbox-icon custom property.
    /// Returns a style like: --checkbox-icon: url('data:...');
    /// Note: SVGs use black stroke/fill since they're used as mask-image,
    /// and the actual color is controlled by --checkbox-icon-color CSS variable.
    pub const fn style(self) -> Option<&'static str> {
        match self {
            // Default check icon - no style needed, CSS fallback handles it
            CheckboxIcon::Check => None,
            // Non-default icons need explicit style (use black for mask)
            CheckboxIcon::Cross => Some("--checkbox-icon: url('data:image/svg+xml,%3Csvg xmlns=%27http://www.w3.org/2000/svg%27 viewBox=%270 0 24 24%27 fill=%27none%27 stroke=%27black%27 stroke-width=%273%27 stroke-linecap=%27round%27 stroke-linejoin=%27round%27%3E%3Cpath d=%27M18 6 6 18%27/%3E%3Cpath d=%27M6 6 18 18%27/%3E%3C/svg%3E')"),
            CheckboxIcon::Minus => Some("--checkbox-icon: url('data:image/svg+xml,%3Csvg xmlns=%27http://www.w3.org/2000/svg%27 viewBox=%270 0 24 24%27 fill=%27none%27 stroke=%27black%27 stroke-width=%273%27 stroke-linecap=%27round%27 stroke-linejoin=%27round%27%3E%3Cpath d=%27M5 12h14%27/%3E%3C/svg%3E')"),
            CheckboxIcon::Dot => Some("--checkbox-icon: url('data:image/svg+xml,%3Csvg xmlns=%27http://www.w3.org/2000/svg%27 viewBox=%270 0 24 24%27%3E%3Ccircle cx=%2712%27 cy=%2712%27 r=%275%27 fill=%27black%27/%3E%3C/svg%3E')"),
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Check => "check",
            Self::Cross => "cross",
            Self::Minus => "minus",
            Self::Dot => "dot",
        }
    }
}

/// Size variants for the checkbox
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, proa_macros::ProaVariant)]
#[variant(name = "size", path = "crate::components::_proa::variant_spec")]
pub enum CheckboxSize {
    Sm,
    #[default]
    Default,
    Lg,
}

impl CheckboxSize {
    /// Get the Tailwind classes for this size
    pub const fn classes(self) -> &'static str {
        match self {
            CheckboxSize::Sm => "size-3.5",
            CheckboxSize::Default => "size-4",
            CheckboxSize::Lg => "size-5",
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Default => "default",
            Self::Lg => "lg",
        }
    }
}

/// Optional inputs accepted by [`CheckboxRecipe::resolve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CheckboxRecipeProps {
    pub size: Option<CheckboxSize>,
    pub color: Option<CheckboxColor>,
}

/// Fully typed checkbox recipe with compile-time defaults.
#[derive(Debug, Clone, Copy)]
pub struct CheckboxRecipe {
    base: &'static str,
    /// Wrapper used by the rsjs-controlled checkbox surface.
    pub controlled_root: &'static str,
    size_default: CheckboxSize,
    color_default: CheckboxColor,
}

pub const CHECKBOX_CONTROLLED_ROOT: &str = "inline-flex items-center gap-2";

/// Global checkbox recipe instance
pub const CHECKBOX_RECIPE: CheckboxRecipe = CheckboxRecipe {
    base: base::CHECKBOX_BASE,
    controlled_root: CHECKBOX_CONTROLLED_ROOT,
    size_default: CheckboxSize::Default,
    color_default: CheckboxColor::Primary,
};

impl CheckboxRecipe {
    pub const fn resolve(&'static self, props: CheckboxRecipeProps) -> CheckboxStyle {
        let size = match props.size {
            Some(size) => size,
            None => self.size_default,
        };
        let color = match props.color {
            Some(color) => color,
            None => self.color_default,
        };
        CheckboxStyle {
            recipe: self,
            size,
            color,
        }
    }
}

/// Fully resolved, streamable Checkbox style.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct CheckboxStyle {
    recipe: &'static CheckboxRecipe,
    size: CheckboxSize,
    color: CheckboxColor,
}

impl CheckboxStyle {
    pub const fn size(self) -> CheckboxSize {
        self.size
    }

    pub const fn color(self) -> CheckboxColor {
        self.color
    }
}

impl AttrValue for CheckboxStyle {
    #[inline(always)]
    fn should_render(&self) -> bool {
        true
    }

    #[inline(always)]
    fn render_attr_value<B: WriteBuf, L: DataLoader>(
        &self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), WriteError> {
        cx.out.extend_static(self.recipe.base.as_bytes())?;
        cx.out.extend_static(b" ")?;
        cx.out.extend_static(self.size.classes().as_bytes())?;
        cx.out.extend_static(b" ")?;
        cx.out.extend_static(self.color.classes().as_bytes())
    }
}

/// Get the global checkbox recipe
pub const fn checkbox_recipe() -> &'static CheckboxRecipe {
    &CHECKBOX_RECIPE
}

/// Note: Icon variants are now handled via CSS custom properties (--checkbox-icon)
/// which are set via inline style in the component.
pub const fn checkbox_classes(size: CheckboxSize, color: CheckboxColor) -> CheckboxStyle {
    CHECKBOX_RECIPE.resolve(CheckboxRecipeProps {
        size: Some(size),
        color: Some(color),
    })
}

impl crate::components::_proa::variant_spec::ComponentSpec for CheckboxRecipe {
    const NAME: &'static str = "Checkbox";

    fn base_classes(&self) -> &'static str {
        self.base
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        vec![
            crate::components::_proa::variant_spec::variant_metadata::<CheckboxSize>(),
            crate::components::_proa::variant_spec::variant_metadata::<CheckboxColor>(),
        ]
    }

    fn anatomy(&self) -> Option<crate::components::_proa::Anatomy> {
        Some(super::CHECKBOX_ANATOMY)
    }
}
