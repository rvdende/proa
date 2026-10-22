// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

// Adapted from MIT-licensed sources:
// - Tailark, `packages/mist-kit/blocks/features/eight/index.tsx`
//   https://github.com/tailark/blocks @ 8eadeb3389ccccc4bfc4a10bca959c1b39e47b71 (MIT License)
// - TWBlocks, `blocks/feature/feature7.tsx`
//   https://github.com/tommyjepsen/twblocks @ ff88cb951dab0dc01198b6570ec5f5cf21250d45 (MIT License)
//! Bento features block — asymmetric product stories with lightweight previews.

use proa_macros::html_sync;
use crate::components::{Badge, BadgeVariant, Card};

/// Source-installed `features_bento` block composition.
pub struct FeaturesBentoBlock;

impl<B: proa_core::WriteBuf, L: proa_core::DataLoader> proa_core::WebRenderSync<L, B> for FeaturesBentoBlock {
    fn render(
        self,
        cx: &mut proa_core::WebContext<L, B>,
    ) -> Result<(), proa_core::WriteError> {
        html_sync! {
            <div class="flex flex-col gap-6">
                <section
                    data-block="features-bento"
                    class="rounded-3xl border border-border bg-muted/30 px-5 py-14 md:px-8 md:py-20 lg:px-12"
                >
                    <div class="mx-auto max-w-5xl">
                        <div class="max-w-2xl">
                            {Badge {
                                variant: BadgeVariant::Outline,
                                class: Some("rounded-full bg-background"),
                                children: Some("Built for product teams"),
                            }}
                            <h2 class="mt-5 text-balance text-3xl font-semibold tracking-tight text-foreground md:text-5xl">
                                "One system from first sketch to production."
                            </h2>
                            <p class="mt-5 text-pretty text-lg leading-8 text-muted-foreground">
                                "Compose dependable primitives, keep every design decision visible, and ship server-rendered experiences without a client-side tax."
                            </p>
                        </div>
                        <div class="mt-10 grid gap-4 md:grid-cols-2 lg:grid-cols-3">
                            {Card {
                                class: Some("group overflow-hidden p-0 md:col-span-2 lg:col-span-2"),
                                children: Some(html_sync! {
                                    <div class="grid h-full gap-8 p-6 md:p-8 xl:grid-cols-[0.8fr_1.2fr]">
                                        <div class="flex flex-col justify-between">
                                            <div>
                                                <span class="flex size-10 items-center justify-center rounded-xl border border-border bg-background text-sm font-semibold text-primary">
                                                    "01"
                                                </span>
                                                <h3 class="mt-5 text-xl font-semibold text-foreground">
                                                    "Rust all the way down"
                                                </h3>
                                                <p class="mt-3 leading-7 text-muted-foreground">
                                                    "Render typed components on the server and add browser behavior only where the experience earns it."
                                                </p>
                                            </div>
                                            <p class="mt-8 text-sm font-medium text-foreground">
                                                "Fast path by default  →"
                                            </p>
                                        </div>
                                        <div class="overflow-hidden rounded-2xl border border-border bg-zinc-950 p-3 shadow-xl">
                                            <div class="flex items-center gap-1.5 border-b border-white/10 px-2 pb-3">
                                                <span class="size-2 rounded-full bg-rose-400"></span>
                                                <span class="size-2 rounded-full bg-amber-400"></span>
                                                <span class="size-2 rounded-full bg-emerald-400"></span>
                                                <span class="ml-2 text-[10px] text-zinc-500">
                                                    "hero.rs"
                                                </span>
                                            </div>
                                            <pre class="overflow-x-auto px-3 py-5 text-xs leading-6 text-zinc-300">
                                                <code class="block min-w-max">
                                                    <span class="text-fuchsia-300">
                                                        "html_sync!"
                                                    </span>
                                                    " {\n"
                                                    "    "
                                                    <span class="text-sky-300">"<section"</span>
                                                    " class="
                                                    <span class="text-emerald-300">
                                                        "\"grid gap-8\""
                                                    </span>
                                                    <span class="text-sky-300">">"</span>
                                                    "\n        "
                                                    <span class="text-sky-300">"<h1>"</span>
                                                    "Ship in Rust"
                                                    <span class="text-sky-300">"</h1>"</span>
                                                    "\n        "
                                                    <span class="text-amber-200">
                                                        "{product_preview}"
                                                    </span>
                                                    "\n    "
                                                    <span class="text-sky-300">"</section>"</span>
                                                    "\n}"
                                                </code>
                                            </pre>
                                            <div class="grid grid-cols-3 gap-2 border-t border-white/10 px-2 pt-3 text-center text-[10px] text-zinc-500">
                                                <span>"0 hydration"</span>
                                                <span>"typed HTML"</span>
                                                <span>"7.6ms SSR"</span>
                                            </div>
                                        </div>
                                    </div>
                                }),
                            }}
                            {Card {
                                class: Some("overflow-hidden p-6 md:p-8"),
                                children: Some(html_sync! {
                                    <span class="flex size-10 items-center justify-center rounded-xl border border-border bg-muted text-sm font-semibold text-primary">
                                        "02"
                                    </span>
                                    <h3 class="mt-5 text-xl font-semibold text-foreground">
                                        "Accessible by construction"
                                    </h3>
                                    <p class="mt-3 leading-7 text-muted-foreground">
                                        "Clear semantics, keyboard-ready patterns, and ARIA wiring live alongside the component API."
                                    </p>
                                    <div class="mt-8 space-y-2 rounded-xl border border-border bg-muted/40 p-3">
                                        <div class="flex items-center justify-between rounded-lg bg-background px-3 py-2 text-xs shadow-xs">
                                            <span class="text-foreground">
                                                "Keyboard navigation"
                                            </span>
                                            <span class="font-medium text-emerald-600 dark:text-emerald-400">
                                                "Ready"
                                            </span>
                                        </div>
                                        <div class="flex items-center justify-between rounded-lg bg-background px-3 py-2 text-xs shadow-xs">
                                            <span class="text-foreground">"Focus management"</span>
                                            <span class="font-medium text-emerald-600 dark:text-emerald-400">
                                                "Ready"
                                            </span>
                                        </div>
                                    </div>
                                }),
                            }}
                            {Card {
                                class: Some("overflow-hidden p-6 md:p-8"),
                                children: Some(html_sync! {
                                    <span class="flex size-10 items-center justify-center rounded-xl border border-border bg-muted text-sm font-semibold text-primary">
                                        "03"
                                    </span>
                                    <h3 class="mt-5 text-xl font-semibold text-foreground">
                                        "Tokens stay coherent"
                                    </h3>
                                    <p class="mt-3 leading-7 text-muted-foreground">
                                        "Theme primitives and typed recipes keep every surface on-brand without class-string sprawl."
                                    </p>
                                    <div class="mt-8 flex items-end gap-2 rounded-xl border border-border bg-muted/40 p-4">
                                        <span class="h-10 flex-1 rounded-md bg-primary/20"></span>
                                        <span class="h-16 flex-1 rounded-md bg-primary/40"></span>
                                        <span class="h-24 flex-1 rounded-md bg-primary/70"></span>
                                        <span class="h-32 flex-1 rounded-md bg-primary"></span>
                                    </div>
                                }),
                            }}
                            {Card {
                                class: Some("overflow-hidden p-6 md:col-span-2 md:p-8"),
                                children: Some(html_sync! {
                                    <div class="grid items-center gap-8 sm:grid-cols-[1fr_auto]">
                                        <div>
                                            <span class="flex size-10 items-center justify-center rounded-xl border border-border bg-muted text-sm font-semibold text-primary">
                                                "04"
                                            </span>
                                            <h3 class="mt-5 text-xl font-semibold text-foreground">
                                                "Change every layer"
                                            </h3>
                                            <p class="mt-3 max-w-xl leading-7 text-muted-foreground">
                                                "Copy the source, adapt the API, and evolve the system with your product. No opaque runtime and no design-system dead end."
                                            </p>
                                        </div>
                                        <div class="grid grid-cols-2 gap-2 text-center text-xs font-medium">
                                            <span class="rounded-xl border border-border bg-muted/50 px-4 py-5 text-foreground">
                                                "Source"
                                            </span>
                                            <span class="rounded-xl border border-border bg-muted/50 px-4 py-5 text-foreground">
                                                "Recipes"
                                            </span>
                                            <span class="rounded-xl border border-border bg-muted/50 px-4 py-5 text-foreground">
                                                "Themes"
                                            </span>
                                            <span class="rounded-xl border border-border bg-primary px-4 py-5 text-primary-foreground">
                                                "Control"
                                            </span>
                                        </div>
                                    </div>
                                }),
                            }}
                        </div>
                    </div>
                </section>
            </div>
        }
        .render(cx)
    }
}
