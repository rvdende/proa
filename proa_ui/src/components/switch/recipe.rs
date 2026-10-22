// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Switch recipe — typed root sizing and a named thumb slot.
//!
//! The root resolves size once and publishes the thumb dimensions as CSS
//! variables. The thumb consumes those variables, keeping the root as the
//! single source of truth for the compound component's appearance.

use crate::tw_join;
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

/// Size variants for the switch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, proa_macros::ProaVariant)]
#[variant(name = "size", path = "crate::components::_proa::variant_spec")]
pub enum SwitchSize {
    /// Small switch.
    Sm,
    /// Default switch size.
    #[default]
    Default,
}

impl SwitchSize {
    /// Root sizing plus variables inherited by the thumb slot.
    pub const fn classes(self) -> &'static str {
        match self {
            Self::Sm => tw_join!(
                "h-4",
                "w-7",
                "[--switch-thumb-size:0.75rem]",
                "[--switch-thumb-translate:0.75rem]"
            ),
            Self::Default => tw_join!(
                "h-5",
                "w-9",
                "[--switch-thumb-size:1rem]",
                "[--switch-thumb-translate:1rem]"
            ),
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Default => "default",
        }
    }
}

mod base {
    use super::*;

    pub const SWITCH_ROOT: &str = tw_join!(
        "peer",
        "inline-flex",
        "shrink-0",
        "cursor-pointer",
        "items-center",
        "rounded-full",
        "border-2",
        "border-transparent",
        "shadow-xs",
        "transition-colors",
        "focus-visible:outline-hidden",
        "focus-visible:ring-2",
        "focus-visible:ring-ring",
        "focus-visible:ring-offset-2",
        "focus-visible:ring-offset-background",
        "disabled:cursor-not-allowed",
        "disabled:opacity-50",
        "data-[state=checked]:bg-primary",
        "data-[state=unchecked]:bg-input"
    );

    pub const SWITCH_THUMB: &str = tw_join!(
        "pointer-events-none",
        "block",
        "size-[var(--switch-thumb-size)]",
        "rounded-full",
        "bg-background",
        "shadow-lg",
        "ring-0",
        "transition-transform",
        "data-[state=unchecked]:translate-x-0",
        "data-[state=checked]:translate-x-[var(--switch-thumb-translate)]"
    );

    pub const SWITCH_THUMB_SM_COMPAT: &str = tw_join!(
        "pointer-events-none",
        "block",
        "size-3",
        "rounded-full",
        "bg-background",
        "shadow-lg",
        "ring-0",
        "transition-transform",
        "data-[state=unchecked]:translate-x-0",
        "data-[state=checked]:translate-x-3"
    );

    pub const SWITCH_THUMB_DEFAULT_COMPAT: &str = tw_join!(
        "pointer-events-none",
        "block",
        "size-4",
        "rounded-full",
        "bg-background",
        "shadow-lg",
        "ring-0",
        "transition-transform",
        "data-[state=unchecked]:translate-x-0",
        "data-[state=checked]:translate-x-4"
    );
}

/// Optional inputs accepted by [`SwitchRecipe::resolve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SwitchRecipeProps {
    pub size: Option<SwitchSize>,
}

/// Static named slots for the Switch compound component.
#[derive(Debug, Clone, Copy)]
pub struct SwitchRecipe {
    pub root: &'static str,
    pub thumb: &'static str,
    size_default: SwitchSize,
}

impl SwitchRecipe {
    pub const fn resolve(&'static self, props: SwitchRecipeProps) -> SwitchStyle {
        let size = match props.size {
            Some(size) => size,
            None => self.size_default,
        };

        SwitchStyle { recipe: self, size }
    }
}

/// Fully resolved, streamable Switch root style.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct SwitchStyle {
    recipe: &'static SwitchRecipe,
    size: SwitchSize,
}

impl SwitchStyle {
    pub const fn size(self) -> SwitchSize {
        self.size
    }
}

impl AttrValue for SwitchStyle {
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
        cx.out.extend_static(self.size.classes().as_bytes())
    }
}

/// Global, zero-initialization Switch recipe.
pub const SWITCH_RECIPE: SwitchRecipe = SwitchRecipe {
    root: base::SWITCH_ROOT,
    thumb: base::SWITCH_THUMB,
    size_default: SwitchSize::Default,
};

pub const fn switch_recipe() -> &'static SwitchRecipe {
    &SWITCH_RECIPE
}

/// Compatibility resolver for the former track helper.
pub const fn switch_track_classes(size: SwitchSize) -> SwitchStyle {
    SWITCH_RECIPE.resolve(SwitchRecipeProps { size: Some(size) })
}

/// Compatibility accessor for the former independently sized thumb helper.
///
/// New render code should use [`SWITCH_RECIPE`]'s `thumb` slot, which inherits
/// the size resolved by the nearest Switch root.
pub const fn switch_thumb_classes(size: SwitchSize) -> &'static str {
    match size {
        SwitchSize::Sm => base::SWITCH_THUMB_SM_COMPAT,
        SwitchSize::Default => base::SWITCH_THUMB_DEFAULT_COMPAT,
    }
}

impl crate::components::_proa::variant_spec::ComponentSpec for SwitchRecipe {
    const NAME: &'static str = "Switch";

    fn base_classes(&self) -> &'static str {
        self.root
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        vec![crate::components::_proa::variant_spec::variant_metadata::<SwitchSize>()]
    }

    fn anatomy(&self) -> Option<crate::components::_proa::Anatomy> {
        Some(super::SWITCH_ANATOMY)
    }
}
