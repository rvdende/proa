// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Accordion recipe — named slots for interactive and CSS-only accordions.
//!
//! Accordion behavior owns semantic state attributes. This recipe owns only
//! the static appearance of each rendered part, including the additional
//! slots used by the zero-JavaScript implementation.

use crate::tw_join;
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

/// Accordion type - controls how many items can be open at once.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AccordionType {
    /// Only one item can be open at a time.
    #[default]
    Single,
    /// Multiple items can be open simultaneously.
    Multiple,
}

impl AccordionType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Multiple => "multiple",
        }
    }
}

mod base {
    use super::*;

    pub const ROOT: &str = "w-full";
    pub const ITEM: &str = tw_join!("border-b", "border-border");
    pub const HEADING: &str = "flex";
    pub const TRIGGER: &str = tw_join!(
        "flex",
        "flex-1",
        "items-center",
        "justify-between",
        "py-4",
        "text-sm",
        "font-medium",
        "text-foreground",
        "transition-all",
        "hover:underline",
        "text-left",
        "[&[data-state=open]>svg]:rotate-180"
    );
    pub const CHEVRON: &str = "shrink-0 transition-transform duration-200";
    pub const CONTENT: &str = tw_join!(
        "overflow-hidden",
        "text-sm",
        "text-muted-foreground",
        "data-[state=closed]:animate-accordion-up",
        "data-[state=open]:animate-accordion-down"
    );
    pub const CONTENT_INNER: &str = tw_join!("pb-4", "pt-0");

    pub const CSS_ONLY_ITEM: &str = tw_join!("border-b", "border-border", "group/item", "relative");
    pub const CSS_ONLY_ITEM_DISABLED: &str = "pointer-events-none opacity-50";
    pub const CSS_ONLY_INPUT: &str = "peer absolute opacity-0 -z-10";
    pub const CSS_ONLY_TRIGGER: &str = tw_join!(
        "flex",
        "flex-1",
        "items-center",
        "justify-between",
        "py-4",
        "text-sm",
        "font-medium",
        "text-foreground",
        "transition-all",
        "hover:underline",
        "text-left",
        "cursor-pointer"
    );
    pub const CSS_ONLY_CHEVRON: &str =
        "shrink-0 transition-transform duration-200 peer-checked:group-[]/item:rotate-180";
    pub const CSS_ONLY_CONTENT: &str =
        "grid grid-rows-[0fr] transition-all duration-300 peer-checked:grid-rows-[1fr]";
    pub const CSS_ONLY_CONTENT_INNER: &str = "overflow-hidden";
    pub const CSS_ONLY_CONTENT_BODY: &str = "pb-4 pt-0 text-sm text-muted-foreground";
}

/// Optional inputs accepted by [`AccordionRecipe::resolve`].
///
/// Accordion type and open state are behavioral rather than visual axes, so
/// the current recipe has no optional appearance inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AccordionRecipeProps;

/// Static named classes for all Accordion parts.
#[derive(Debug, Clone, Copy)]
pub struct AccordionRecipe {
    pub root: &'static str,
    pub item: &'static str,
    pub heading: &'static str,
    pub trigger: &'static str,
    pub chevron: &'static str,
    pub content: &'static str,
    pub content_inner: &'static str,
    pub css_only_item: &'static str,
    pub css_only_item_disabled: &'static str,
    pub css_only_input: &'static str,
    pub css_only_trigger: &'static str,
    pub css_only_chevron: &'static str,
    pub css_only_content: &'static str,
    pub css_only_content_inner: &'static str,
    pub css_only_content_body: &'static str,
}

impl AccordionRecipe {
    pub const fn resolve(&'static self, _props: AccordionRecipeProps) -> AccordionStyle {
        AccordionStyle { recipe: self }
    }
}

/// Fully resolved, streamable Accordion root style.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct AccordionStyle {
    recipe: &'static AccordionRecipe,
}

impl AttrValue for AccordionStyle {
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

pub const ACCORDION_RECIPE: AccordionRecipe = AccordionRecipe {
    root: base::ROOT,
    item: base::ITEM,
    heading: base::HEADING,
    trigger: base::TRIGGER,
    chevron: base::CHEVRON,
    content: base::CONTENT,
    content_inner: base::CONTENT_INNER,
    css_only_item: base::CSS_ONLY_ITEM,
    css_only_item_disabled: base::CSS_ONLY_ITEM_DISABLED,
    css_only_input: base::CSS_ONLY_INPUT,
    css_only_trigger: base::CSS_ONLY_TRIGGER,
    css_only_chevron: base::CSS_ONLY_CHEVRON,
    css_only_content: base::CSS_ONLY_CONTENT,
    css_only_content_inner: base::CSS_ONLY_CONTENT_INNER,
    css_only_content_body: base::CSS_ONLY_CONTENT_BODY,
};

pub const fn accordion_recipe() -> &'static AccordionRecipe {
    &ACCORDION_RECIPE
}

impl crate::components::_proa::variant_spec::ComponentSpec for AccordionRecipe {
    const NAME: &'static str = "Accordion";

    fn base_classes(&self) -> &'static str {
        self.root
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        Vec::new()
    }

    fn anatomy(&self) -> Option<crate::components::_proa::Anatomy> {
        Some(super::ACCORDION_ANATOMY)
    }
}

/// Compatibility accessors for the former standalone helpers.
pub const fn accordion_item_classes() -> &'static str {
    ACCORDION_RECIPE.item
}

pub const fn accordion_trigger_classes() -> &'static str {
    ACCORDION_RECIPE.trigger
}

pub const fn accordion_content_classes() -> &'static str {
    ACCORDION_RECIPE.content
}

pub const fn accordion_content_inner_classes() -> &'static str {
    ACCORDION_RECIPE.content_inner
}
