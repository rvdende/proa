// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Alert Dialog recipe — one const named-slot table for every public part.
//!
//! Static modal primitives are sourced from Dialog where the two components
//! are visually identical. Alert-specific action and cancel controls remain
//! local to this recipe.

use crate::components::dialog::{DialogSize, DIALOG_RECIPE, DIALOG_SIZE_MD};
use crate::tw_join;

const ALERT_DIALOG_ROOT: &str = "group/alert-dialog";
const ALERT_DIALOG_CONTENT: &str = tw_join!(DIALOG_RECIPE.content, DIALOG_SIZE_MD);

const ALERT_DIALOG_ACTION: &str = tw_join!(
    "inline-flex",
    "items-center",
    "justify-center",
    "gap-2",
    "whitespace-nowrap",
    "rounded-md",
    "text-sm",
    "font-medium",
    "transition-colors",
    "h-9",
    "px-4",
    "py-2",
    "bg-destructive",
    "text-destructive-foreground",
    "shadow-xs",
    "hover:bg-destructive/90",
    "focus-visible:outline-hidden",
    "focus-visible:ring-2",
    "focus-visible:ring-ring"
);

const ALERT_DIALOG_CANCEL: &str = tw_join!(
    "inline-flex",
    "items-center",
    "justify-center",
    "gap-2",
    "whitespace-nowrap",
    "rounded-md",
    "text-sm",
    "font-medium",
    "transition-colors",
    "h-9",
    "px-4",
    "py-2",
    "border",
    "border-input",
    "bg-background",
    "text-foreground",
    "shadow-xs",
    "hover:bg-accent",
    "hover:text-accent-foreground",
    "focus-visible:outline-hidden",
    "focus-visible:ring-2",
    "focus-visible:ring-ring"
);

/// Static named classes for every Alert Dialog part.
#[derive(Debug, Clone, Copy)]
pub struct AlertDialogRecipe {
    pub root: &'static str,
    pub trigger: &'static str,
    pub overlay: &'static str,
    pub content: &'static str,
    pub header: &'static str,
    pub footer: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub action: &'static str,
    pub cancel: &'static str,
}

/// Global zero-initialization Alert Dialog recipe.
pub const ALERT_DIALOG_RECIPE: AlertDialogRecipe = AlertDialogRecipe {
    root: ALERT_DIALOG_ROOT,
    trigger: DIALOG_RECIPE.trigger,
    overlay: DIALOG_RECIPE.overlay,
    content: ALERT_DIALOG_CONTENT,
    header: DIALOG_RECIPE.header,
    footer: DIALOG_RECIPE.footer,
    title: DIALOG_RECIPE.title,
    description: DIALOG_RECIPE.description,
    action: ALERT_DIALOG_ACTION,
    cancel: ALERT_DIALOG_CANCEL,
};

pub const fn alert_dialog_recipe() -> &'static AlertDialogRecipe {
    &ALERT_DIALOG_RECIPE
}

impl crate::components::_proa::variant_spec::ComponentSpec for AlertDialogRecipe {
    const NAME: &'static str = "AlertDialog";

    fn base_classes(&self) -> &'static str {
        self.root
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        Vec::new()
    }

    fn anatomy(&self) -> Option<crate::components::_proa::Anatomy> {
        Some(super::ALERT_DIALOG_ANATOMY)
    }
}

/// Compatibility helpers for the former standalone class API.
pub const fn alert_dialog_action_classes() -> &'static str {
    ALERT_DIALOG_RECIPE.action
}

pub const fn alert_dialog_cancel_classes() -> &'static str {
    ALERT_DIALOG_RECIPE.cancel
}

pub const fn alert_dialog_content_base_classes() -> &'static str {
    DIALOG_RECIPE.content
}

pub const fn alert_dialog_content_size_classes() -> &'static str {
    DialogSize::Md.classes()
}

pub const fn alert_dialog_overlay_classes() -> &'static str {
    ALERT_DIALOG_RECIPE.overlay
}

pub const fn alert_dialog_header_classes() -> &'static str {
    ALERT_DIALOG_RECIPE.header
}

pub const fn alert_dialog_footer_classes() -> &'static str {
    ALERT_DIALOG_RECIPE.footer
}

pub const fn alert_dialog_title_classes() -> &'static str {
    ALERT_DIALOG_RECIPE.title
}

pub const fn alert_dialog_description_classes() -> &'static str {
    ALERT_DIALOG_RECIPE.description
}
