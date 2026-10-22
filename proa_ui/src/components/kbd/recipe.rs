// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Kbd recipe — static named slots for keys and key groups.

use crate::tw_join;
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

const KBD_ROOT: &str = tw_join!(
    "inline-flex",
    "h-5",
    "w-fit",
    "min-w-5",
    "items-center",
    "justify-center",
    "gap-1",
    "rounded-xs",
    "bg-muted",
    "px-1",
    "font-sans",
    "text-xs",
    "font-medium",
    "text-muted-foreground",
    "pointer-events-none",
    "select-none"
);
const KBD_GROUP: &str = tw_join!("inline-flex", "items-center", "gap-1");

/// Optional inputs accepted by [`KbdRecipe::resolve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct KbdRecipeProps;

/// Static named classes for Kbd and KbdGroup.
#[derive(Debug, Clone, Copy)]
pub struct KbdRecipe {
    pub root: &'static str,
    pub group: &'static str,
}

impl KbdRecipe {
    pub const fn resolve(&'static self, _props: KbdRecipeProps) -> KbdStyle {
        KbdStyle { recipe: self }
    }
}

/// Fully resolved, streamable Kbd root style.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct KbdStyle {
    recipe: &'static KbdRecipe,
}

impl AttrValue for KbdStyle {
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

pub const KBD_RECIPE: KbdRecipe = KbdRecipe {
    root: KBD_ROOT,
    group: KBD_GROUP,
};

pub const fn kbd_recipe() -> &'static KbdRecipe {
    &KBD_RECIPE
}

impl crate::components::_proa::variant_spec::ComponentSpec for KbdRecipe {
    const NAME: &'static str = "Kbd";

    fn base_classes(&self) -> &'static str {
        self.root
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        Vec::new()
    }
}

/// Compatibility accessors for the former standalone helpers.
pub const fn kbd_base_classes() -> &'static str {
    KBD_RECIPE.root
}

pub const fn kbd_group_classes() -> &'static str {
    KBD_RECIPE.group
}
