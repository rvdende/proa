// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

// Adapted from MIT-licensed sources:
// - Tailark, `packages/dusk-kit/blocks/pricing/five/index.tsx`
//   https://github.com/tailark/blocks @ 8eadeb3389ccccc4bfc4a10bca959c1b39e47b71 (MIT License)
// - TWBlocks, `blocks/pricing/pricing1.tsx`
//   https://github.com/tommyjepsen/twblocks @ ff88cb951dab0dc01198b6570ec5f5cf21250d45 (MIT License)
//! Pricing cards block — clear tiers, feature lists, and a featured plan.

use proa_core::WebRenderSync;
use proa_macros::html_sync;
use crate::components::{
    Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Card, CardContent, CardFooter,
    CardHeader,
};

struct Tier {
    name: &'static str,
    blurb: &'static str,
    price: &'static str,
    /// `None` on the custom-priced tier, which shows no "/ month" suffix.
    period: Option<&'static str>,
    features_heading: &'static str,
    features: &'static [&'static str],
    cta: &'static str,
    /// The recommended plan: gets the glow frame, the badge, and a solid button.
    featured: bool,
}

const TIERS: &[Tier] = &[
    Tier {
        name: "Launch",
        blurb: "For solo builders shipping the first version.",
        price: "$29",
        period: Some("/ month"),
        features_heading: "Everything to launch",
        features: &[
            "3 production projects",
            "Core component library",
            "Community support",
            "Automatic upgrades",
        ],
        cta: "Start with Launch",
        featured: false,
    },
    Tier {
        name: "Scale",
        blurb: "For teams turning reliable releases into growth.",
        price: "$89",
        period: Some("/ month"),
        features_heading: "Everything in Launch, plus",
        features: &[
            "Unlimited projects",
            "Advanced blocks and themes",
            "Up to 10 collaborators",
            "Priority implementation help",
        ],
        cta: "Choose Scale  →",
        featured: true,
    },
    Tier {
        name: "Company",
        blurb: "For organizations standardizing the product surface.",
        price: "Custom",
        period: None,
        features_heading: "Everything in Scale, plus",
        features: &[
            "Unlimited collaborators",
            "Private component registry",
            "Design-system onboarding",
            "Security and SLA review",
        ],
        cta: "Talk to our team",
        featured: false,
    },
];

/// Source-installed `pricing_cards` block composition.
pub struct PricingCardsBlock;

impl<B: proa_core::WriteBuf, L: proa_core::DataLoader> proa_core::WebRenderSync<L, B> for PricingCardsBlock {
    fn render(
        self,
        cx: &mut proa_core::WebContext<L, B>,
    ) -> Result<(), proa_core::WriteError> {
        html_sync! {
            <div class="flex flex-col gap-6">
                <section
                    data-block="pricing-cards"
                    class="rounded-3xl border border-border bg-muted/30 px-5 py-14 md:px-8 md:py-20 lg:px-12"
                >
                    <div class="mx-auto max-w-5xl">
                        <div class="mx-auto max-w-2xl text-center">
                            {Badge { variant: BadgeVariant::Outline, class: Some("rounded-full bg-background"), children: Some("Simple pricing"), }}
                            <h2 class="mt-5 text-balance text-3xl font-semibold tracking-tight text-foreground md:text-5xl">
                                "Start small. Keep your momentum."
                            </h2>
                            <p class="mx-auto mt-5 max-w-xl text-pretty text-lg leading-8 text-muted-foreground">
                                "Every plan includes the production essentials. Upgrade when your traffic, team, or ambitions demand it."
                            </p>
                            <div class="mx-auto mt-7 inline-flex items-center rounded-full border border-border bg-background px-4 py-2 text-sm text-muted-foreground">
                                "Monthly billing · cancel anytime"
                            </div>
                        </div>
                        <div class="mt-12 grid items-stretch gap-5 lg:grid-cols-3">
                            {for tier in TIERS {
                                // The featured plan sits in its own wrapper so the
                                // gradient frame can bleed a pixel past the card and
                                // the column can grow taller than its neighbours.
                                let card = Card {
                                    class: Some(if tier.featured {
                                        "relative flex h-full flex-col border-primary shadow-xl lg:py-3"
                                    } else {
                                        "flex h-full flex-col shadow-none"
                                    }),
                                    children: Some(html_sync! {
                                        {CardHeader { class: Some("gap-4"), children: Some(html_sync! {
                                            <div class="flex items-start justify-between gap-4">
                                                <div>
                                                    <h3 class="text-xl font-semibold text-foreground">
                                                        {tier.name}
                                                    </h3>
                                                    <p class="mt-2 min-h-10 text-sm leading-6 text-muted-foreground">
                                                        {tier.blurb}
                                                    </p>
                                                </div>
                                                {if tier.featured {
                                                    Badge { variant: BadgeVariant::Default, class: Some("rounded-full"), children: Some("Most popular") }.render(cx)?;
                                                }}
                                            </div>
                                            <p class="flex items-end gap-2 text-foreground">
                                                <span class="text-4xl font-semibold tracking-tight">
                                                    {tier.price}
                                                </span>
                                                {if let Some(period) = tier.period {
                                                    html_sync! {
                                                        <span class="pb-1 text-sm text-muted-foreground">
                                                            {period}
                                                        </span>
                                                    }.render(cx)?;
                                                }}
                                            </p>
                                        }), }}
                                        {CardContent { class: Some("flex-1"), children: Some(html_sync! {
                                            <p class="text-xs font-semibold uppercase tracking-widest text-muted-foreground">
                                                {tier.features_heading}
                                            </p>
                                            <ul class="mt-5 space-y-3 text-sm text-foreground">
                                                {for feature in tier.features {
                                                    html_sync! {
                                                        <li class="flex gap-3">
                                                            <span class="font-bold text-primary">"✓"</span>
                                                            {*feature}
                                                        </li>
                                                    }
                                                }}
                                            </ul>
                                        }), }}
                                        {CardFooter { children: Some(Button {
                                            size: Some(ButtonSize::Lg),
                                            variant: if tier.featured { None } else { Some(ButtonVariant::Outline) },
                                            class: Some("w-full"),
                                            children: Some(tier.cta),
                                            ..Default::default()
                                        }), ..Default::default() }}
                                    }),
                                };

                                if tier.featured {
                                    html_sync! {
                                        <div class="relative lg:-my-3">
                                            <div
                                                aria-hidden="true"
                                                class="absolute -inset-px rounded-[calc(0.75rem+1px)] bg-primary"
                                            ></div>
                                            {card}
                                        </div>
                                    }.render(cx)?;
                                } else {
                                    card.render(cx)?;
                                }
                            }}
                        </div>
                        <p class="mt-8 text-center text-sm text-muted-foreground">
                            "All plans include a 14-day trial. No credit card required. Cancel any time."
                        </p>
                    </div>
                </section>
            </div>
        }
        .render(cx)
    }
}
