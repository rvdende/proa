// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND (ISC AND MIT)

//! Breadcrumb component implementation
//!
//! Renders a navigation hierarchy with accessible breadcrumb markup.
use super::recipe::{BreadcrumbRecipeProps, BREADCRUMB_RECIPE};
use super::BREADCRUMB_ANATOMY;
use proa_core::{UnsafeRaw, WebContext, WebRenderSync, WriteBuf};
use proa_macros::html_sync;
/// ChevronRight SVG icon used as the default separator
const CHEVRON_RIGHT_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>"#;
/// MoreHorizontal SVG icon used for the ellipsis indicator
const MORE_HORIZONTAL_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/></svg>"#;
/// Breadcrumb component - `<nav>` root wrapper with `aria-label="breadcrumb"`
pub struct Breadcrumb<C = ()> {
    /// Additional CSS classes
    pub class: Option<&'static str>,
    /// The breadcrumb content
    pub children: Option<C>,
}
impl<C> Default for Breadcrumb<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for Breadcrumb<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = BREADCRUMB_RECIPE.resolve(BreadcrumbRecipeProps);
        html_sync! {
            <nav
                data-breadcrumb
                data-scope={BREADCRUMB_ANATOMY.scope()}
                data-part="root"
                aria-label={proa_macros::content_part!("browser-control-label", "breadcrumb")}
                data-slot="breadcrumb"
                class={proa_macros::text!("{} {}", style, self.class)}
            >
                {self.children}
            </nav>
        }
        .render(cx)
    }
}
/// BreadcrumbList component - `<ol>` ordered list container with flexbox layout
pub struct BreadcrumbList<C = ()> {
    /// Additional CSS classes
    pub class: Option<&'static str>,
    /// The list content
    pub children: Option<C>,
}
impl<C> Default for BreadcrumbList<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for BreadcrumbList<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <ol
                data-scope={BREADCRUMB_ANATOMY.scope()}
                data-part="list"
                data-slot="breadcrumb-list"
                class={proa_macros::text!("{} {}", BREADCRUMB_RECIPE.list, self.class)}
            >
                {self.children}
            </ol>
        }
        .render(cx)
    }
}
/// BreadcrumbItem component - `<li>` individual breadcrumb wrapper
pub struct BreadcrumbItem<C = ()> {
    /// Additional CSS classes
    pub class: Option<&'static str>,
    /// The item content
    pub children: Option<C>,
}
impl<C> Default for BreadcrumbItem<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for BreadcrumbItem<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <li
                data-scope={BREADCRUMB_ANATOMY.scope()}
                data-part="item"
                data-slot="breadcrumb-item"
                class={proa_macros::text!("{} {}", BREADCRUMB_RECIPE.item, self.class)}
            >
                {self.children}
            </li>
        }
        .render(cx)
    }
}
/// BreadcrumbLink component - `<a>` clickable navigation element
///
/// When `href` is `Some(..)` this renders a real `<a href="...">` anchor. When
/// `href` is `None` there is nothing to navigate to, so it degrades to a
/// `<span>` with the same `data-slot` and classes - an anchor without an `href`
/// is not a link, and rendering one invites accidental clicks that go nowhere.
pub struct BreadcrumbLink<C = ()> {
    /// Additional CSS classes
    pub class: Option<&'static str>,
    /// The link href
    pub href: Option<&'static str>,
    /// The link content
    pub children: Option<C>,
}
impl<C> Default for BreadcrumbLink<C> {
    fn default() -> Self {
        Self {
            class: None,
            href: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for BreadcrumbLink<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        if let Some(href) = self.href {
            html_sync! {
                <a
                    data-scope={BREADCRUMB_ANATOMY.scope()}
                    data-part="link"
                    data-slot="breadcrumb-link"
                    class={proa_macros::text!("{} {}", BREADCRUMB_RECIPE.link, self.class)}
                    href={href}
                >
                    {self.children}
                </a>
            }
            .render(cx)
        } else {
            html_sync! {
                <span
                    data-scope={BREADCRUMB_ANATOMY.scope()}
                    data-part="link"
                    data-slot="breadcrumb-link"
                    class={proa_macros::text!("{} {}", BREADCRUMB_RECIPE.link, self.class)}
                >
                    {self.children}
                </span>
            }
            .render(cx)
        }
    }
}
/// BreadcrumbPage component - `<span>` non-clickable current page indicator
pub struct BreadcrumbPage<C = ()> {
    /// Additional CSS classes
    pub class: Option<&'static str>,
    /// The page content
    pub children: Option<C>,
}
impl<C> Default for BreadcrumbPage<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for BreadcrumbPage<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <span
                data-scope={BREADCRUMB_ANATOMY.scope()}
                data-part="page"
                data-slot="breadcrumb-page"
                aria-disabled="true"
                aria-current="page"
                class={proa_macros::text!("{} {}", BREADCRUMB_RECIPE.page, self.class)}
            >
                {self.children}
            </span>
        }
        .render(cx)
    }
}
/// BreadcrumbSeparator component - `<li>` divider between breadcrumb items
///
/// Defaults to a ChevronRight SVG icon if no children are provided.
pub struct BreadcrumbSeparator<C = ()> {
    /// Additional CSS classes
    pub class: Option<&'static str>,
    /// Custom separator content; defaults to ChevronRight SVG if None
    pub children: Option<C>,
}
impl<C> Default for BreadcrumbSeparator<C> {
    fn default() -> Self {
        Self {
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for BreadcrumbSeparator<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        if let Some(children) = self.children {
            html_sync! {
                <li
                    data-scope={BREADCRUMB_ANATOMY.scope()}
                    data-part="separator"
                    data-slot="breadcrumb-separator"
                    role="presentation"
                    aria-hidden="true"
                    class={proa_macros::text!("{} {}", BREADCRUMB_RECIPE.separator, self.class)}
                >
                    {children}
                </li>
            }
            .render(cx)
        } else {
            html_sync! {
                <li
                    data-scope={BREADCRUMB_ANATOMY.scope()}
                    data-part="separator"
                    data-slot="breadcrumb-separator"
                    role="presentation"
                    aria-hidden="true"
                    class={proa_macros::text!("{} {}", BREADCRUMB_RECIPE.separator, self.class)}
                >
                    {proa_macros::content_part!("separator-glyph", UnsafeRaw(CHEVRON_RIGHT_SVG))} // proa-perf-ok: trusted static markup emitted through the low-level writer
                </li>
            }
            .render(cx)
        }
    }
}
/// BreadcrumbEllipsis component - collapsed state indicator with MoreHorizontal icon
#[derive(Default)]
pub struct BreadcrumbEllipsis {
    /// Additional CSS classes
    pub class: Option<&'static str>,
}
impl<B: WriteBuf, L: ::proa_core::DataLoader> WebRenderSync<L, B> for BreadcrumbEllipsis {
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <span
                data-scope={BREADCRUMB_ANATOMY.scope()}
                data-part="ellipsis"
                data-slot="breadcrumb-ellipsis"
                role="presentation"
                aria-hidden="true"
                class={proa_macros::text!("{} {}", BREADCRUMB_RECIPE.ellipsis, self.class)}
            >
                {proa_macros::content_part!("ellipsis-glyph", UnsafeRaw(MORE_HORIZONTAL_SVG))} // proa-perf-ok: trusted static markup emitted through the low-level writer
                <span
                    data-scope={BREADCRUMB_ANATOMY.scope()}
                    data-part="ellipsis-label"
                    data-slot="breadcrumb-ellipsis-label"
                    class={BREADCRUMB_RECIPE.ellipsis_label}
                >
                    {proa_macros::content_part!("browser-control-label", "More")}
                </span>
            </span>
        }
        .render(cx)
    }
}
