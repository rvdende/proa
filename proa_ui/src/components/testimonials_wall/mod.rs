// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

// Adapted from MIT-licensed sources:
// - Tailark, `packages/dusk-kit/blocks/testimonials/six/index.tsx`
//   https://github.com/tailark/blocks @ 8eadeb3389ccccc4bfc4a10bca959c1b39e47b71 (MIT License)
// - Tailark, `packages/mist-kit/blocks/testimonials/two/index.tsx`
//   https://github.com/tailark/blocks @ 8eadeb3389ccccc4bfc4a10bca959c1b39e47b71 (MIT License)
// - TWBlocks, `blocks/testimonials/testimonials1.tsx`
//   https://github.com/tommyjepsen/twblocks @ ff88cb951dab0dc01198b6570ec5f5cf21250d45 (MIT License)
//! Testimonials wall block — a responsive social-proof grid with customer identities.

use proa_core::{DataLoader, WebContext, WebRenderSync, WriteBuf, WriteError};
use proa_macros::html_sync;
use crate::components::{
    Avatar, AvatarFallback, AvatarShape, AvatarSize, Badge, BadgeVariant, Card, CardContent,
};

/// The flourish above a quote. The wall deliberately varies these so the grid
/// does not read as six copies of one card.
enum Topper {
    None,
    /// A five-star rating, with the rating also stated for screen readers.
    Stars,
    /// An oversized quote mark paired with a category badge.
    Category(&'static str),
    /// Short technology tags.
    Tags(&'static [&'static str]),
}

/// Light card on the page background, or the one inverted card that anchors the
/// grid. Every colour that has to flip lives here rather than being repeated as
/// a per-card class string.
#[derive(Clone, Copy, PartialEq)]
enum Tone {
    Light,
    Inverted,
}

impl Tone {
    fn card(self) -> &'static str {
        match self {
            Self::Light => "border-border/80 bg-background shadow-none",
            Self::Inverted => "border-border/80 bg-primary text-primary-foreground shadow-none",
        }
    }

    fn quote(self) -> &'static str {
        match self {
            Self::Light => "text-base leading-7 text-foreground",
            Self::Inverted => "text-lg font-medium leading-8",
        }
    }

    fn avatar(self) -> Option<&'static str> {
        match self {
            Self::Light => None,
            Self::Inverted => Some("ring-2 ring-primary-foreground/20"),
        }
    }

    fn cite(self) -> &'static str {
        match self {
            Self::Light => "not-italic text-sm font-semibold text-foreground",
            Self::Inverted => "not-italic text-sm font-semibold",
        }
    }

    fn role(self) -> &'static str {
        match self {
            Self::Light => "text-xs text-muted-foreground",
            Self::Inverted => "text-xs text-primary-foreground/70",
        }
    }
}

struct Testimonial {
    topper: Topper,
    tone: Tone,
    quote: &'static str,
    initials: &'static str,
    /// Avatar fill, spelled out in full for Tailwind's source scan.
    avatar_tint: &'static str,
    name: &'static str,
    role: &'static str,
}

impl<B: WriteBuf, L: DataLoader> WebRenderSync<L, B> for &Testimonial {
    fn render(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        // Only the star rating pushes the quote down; the other toppers carry
        // their own bottom margin.
        let quote_spacing = if matches!(self.topper, Topper::Stars) {
            "mt-5"
        } else {
            ""
        };

        html_sync! {
            {Card {
                class: Some(self.tone.card()),
                children: Some(CardContent {
                    class: Some("pt-6"),
                    children: Some(html_sync! {
                        {match &self.topper {
                            Topper::None => {}
                            Topper::Stars => html_sync! {
                                <span class="sr-only">"5 out of 5 stars"</span>
                                <div aria-hidden="true" class="text-sm tracking-[0.2em] text-amber-500">
                                    "★★★★★"
                                </div>
                            }.render(cx)?,
                            Topper::Category(label) => html_sync! {
                                <div class="mb-5 flex items-center justify-between">
                                    <span class="text-3xl font-serif text-primary">"“"</span>
                                    {Badge { variant: BadgeVariant::Secondary, class: Some("rounded-full"), children: Some(*label), }}
                                </div>
                            }.render(cx)?,
                            Topper::Tags(tags) => html_sync! {
                                <div class="mb-5 flex gap-2">
                                    {for tag in *tags {
                                        html_sync! {
                                            <span class="rounded-md bg-muted px-2 py-1 text-xs font-medium text-muted-foreground">
                                                {*tag}
                                            </span>
                                        }
                                    }}
                                </div>
                            }.render(cx)?,
                        }}
                        <blockquote class={proa_macros::text!("{} {}", quote_spacing, self.tone.quote())}>
                            {self.quote}
                        </blockquote>
                        <footer class="mt-6 flex items-center gap-3">
                            {Avatar {
                                size: AvatarSize::Lg,
                                shape: AvatarShape::Circle,
                                class: self.tone.avatar(),
                                image: None::<()>,
                                fallback: Some(AvatarFallback {
                                    size: AvatarSize::Lg,
                                    class: Some(self.avatar_tint),
                                    children: Some(self.initials),
                                }),
                            }}
                            <div>
                                <cite class={self.tone.cite()}>{self.name}</cite>
                                <p class={self.tone.role()}>{self.role}</p>
                            </div>
                        </footer>
                    }),
                }),
            }}
        }
        .render(cx)
    }
}

