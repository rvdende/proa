// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Label component - accessible form label
//!
//! A simple styled label component that renders as a native HTML `<label>` element.
//! Based on shadcn/ui's Label component.
mod md;
use crate::tw_join;
use proa_core::{ClassList, WebContext, WebRenderSync, WriteBuf};
use proa_macros::html_sync;
/// Base styles for the label - matches shadcn/ui exactly
/// "flex items-center gap-2 text-sm leading-none font-medium select-none
///  group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50
///  peer-disabled:cursor-not-allowed peer-disabled:opacity-50"
const LABEL_BASE: &str = tw_join!(
    "flex",
    "items-center",
    "gap-2",
    "text-sm",
    "leading-none",
    "font-medium",
    "text-foreground",
    "select-none",
    "group-data-[disabled=true]:pointer-events-none",
    "group-data-[disabled=true]:opacity-50",
    "peer-disabled:cursor-not-allowed",
    "peer-disabled:opacity-50"
);
/// Type-safe Label component.
///
/// Renders an accessible label associated with form controls.
/// Uses the native HTML `<label>` element for proper accessibility.
///
/// # Example
///
/// ```ignore
/// use crate::components::Label;
/// use proa_macros::html_sync;
///
/// // Basic usage with for attribute
/// html_sync! {
///     <div class="flex items-center gap-2">
///         <input type="checkbox" id="terms" />
///         {Label {
///             for_id: Some("terms"),
///             text: "Accept terms and conditions",
///             ..Label::new()
///         }}
///     </div>
/// }.render(cx);
///
/// // With custom class
/// html_sync! {
///     {Label {
///         for_id: Some("email"),
///         text: "Email address",
///         class: Some("text-red-500"),
///         ..Label::new()
///     }}
/// }.render(cx);
/// ```
pub struct Label {
    /// The id of the form element this label is for
    pub for_id: Option<&'static str>,
    /// The text content of the label
    pub text: &'static str,
    /// Additional CSS classes
    pub class: Option<&'static str>,
    /// Custom id for the label element
    pub id: Option<&'static str>,
}
impl Label {
    /// Create a new label with default settings
    pub const fn new() -> Self {
        Self {
            for_id: None,
            text: "",
            class: None,
            id: None,
        }
    }
    /// Create a new label with text
    pub const fn with_text(text: &'static str) -> Self {
        Self {
            for_id: None,
            text,
            class: None,
            id: None,
        }
    }
}
impl Default for Label {
    fn default() -> Self {
        Self::new()
    }
}
/// Get the label classes
#[inline(always)]
pub fn label_classes() -> ClassList<2> {
    ClassList::new().add(LABEL_BASE)
}
impl<B: WriteBuf, L: ::proa_core::DataLoader> WebRenderSync<L, B> for Label {
    #[inline(always)]
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let classes = label_classes().add_opt(self.class);
        html_sync! {
            <label data-label class={classes} for={self.for_id} id={self.id} data-slot="label">
                {self.text}
            </label>
        }
        .render(cx)
    }
}
pub use Label as LabelComponent;
