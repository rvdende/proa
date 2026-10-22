// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

// Adapted from MIT-licensed sources:
// - Tailark, `packages/mist-kit/blocks/hero-section/four/index.tsx`
//   https://github.com/tailark/blocks @ 8eadeb3389ccccc4bfc4a10bca959c1b39e47b71 (MIT License)
// - TWBlocks, `blocks/hero/hero3.tsx`
//   https://github.com/tommyjepsen/twblocks @ ff88cb951dab0dc01198b6570ec5f5cf21250d45 (MIT License)
//! SaaS hero block — product-led copy, dual CTAs, and an app preview.

use proa_macros::html_sync;
use crate::components::{Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Card};

const NAV_LINKS: &[(&str, &str)] = &[
    ("/components", "Platform"),
    ("/blocks", "Customers"),
    ("/pricing", "Pricing"),
];

/// Preview-panel metrics. The trailing note is toned green when it reports an
/// improvement and muted when it is only context.
const METRICS: &[(&str, &str, &str, &str)] = &[
    (
        "Requests",
        "1.84M",
        "↑ 18.2%",
        "mt-1 text-xs text-emerald-600 dark:text-emerald-400",
    ),
    (
        "Render time",
        "7.6ms",
        "↓ 2.1ms",
        "mt-1 text-xs text-emerald-600 dark:text-emerald-400",
    ),
    (
        "Uptime",
        "99.99%",
        "Last 30 days",
        "mt-1 text-xs text-muted-foreground",
    ),
];

const DEPLOYMENTS: &[(&str, &str)] = &[
    ("catalog-v2", "2 min ago"),
    ("dashboard-polish", "18 min ago"),
];

/// Source-installed `hero_saas` block composition.
pub struct HeroSaasBlock;

impl<B: proa_core::WriteBuf, L: proa_core::DataLoader> proa_core::WebRenderSync<L, B> for HeroSaasBlock {
    fn render(
        self,
        cx: &mut proa_core::WebContext<L, B>,
    ) -> Result<(), proa_core::WriteError> {
        html_sync! {
            <div class="flex flex-col gap-6">
                <section
                    data-block="hero-saas"
                    class="overflow-hidden rounded-3xl border border-border bg-background shadow-xs"
                >
                    <header class="flex items-center justify-between border-b border-border/70 px-5 py-4 md:px-8">
                        <a
                            href="/blocks/hero_saas"
                            class="flex items-center gap-2 font-semibold tracking-tight text-foreground"
                        >
                            <span class="flex size-7 items-center justify-center rounded-lg bg-primary text-xs font-bold text-primary-foreground">
                                "P"
                            </span>
                            "Proa Cloud"
                        </a>
                        <nav
                            aria-label="Product navigation"
                            class="hidden items-center gap-6 text-sm text-muted-foreground md:flex"
                        >
                            {for (href, label) in NAV_LINKS {
                                html_sync! {
                                    <a href={*href} class="transition-colors hover:text-foreground">
                                        {*label}
                                    </a>
                                }
                            }}
                        </nav>
                        {Button { size: Some(ButtonSize::Sm), children: Some("Start building"), ..Default::default() }}
                    </header>
                    <div class="relative px-5 py-14 md:px-8 md:py-20 lg:px-12">
                        <div
                            aria-hidden="true"
                            class="absolute inset-x-0 top-0 h-72 bg-gradient-to-b from-primary/10 to-transparent"
                        ></div>
                        <div class="relative grid items-center gap-12 lg:grid-cols-[0.9fr_1.1fr]">
                            <div class="flex flex-col items-start">
                                {Badge { variant: BadgeVariant::Outline, class: Some("rounded-full bg-background/80"), children: Some("Rust-native. Web-ready."), }}
                                <h2 class="mt-6 max-w-xl text-balance text-4xl font-semibold tracking-tight text-foreground md:text-5xl lg:text-6xl">
                                    "Ship the interface your product deserves."
                                </h2>
                                <p class="mt-6 max-w-lg text-pretty text-lg leading-8 text-muted-foreground">
                                    "Build polished, accessible product surfaces in Rust. Proa keeps rendering fast, components composable, and your design system fully yours."
                                </p>
                                <div class="mt-8 flex flex-col gap-3 sm:flex-row">
                                    {Button { size: Some(ButtonSize::Lg), class: Some("min-w-40"), children: Some("Create a project  →"), ..Default::default() }}
                                    {Button { size: Some(ButtonSize::Lg), variant: Some(ButtonVariant::Outline), class: Some("min-w-40"), children: Some("Explore components"), ..Default::default() }}
                                </div>
                                <p class="mt-4 text-xs text-muted-foreground">
                                    "No framework lock-in · Accessible defaults · Source in your repo"
                                </p>
                            </div>
                            <div class="relative mx-auto w-full max-w-2xl">
                                <div
                                    aria-hidden="true"
                                    class="absolute -inset-6 rounded-[2rem] bg-primary/10 blur-3xl"
                                ></div>
                                {Card { class: Some("relative overflow-hidden rounded-2xl border-border/80 bg-card/95 shadow-2xl"), children: Some(html_sync! {
                                    <div class="flex items-center justify-between border-b border-border px-5 py-4">
                                        <div>
                                            <p class="text-xs font-medium uppercase tracking-widest text-muted-foreground">
                                                "Production"
                                            </p>
                                            <p class="mt-1 font-medium text-foreground">
                                                "Workspace overview"
                                            </p>
                                        </div>
                                        {Badge { variant: BadgeVariant::Success, class: Some("rounded-full"), children: Some("Healthy"), }}
                                    </div>
                                    <div class="grid gap-3 p-5 sm:grid-cols-3 lg:grid-cols-1 xl:grid-cols-3">
                                        {for (label, value, note, note_tone) in METRICS {
                                            html_sync! {
                                                <div class="rounded-xl border border-border bg-muted/40 p-4">
                                                    <p class="text-xs text-muted-foreground">{*label}</p>
                                                    <p class="mt-2 text-2xl font-semibold text-foreground">
                                                        {*value}
                                                    </p>
                                                    <p class={*note_tone}>{*note}</p>
                                                </div>
                                            }
                                        }}
                                    </div>
                                    <div class="px-5 pb-5">
                                        <div class="rounded-xl border border-border">
                                            <div class="flex items-center justify-between border-b border-border px-4 py-3">
                                                <span class="text-sm font-medium text-foreground">
                                                    "Recent deployments"
                                                </span>
                                                <span class="text-xs text-muted-foreground">
                                                    "View all"
                                                </span>
                                            </div>
                                            {for (index, (name, when)) in DEPLOYMENTS.iter().enumerate() {
                                                // Only rows after the first carry a top rule; the
                                                // header above already provides the first divider.
                                                let row = if index == 0 {
                                                    "flex items-center justify-between px-4 py-3 text-sm"
                                                } else {
                                                    "flex items-center justify-between border-t border-border px-4 py-3 text-sm"
                                                };

                                                html_sync! {
                                                    <div class={row}>
                                                        <div class="flex items-center gap-3">
                                                            <span class="size-2 rounded-full bg-emerald-500"></span>
                                                            <span class="text-foreground">{*name}</span>
                                                        </div>
                                                        <span class="text-xs text-muted-foreground">
                                                            {*when}
                                                        </span>
                                                    </div>
                                                }
                                            }}
                                        </div>
                                    </div>
                                }), }}
                            </div>
                        </div>
                    </div>
                </section>
            </div>
        }
        .render(cx)
    }
}
