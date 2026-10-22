// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::module_inception)]
//! Alert Dialog component - modal dialog for destructive action confirmation
//!
//! Built on top of the Dialog component with `role="alertdialog"` semantics.
//! Unlike a regular Dialog, an AlertDialog:
//! - Uses `role="alertdialog"` instead of `role="dialog"`
//! - Has explicit `AlertDialogAction` and `AlertDialogCancel` buttons
//! - Does not show a close (X) button
//! - Is typically non-dismissible via click-outside
//!
//! # Sub-components
//!
//! - `AlertDialog` - Root component (wraps `Dialog`)
//! - `AlertDialogTrigger` - Button that opens the alert dialog (wraps `DialogTrigger`)
//! - `AlertDialogContent` - The modal content with `role="alertdialog"`
//! - `AlertDialogHeader` - Container for title and description
//! - `AlertDialogTitle` - Accessible alert dialog title
//! - `AlertDialogDescription` - Description text explaining the action
//! - `AlertDialogFooter` - Container for action/cancel buttons
//! - `AlertDialogAction` - Destructive/primary action button
//! - `AlertDialogCancel` - Cancel/dismiss button
//!
//! # Example
//!
//! ```ignore
//! html! {
//!     <AlertDialog>
//!         <AlertDialogTrigger>"Delete Account"</AlertDialogTrigger>
//!         <AlertDialogContent>
//!             <AlertDialogHeader>
//!                 <AlertDialogTitle>"Are you absolutely sure?"</AlertDialogTitle>
//!                 <AlertDialogDescription>
//!                     "This action cannot be undone."
//!                 </AlertDialogDescription>
//!             </AlertDialogHeader>
//!             <AlertDialogFooter>
//!                 <AlertDialogCancel>"Cancel"</AlertDialogCancel>
//!                 <AlertDialogAction>"Continue"</AlertDialogAction>
//!             </AlertDialogFooter>
//!         </AlertDialogContent>
//!     </AlertDialog>
//! }
//! ```

use crate::components::_proa::Anatomy;

mod alert_dialog;
mod md;
mod recipe;

/// Static anatomy contract shared by SSR and the Dialog delegated runtime.
pub const ALERT_DIALOG_ANATOMY: Anatomy = Anatomy::new(
    "alert-dialog",
    &[
        "root",
        "trigger",
        "portal",
        "overlay",
        "content",
        "header",
        "title",
        "description",
        "footer",
        "action",
        "cancel",
    ],
);

pub use alert_dialog::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
pub use recipe::{
    alert_dialog_action_classes, alert_dialog_cancel_classes, alert_dialog_content_base_classes,
    alert_dialog_content_size_classes, alert_dialog_description_classes,
    alert_dialog_footer_classes, alert_dialog_header_classes, alert_dialog_overlay_classes,
    alert_dialog_recipe, alert_dialog_title_classes, AlertDialogRecipe, ALERT_DIALOG_RECIPE,
};