/// One entry per masonry column. The columns are uneven on purpose, so this is a
/// slice of slices rather than a flat list chunked at render time.
const COLUMNS: &[&[Testimonial]] = &[
    &[
        Testimonial {
            topper: Topper::Stars,
            tone: Tone::Light,
            quote: "We replaced a fragile set of templates with a component system the whole Rust team can actually improve. The first migration shipped in days.",
            initials: "AK",
            avatar_tint: "bg-primary/10 text-primary",
            name: "Ari Kim",
            role: "Engineering lead, Lantern",
        },
        Testimonial {
            topper: Topper::None,
            tone: Tone::Inverted,
            quote: "The source-first model is the rare promise that gets better as your product becomes more opinionated.",
            initials: "MP",
            avatar_tint: "bg-primary-foreground/15 text-primary-foreground",
            name: "Mina Patel",
            role: "Founder, Incline",
        },
    ],
    &[
        Testimonial {
            topper: Topper::Category("Design systems"),
            tone: Tone::Light,
            quote: "Typed recipes made our design reviews concrete. Designers can point to a token, engineers can point to an API, and both sides stay in sync.",
            initials: "JL",
            avatar_tint: "bg-violet-500/10 text-violet-600 dark:text-violet-400",
            name: "Jordan Lee",
            role: "Design director, Thread",
        },
        Testimonial {
            topper: Topper::None,
            tone: Tone::Light,
            quote: "Our marketing pages got lighter, our product surfaces got more consistent, and we stopped debugging hydration for content that never needed it.",
            initials: "SC",
            avatar_tint: "bg-emerald-500/10 text-emerald-600 dark:text-emerald-400",
            name: "Sam Chen",
            role: "Staff engineer, Northstar",
        },
    ],
    &[Testimonial {
        topper: Topper::Tags(&["SSR", "Rust"]),
        tone: Tone::Light,
        quote: "Proa let us keep the ergonomics our frontend developers expect while moving the rendering path somewhere far easier to reason about.",
        initials: "RN",
        avatar_tint: "bg-sky-500/10 text-sky-600 dark:text-sky-400",
        name: "Riley Novak",
        role: "CTO, Fieldwork",
    }],
];

const COLUMN_CLASSES: &[&str] = &[
    "space-y-4",
    "space-y-4 lg:pt-8",
    "space-y-4 md:col-span-2 lg:col-span-1",
];

/// Source-installed `testimonials_wall` block composition.
pub struct TestimonialsWallBlock;

impl<B: proa_core::WriteBuf, L: proa_core::DataLoader> proa_core::WebRenderSync<L, B> for TestimonialsWallBlock {
    fn render(
        self,
        cx: &mut proa_core::WebContext<L, B>,
    ) -> Result<(), proa_core::WriteError> {
        html_sync! {
            <div class="flex flex-col gap-6">
                <section
                    data-block="testimonials-wall"
                    class="overflow-hidden rounded-3xl border border-border bg-muted/30 px-5 py-14 md:px-8 md:py-20 lg:px-12"
                >
                    <div class="mx-auto max-w-5xl">
                        <div class="mx-auto max-w-2xl text-center">
                            {Badge { variant: BadgeVariant::Outline, class: Some("rounded-full bg-background"), children: Some("Built with Proa"), }}
                            <h2 class="mt-5 text-balance text-3xl font-semibold tracking-tight text-foreground md:text-5xl">
                                "Teams move faster when the UI stops fighting back."
                            </h2>
                            <p class="mx-auto mt-5 max-w-xl text-pretty text-lg leading-8 text-muted-foreground">
                                "From first prototype to mature design system, Proa gives product teams a clearer path to the interface they meant to ship."
                            </p>
                        </div>
                        <div class="mt-12 grid items-start gap-4 md:grid-cols-2 lg:grid-cols-3">
                            {for (index, column) in COLUMNS.iter().enumerate() {
                                html_sync! {
                                    <div class={COLUMN_CLASSES[index]}>
                                        {for testimonial in *column {
                                            html_sync! { {testimonial} }
                                        }}
                                        {if index + 1 == COLUMNS.len() {
                                            // The result card closes the wall: a number rather
                                            // than a quote, so it stays hand-written instead of
                                            // being bent into the Testimonial shape.
                                            html_sync! {
                                                {Card { class: Some("overflow-hidden border-border/80 bg-zinc-950 text-zinc-50 shadow-none"), children: Some(CardContent { class: Some("relative pt-6"), children: Some(html_sync! {
                                                    <div aria-hidden="true" class="absolute -right-8 -top-8 size-32 rounded-full bg-primary/30 blur-3xl"></div>
                                                    <p class="relative text-xs font-semibold uppercase tracking-[0.2em] text-zinc-400">
                                                        "Release velocity"
                                                    </p>
                                                    <p class="relative mt-3 text-4xl font-semibold">"2.4×"</p>
                                                    <p class="relative mt-3 text-sm leading-6 text-zinc-400">
                                                        "More product surfaces shipped per quarter after consolidating on one component system."
                                                    </p>
                                                    <div class="relative mt-6 flex items-end gap-1.5">
                                                        <span class="h-5 flex-1 rounded-xs bg-zinc-800"></span>
                                                        <span class="h-8 flex-1 rounded-xs bg-zinc-700"></span>
                                                        <span class="h-12 flex-1 rounded-xs bg-primary/70"></span>
                                                        <span class="h-16 flex-1 rounded-xs bg-primary"></span>
                                                    </div>
                                                }), }), }}
                                            }.render(cx)?;
                                        }}
                                    </div>
                                }
                            }}
                        </div>
                    </div>
                </section>
            </div>
        }
        .render(cx)
    }
}
