// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

// Adapted from MIT-licensed sources:
// - Tailark, `packages/dusk-kit/blocks/integrations/eight/index.tsx`
//   https://github.com/tailark/blocks @ 8eadeb3389ccccc4bfc4a10bca959c1b39e47b71 (MIT License)
// - Tailark, `packages/mist-kit/blocks/integrations/two/index.tsx`
//   https://github.com/tailark/blocks @ 8eadeb3389ccccc4bfc4a10bca959c1b39e47b71 (MIT License)
//! Integrations grid block — product copy paired with a compact ecosystem matrix.

use proa_macros::html_sync;
use crate::components::{Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Card};

struct Integration {
    glyph: &'static str,
    /// Tile tint. Spelled out in full because Tailwind scans for literal class
    /// names and would not see a name assembled at runtime.
    tint: &'static str,
    name: &'static str,
    summary: &'static str,
}

const INTEGRATIONS: &[Integration] = &[
    Integration {
        glyph: "DB",
        tint: "bg-violet-500/10 text-violet-600 dark:text-violet-400",
        name: "Postgres",
        summary: "Typed queries, migrations, and live application data.",
    },
    Integration {
        glyph: "ID",
        tint: "bg-indigo-500/10 text-indigo-600 dark:text-indigo-400",
        name: "Identity",
        summary: "Sessions, roles, organizations, and secure access.",
    },
    Integration {
        glyph: "S3",
        tint: "bg-sky-500/10 text-sky-600 dark:text-sky-400",
        name: "Object storage",
        summary: "Uploads, documents, media, and signed delivery.",
    },
    Integration {
        glyph: "$",
        tint: "bg-emerald-500/10 text-emerald-600 dark:text-emerald-400",
        name: "Payments",
        summary: "Checkout, subscriptions, invoices, and webhooks.",
    },
    Integration {
        glyph: "↗",
        tint: "bg-amber-500/10 text-amber-600 dark:text-amber-400",
        name: "Deployments",
        summary: "Preview environments and production releases.",
    },
    Integration {
        glyph: "◎",
        tint: "bg-rose-500/10 text-rose-600 dark:text-rose-400",
        name: "Observability",
        summary: "Traces, metrics, logs, and actionable alerts.",
    },
];

/// Source-installed `integrations_grid` block composition.
pub struct IntegrationsGridBlock;

impl<B: proa_core::WriteBuf, L: proa_core::DataLoader> proa_core::WebRenderSync<L, B> for IntegrationsGridBlock {
    fn render(
        self,
        cx: &mut proa_core::WebContext<L, B>,
    ) -> Result<(), proa_core::WriteError> {
        html_sync! {
            <div class="flex flex-col gap-6">
                <section
                    data-block="integrations-grid"
                    class="rounded-3xl border border-border bg-muted/30 px-5 py-14 md:px-8 md:py-20 lg:px-12"
                >
                    <div class="mx-auto grid max-w-5xl items-stretch gap-12 lg:grid-cols-[0.8fr_1.2fr]">
                        <div class="flex flex-col">
                            <div>
                                {Badge {
                                    variant: BadgeVariant::Outline,
                                    class: Some("rounded-full bg-background"),
                                    children: Some("Your stack, connected"),
                                }}
                                <h2 class="mt-5 text-balance text-3xl font-semibold tracking-tight text-foreground md:text-5xl">
                                    "Meet your tools where the work already happens."
                                </h2>
                                <p class="mt-5 text-pretty text-lg leading-8 text-muted-foreground">
                                    "Connect deployment, data, billing, identity, and observability without turning your product into a maze of one-off screens."
                                </p>
                                <div class="mt-8 flex flex-wrap gap-3">
                                    {Button {
                                        size: Some(ButtonSize::Lg),
                                        children: Some("Browse integrations  →"),
                                        ..Default::default()
                                    }}
                                    {Button {
                                        size: Some(ButtonSize::Lg),
                                        variant: Some(ButtonVariant::Outline),
                                        children: Some("Read the API docs"),
                                        ..Default::default()
                                    }}
                                </div>
                            </div>
                        </div>
                        {Card {
                            class: Some("overflow-hidden rounded-2xl border-border/80 bg-background p-3 shadow-xl"),
                            children: Some(html_sync! {
                                <div class="flex items-center justify-between border-b border-border px-3 py-3">
                                    <div>
                                        <p class="text-sm font-semibold text-foreground">
                                            "Integration catalog"
                                        </p>
                                        <p class="mt-1 text-xs text-muted-foreground">
                                            "Six production-ready connections"
                                        </p>
                                    </div>
                                    {Badge {
                                        variant: BadgeVariant::Success,
                                        class: Some("rounded-full"),
                                        children: Some("All systems online"),
                                    }}
                                </div>
                                <div class="grid gap-2 p-3 sm:grid-cols-2">
                                    {for integration in INTEGRATIONS {
                                        html_sync! {
                                            {Card { class: Some("group space-y-4 border-border/70 p-4 shadow-none transition-colors hover:bg-muted/50"), children: Some(html_sync! {
                                                <div class="flex items-center justify-between">
                                                    <span class={proa_macros::text!("flex size-10 items-center justify-center rounded-xl text-sm font-bold {}", integration.tint)}>
                                                        {integration.glyph}
                                                    </span>
                                                    <span class="size-2 rounded-full bg-emerald-500"></span>
                                                </div>
                                                <div>
                                                    <h3 class="text-sm font-semibold text-foreground">
                                                        {integration.name}
                                                    </h3>
                                                    <p class="mt-1 text-xs leading-5 text-muted-foreground">
                                                        {integration.summary}
                                                    </p>
                                                </div>
                                            }), }}
                                        }
                                    }}
                                </div>
                            }),
                        }}
                    </div>
                </section>
            </div>
        }
        .render(cx)
    }
}
