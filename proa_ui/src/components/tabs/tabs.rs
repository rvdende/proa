// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tabs component implementations
//!
//! SSR markup for tabs. The JavaScript hydration handles the
//! tab switching behavior and keyboard navigation.
use super::recipe::{TabsOrientation, TabsRecipeProps, TABS_RECIPE};
use super::TABS_ANATOMY;
use proa_core::{WebContext, WebRenderSync, WriteBuf};
use proa_macros::html_sync;
/// Tabs root component - manages the tabs state
///
/// # Example
///
/// ```ignore
/// use crate::components::{Tabs, TabsList, TabsTrigger, TabsContent};
/// use proa_macros::html_sync;
///
/// html_sync! {
///     {Tabs {
///         default_value: Some("account"),
///         children: Some(html_sync! {
///             {TabsList {
///                 children: Some(html_sync! {
///                     {TabsTrigger { value: "account", children: Some("Account"), ..Default::default() }}
///                     {TabsTrigger { value: "password", children: Some("Password"), ..Default::default() }}
///                 }),
///                 ..Default::default()
///             }}
///             {TabsContent { value: "account", children: Some("Account content"), ..Default::default() }}
///             {TabsContent { value: "password", children: Some("Password content"), ..Default::default() }}
///         }),
///         ..Tabs::new()
///     }}
/// }
/// ```
pub struct Tabs<C = ()> {
    /// Default selected tab value
    pub default_value: Option<&'static str>,
    /// Tabs orientation (horizontal or vertical)
    pub orientation: TabsOrientation,
    /// Activation mode: "automatic" (on focus) or "manual" (on click/enter)
    pub activation_mode: TabsActivationMode,
    /// Additional classes
    pub class: Option<&'static str>,
    /// Tab children (list + content panels)
    pub children: Option<C>,
}
/// Activation mode for tabs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TabsActivationMode {
    /// Tab activates on focus (arrow key navigation activates tab)
    #[default]
    Automatic,
    /// Tab activates only on explicit selection (Enter/Space/Click)
    Manual,
}
impl TabsActivationMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            TabsActivationMode::Automatic => "automatic",
            TabsActivationMode::Manual => "manual",
        }
    }
}
impl Tabs<()> {
    pub const fn new() -> Self {
        Self {
            default_value: None,
            orientation: TabsOrientation::Horizontal,
            activation_mode: TabsActivationMode::Automatic,
            class: None,
            children: None,
        }
    }
}
impl<C> Default for Tabs<C> {
    fn default() -> Self {
        Self {
            default_value: None,
            orientation: TabsOrientation::Horizontal,
            activation_mode: TabsActivationMode::Automatic,
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for Tabs<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = TABS_RECIPE.resolve(TabsRecipeProps {
            orientation: Some(self.orientation),
        });
        let orientation = style.orientation();
        html_sync! {
            <div
                data-tabs-root
                data-scope={TABS_ANATOMY.scope()}
                data-part="root"
                data-slot="tabs"
                data-orientation={orientation.as_str()}
                data-activation-mode={self.activation_mode.as_str()}
                data-value={self.default_value}
                class={proa_macros::text!("{} {}", style, self.class)}
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
/// TabsList - container for tab triggers
pub struct TabsList<C = ()> {
    /// Whether to loop focus at edges
    pub loop_focus: bool,
    /// Semantic orientation emitted on the tablist for assistive technology.
    ///
    /// Visual orientation is always inherited from the nearest [`Tabs`] root;
    /// this value never selects classes.
    pub orientation: Option<TabsOrientation>,
    /// Additional classes
    pub class: Option<&'static str>,
    /// Tab triggers
    pub children: Option<C>,
}
impl<C> Default for TabsList<C> {
    fn default() -> Self {
        Self {
            loop_focus: true,
            orientation: None,
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for TabsList<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let aria_orientation = self.orientation.unwrap_or_default();
        let loop_attr = if self.loop_focus { "true" } else { "false" };
        html_sync! {
            <div
                data-tabs-list
                data-scope={TABS_ANATOMY.scope()}
                data-part="list"
                data-slot="tabs-list"
                role="tablist"
                aria-orientation={aria_orientation.as_str()}
                data-loop={loop_attr}
                class={proa_macros::text!("{} {}", TABS_RECIPE.list, self.class)}
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
/// TabsTrigger - clickable tab header
pub struct TabsTrigger<C = ()> {
    /// Unique value for this tab (required)
    pub value: &'static str,
    /// Whether this tab is disabled
    pub disabled: bool,
    /// Additional classes
    pub class: Option<&'static str>,
    /// Trigger content (usually text)
    pub children: Option<C>,
}
impl<C> Default for TabsTrigger<C> {
    fn default() -> Self {
        Self {
            value: "",
            disabled: false,
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for TabsTrigger<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let disabled = if self.disabled { "true" } else { "false" };
        html_sync! {
            <button
                type="button"
                role="tab"
                data-tabs-trigger
                data-scope={TABS_ANATOMY.scope()}
                data-part="trigger"
                data-slot="tabs-trigger"
                data-value={self.value}
                data-state="inactive"
                data-disabled={disabled}
                disabled={self.disabled}
                aria-selected="false"
                tabindex="-1"
                class={proa_macros::text!("{} {}", TABS_RECIPE.trigger, self.class)}
            >
                {self.children}
            </button>
        }
        .render(cx)
    }
}
/// TabsContent - the content panel for a tab
pub struct TabsContent<C = ()> {
    /// Value that matches the trigger (required)
    pub value: &'static str,
    /// Force mount even when inactive
    pub force_mount: bool,
    /// Additional classes
    pub class: Option<&'static str>,
    /// Content to show when tab is active
    pub children: Option<C>,
}
impl<C> Default for TabsContent<C> {
    fn default() -> Self {
        Self {
            value: "",
            force_mount: false,
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for TabsContent<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <div
                data-tabs-content
                data-scope={TABS_ANATOMY.scope()}
                data-part="content"
                data-slot="tabs-content"
                data-value={self.value}
                data-state="inactive"
                data-force-mount={self.force_mount}
                role="tabpanel"
                hidden
                tabindex="0"
                class={proa_macros::text!("{} {}", TABS_RECIPE.content, self.class)}
            >
                {self.children}
            </div>
        }
        .render(cx)
    }
}
