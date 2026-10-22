// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Avatar component implementations
//!
//! Renders avatar images with fallback support. The TypeScript hydration
//! code handles image load errors and shows the fallback when needed.
use super::recipe::{AvatarRecipeProps, AvatarShape, AvatarSize, AVATAR_RECIPE};
use super::AVATAR_ANATOMY;
use proa_core::{WebContext, WebRenderSync, WriteBuf};
use proa_macros::html_sync;
/// Avatar root component - container for image and fallback
///
/// On SSR, this renders a container with the image and fallback.
/// The TypeScript code handles showing the fallback when the image fails to load.
pub struct Avatar<Image = (), Fallback = ()> {
    /// Size variant
    pub size: AvatarSize,
    /// Shape variant
    pub shape: AvatarShape,
    /// Additional CSS classes
    pub class: Option<&'static str>,
    /// The avatar image
    pub image: Option<Image>,
    /// The fallback content (shown when image fails)
    pub fallback: Option<Fallback>,
}
impl Avatar<(), ()> {
    pub const fn new() -> Self {
        Self {
            size: AvatarSize::Md,
            shape: AvatarShape::Circle,
            class: None,
            image: None,
            fallback: None,
        }
    }
}
impl<Image, Fallback> Default for Avatar<Image, Fallback> {
    fn default() -> Self {
        Self {
            size: AvatarSize::Md,
            shape: AvatarShape::Circle,
            class: None,
            image: None,
            fallback: None,
        }
    }
}
impl<
        B: WriteBuf,
        Image: WebRenderSync<L, B>,
        Fallback: WebRenderSync<L, B>,
        L: ::proa_core::DataLoader,
    > WebRenderSync<L, B> for Avatar<Image, Fallback>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = AVATAR_RECIPE.resolve(AvatarRecipeProps {
            size: Some(self.size),
            shape: Some(self.shape),
        });
        html_sync! {
            <span
                data-avatar
                data-scope={AVATAR_ANATOMY.scope()}
                data-part="root"
                data-slot="avatar"
                data-size={style.size().as_str()}
                data-shape={style.shape().as_str()}
                class={proa_macros::text!("{} {}", style, self.class)}
            >
                {self.image}
                {self.fallback}
            </span>
        }
        .render(cx)
    }
}
/// AvatarImage - the image element
#[derive(Default)]
pub struct AvatarImage {
    /// Image source URL
    pub src: &'static str,
    /// Alt text for accessibility
    pub alt: &'static str,
    /// Additional CSS classes
    pub class: Option<&'static str>,
}
impl<B: WriteBuf, L: ::proa_core::DataLoader> WebRenderSync<L, B> for AvatarImage {
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        html_sync! {
            <img
                data-avatar-image
                data-scope={AVATAR_ANATOMY.scope()}
                data-part="image"
                data-slot="avatar-image"
                src={proa_macros::content_part!("image-src", self.src)}
                alt={proa_macros::content_part!("image-alt", self.alt)}
                class={proa_macros::text!("{} {}", AVATAR_RECIPE.image, self.class)}
            />
        }
        .render(cx)
    }
}
/// AvatarFallback - content shown when image fails to load
pub struct AvatarFallback<C = ()> {
    /// Size for text sizing (should match parent Avatar)
    pub size: AvatarSize,
    /// Additional CSS classes
    pub class: Option<&'static str>,
    /// The fallback content (typically initials)
    pub children: Option<C>,
}
impl<C> Default for AvatarFallback<C> {
    fn default() -> Self {
        Self {
            size: AvatarSize::Md,
            class: None,
            children: None,
        }
    }
}
impl<B: WriteBuf, C: WebRenderSync<L, B>, L: ::proa_core::DataLoader> WebRenderSync<L, B>
    for AvatarFallback<C>
{
    fn render(
        self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), proa_core::shared::write_buf::WriteError> {
        let style = AVATAR_RECIPE.resolve_fallback(self.size);
        html_sync! {
            <span
                data-avatar-fallback
                data-scope={AVATAR_ANATOMY.scope()}
                data-part="fallback"
                data-slot="avatar-fallback"
                data-size={self.size.as_str()}
                class={proa_macros::text!("{} {}", style, self.class)}
            >
                {self.children}
            </span>
        }
        .render(cx)
    }
}
