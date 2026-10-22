// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Image recipe — typed fit/layout axes and a named placeholder slot.

use crate::tw_join;
use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

/// Object-fit mode for the image.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, proa_macros::ProaVariant)]
#[variant(name = "fit", path = "crate::components::_proa::variant_spec")]
pub enum ImageFit {
    #[default]
    None,
    Cover,
    Contain,
    Fill,
}

impl ImageFit {
    pub const fn classes(self) -> &'static str {
        match self {
            Self::None => "",
            Self::Cover => "object-cover",
            Self::Contain => "object-contain",
            Self::Fill => "object-fill",
        }
    }

    /// Compatibility form of the former component-local helper.
    pub const fn class(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            _ => Some(self.classes()),
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Cover => "cover",
            Self::Contain => "contain",
            Self::Fill => "fill",
        }
    }
}

/// Optional inputs accepted by [`ImageRecipe::resolve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ImageRecipeProps {
    pub fit: Option<ImageFit>,
    pub fill: Option<bool>,
    pub rounded: Option<bool>,
}

/// Static Image slots, modifiers, and defaults.
#[derive(Debug, Clone, Copy)]
pub struct ImageRecipe {
    pub root: &'static str,
    pub placeholder: &'static str,
    pub fill: &'static str,
    pub rounded: &'static str,
    fit_default: ImageFit,
    fill_default: bool,
    rounded_default: bool,
}

impl ImageRecipe {
    pub const fn resolve(&'static self, props: ImageRecipeProps) -> ImageStyle {
        let fit = match props.fit {
            Some(fit) => fit,
            None => self.fit_default,
        };
        let fill = match props.fill {
            Some(fill) => fill,
            None => self.fill_default,
        };
        let rounded = match props.rounded {
            Some(rounded) => rounded,
            None => self.rounded_default,
        };
        ImageStyle {
            recipe: self,
            fit,
            fill,
            rounded,
        }
    }
}

/// Fully resolved, streamable Image style.
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct ImageStyle {
    recipe: &'static ImageRecipe,
    fit: ImageFit,
    fill: bool,
    rounded: bool,
}

impl ImageStyle {
    pub const fn fit(self) -> ImageFit {
        self.fit
    }

    pub const fn fill(self) -> bool {
        self.fill
    }

    pub const fn rounded(self) -> bool {
        self.rounded
    }
}

impl AttrValue for ImageStyle {
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
        if !self.fit.classes().is_empty() {
            cx.out.extend_static(b" ")?;
            cx.out.extend_static(self.fit.classes().as_bytes())?;
        }
        if self.fill {
            cx.out.extend_static(b" ")?;
            cx.out.extend_static(self.recipe.fill.as_bytes())?;
        }
        if self.rounded {
            cx.out.extend_static(b" ")?;
            cx.out.extend_static(self.recipe.rounded.as_bytes())?;
        }
        Ok(())
    }
}

pub const IMAGE_RECIPE: ImageRecipe = ImageRecipe {
    root: "max-w-full",
    placeholder: tw_join!("relative", "overflow-hidden"),
    fill: tw_join!("absolute", "inset-0", "w-full", "h-full"),
    rounded: "rounded-lg",
    fit_default: ImageFit::None,
    fill_default: false,
    rounded_default: false,
};

pub const fn image_recipe() -> &'static ImageRecipe {
    &IMAGE_RECIPE
}

impl crate::components::_proa::variant_spec::ComponentSpec for ImageRecipe {
    const NAME: &'static str = "Image";

    fn base_classes(&self) -> &'static str {
        self.root
    }

    fn variant_metadata(&self) -> Vec<crate::components::_proa::variant_spec::VariantMetadata> {
        vec![crate::components::_proa::variant_spec::variant_metadata::<ImageFit>()]
    }
}

/// Compatibility accessor for the former base helper.
pub const fn image_base() -> &'static str {
    IMAGE_RECIPE.root
}
