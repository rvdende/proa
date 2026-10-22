// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tabs recipe — typed root orientation and named slot classes.
//!
//! The root resolves orientation once. Descendant slots inherit its visual
//! direction through the semantic `group/tabs` root and never resolve an
//! independent orientation style.

use crate::tw_join;
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

mod base {
    use super::*;

    pub const TABS_ROOT: &str = "group/tabs";

    pub const TABS_LIST: &str = tw_join!(
        "inline-flex",
        "h-9",
        "items-center",
        "justify-center",
        "rounded-lg",
        "bg-muted",
        "p-1",
        "text-muted-foreground",
        "group-data-[orientation=vertical]/tabs:h-auto",
        "group-data-[orientation=vertical]/tabs:flex-col",
        "group-data-[orientation=vertical]/tabs:items-stretch",
        "group-data-[orientation=vertical]/tabs:justify-start"
    );

    pub const TABS_TRIGGER: &str = tw_join!(
        "inline-flex",
        "items-center",
        "justify-center",
        "whitespace-nowrap",
        "rounded-md",
        "px-3",
        "py-1",
        "text-sm",
        "font-medium",
        "ring-offset-background",
        "transition-all",
        "focus-visible:outline-hidden",
        "focus-visible:ring-2",
        "focus-visible:ring-ring",
        "focus-visible:ring-offset-2",
        "disabled:pointer-events-none",
        "disabled:opacity-50",
        "data-[state=active]:bg-background",
        "data-[state=active]:text-foreground",
        "data-[state=active]:shadow",
        "data-[active=true]:bg-background",
        "data-[active=true]:text-foreground",
        "data-[active=true]:shadow",
        "group-data-[orientation=vertical]/tabs:w-full",
        "group-data-[orientation=vertical]/tabs:justify-start"
    );

    pub const TABS_CONTENT: &str = tw_join!(
        "mt-2",
        "ring-offset-background",
        "focus-visible:outline-hidden",
        "focus-visible:ring-2",
        "focus-visible:ring-ring",
        "focus-visible:ring-offset-2",
        "group-data-[orientation=vertical]/tabs:mt-0",
        "group-data-[orientation=vertical]/tabs:min-w-0",
        "group-data-[orientation=vertical]/tabs:flex-1"
    );
}

/// Visual and keyboard orientation owned by the Tabs root.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, proa_macros::ProaVariant)]
#[variant(name = "orientation", path = "crate::components::_proa::variant_spec")]
pub enum TabsOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl TabsOrientation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    pub const fn classes(self) -> &'static str {
        match self {
            Self::Horizontal => "",
            Self::Vertical => tw_join!("flex", "w-full", "items-start", "gap-4"),
        }
    }
}

/// Optional author inputs accepted by [`TabsRecipe::resolve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TabsRecipeProps {
    pub orientation: Option<TabsOrientation>,
}

/// Static named classes for every Tabs slot.
#[derive(Debug, Clone, Copy)]
pub struct TabsRecipe {
    pub root: &'static str,
    pub list: &'static str,
    pub trigger: &'static str,
    pub content: &'static str,
    orientation_default: TabsOrientation,
}

impl TabsRecipe {
    /// Resolve the root's effective orientation without allocating class
    /// storage for it or any descendant.
    pub const fn resolve(&'static self, props: TabsRecipeProps) -> TabsStyle {
        let orientation = match props.orientation {
            Some(orientation) => orientation,
            None => self.orientation_default,
        };

        TabsStyle {
            recipe: self,
            orientation,
        }
    }
}

/// Fully resolved, streamable Tabs root style.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct TabsStyle {
    recipe: &'static TabsRecipe,
    orientation: TabsOrientation,
}

impl TabsStyle {
    pub const fn orientation(self) -> TabsOrientation {
        self.orientation
    }
}

impl AttrValue for TabsStyle {
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
        let orientation = self.orientation.classes();
        if !orientation.is_empty() {
            cx.out.extend_static(b" ")?;
            cx.out.extend_static(orientation.as_bytes())?;
        }
        Ok(())
    }
}

/// Global, zero-initialization named-slot Tabs recipe.
pub const TABS_RECIPE: TabsRecipe = TabsRecipe {
    root: base::TABS_ROOT,
    list: base::TABS_LIST,
    trigger: base::TABS_TRIGGER,
    content: base::TABS_CONTENT,
    orientation_default: TabsOrientation::Horizontal,
};

pub const fn tabs_recipe() -> &'static TabsRecipe {
    &TABS_RECIPE
}

impl crate::components::_proa::variant_spec::ComponentSpec for TabsRecipe {
    const NAME: &'static str = "Tabs";

    fn base_classes(&self) -> &'static str {
        self.root
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        vec![crate::components::_proa::variant_spec::variant_metadata::<TabsOrientation>()]
    }

    fn anatomy(&self) -> Option<crate::components::_proa::Anatomy> {
        Some(super::TABS_ANATOMY)
    }
}

/// Compatibility accessors for the former standalone slot helpers.
pub const fn tabs_list_classes() -> &'static str {
    TABS_RECIPE.list
}

pub const fn tabs_trigger_classes() -> &'static str {
    TABS_RECIPE.trigger
}

pub const fn tabs_content_classes() -> &'static str {
    TABS_RECIPE.content
}
