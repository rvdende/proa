// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

// Adapted from MIT-licensed sources:
// - Tailark, `packages/dusk-kit/blocks/call-to-action/three/index.tsx`
//   https://github.com/tailark/blocks @ 8eadeb3389ccccc4bfc4a10bca959c1b39e47b71 (MIT License)
// - Tailark, `packages/veil-kit/blocks/call-to-action/three/index.tsx`
//   https://github.com/tailark/blocks @ 8eadeb3389ccccc4bfc4a10bca959c1b39e47b71 (MIT License)
// - TWBlocks, `blocks/cta/cta1.tsx`
//   https://github.com/tommyjepsen/twblocks @ ff88cb951dab0dc01198b6570ec5f5cf21250d45 (MIT License)
//! Newsletter CTA block — a premium signup panel with typed form primitives.

use proa_macros::html_sync;
use crate::components::{
    Avatar, AvatarFallback, AvatarShape, AvatarSize, Badge, BadgeVariant, Button, ButtonSize, Card,
    Input, InputSize, InputType, Label,
};

const PROMISES: &[&str] = &[
    "One thoughtful email monthly",
    "Practical source examples",
    "No launch-week noise",
];

/// Reader avatars in the social-proof row. Decorative, so initials carry no
/// accessible name of their own.
const READERS: &[(&str, &str)] = &[
    ("AK", "bg-violet-500 text-[10px] font-bold text-white"),
    ("SC", "bg-emerald-500 text-[10px] font-bold text-white"),
    ("JL", "bg-sky-500 text-[10px] font-bold text-white"),
];

/// Source-installed `newsletter_cta` block composition.
pub struct NewsletterCtaBlock;

impl<B: proa_core::WriteBuf, L: proa_core::DataLoader> proa_core::WebRenderSync<L, B> for NewsletterCtaBlock {
    fn render(
        self,
        cx: &mut proa_core::WebContext<L, B>,
    ) -> Result<(), proa_core::WriteError> {
        html_sync! {
            <div class="flex flex-col gap-6">
                <section
                    data-block="newsletter-cta"
                    class="relative overflow-hidden rounded-3xl border border-zinc-800 bg-zinc-950 px-5 py-14 text-zinc-50 shadow-2xl md:px-8 md:py-20 lg:px-12"
                >
                    <div
                        aria-hidden="true"
                        class="absolute -left-24 top-0 size-72 rounded-full bg-violet-600/25 blur-3xl"
                    ></div>
                    <div
                        aria-hidden="true"
                        class="absolute -bottom-32 right-0 size-80 rounded-full bg-sky-500/20 blur-3xl"
                    ></div>
                    <div
                        aria-hidden="true"
                        class="absolute inset-0 opacity-20 [background-image:linear-gradient(to_right,#ffffff12_1px,transparent_1px),linear-gradient(to_bottom,#ffffff12_1px,transparent_1px)] [background-size:32px_32px]"
                    ></div>
                    <div class="relative mx-auto grid max-w-5xl items-center gap-12 lg:grid-cols-[1fr_0.8fr]">
                        <div>
                            {Badge {
                                variant: BadgeVariant::Outline,
                                class: Some("rounded-full border-white/20 bg-white/5 text-zinc-100"),
                                children: Some("The Proa Dispatch"),
                            }}
                            <h2 class="mt-6 max-w-2xl text-balance text-4xl font-semibold tracking-tight md:text-5xl lg:text-6xl">
                                "Build sharper interfaces, one useful note at a time."
                            </h2>
                            <p class="mt-6 max-w-xl text-pretty text-lg leading-8 text-zinc-400">
                                "A concise field guide to Rust UI architecture, accessible component design, and the details that make products feel finished."
                            </p>
                            <ul class="mt-8 flex flex-wrap gap-x-6 gap-y-3 text-sm text-zinc-300">
                                {for promise in PROMISES {
                                    html_sync! {
                                        <li class="flex items-center gap-2">
                                            <span class="text-emerald-400">"✓"</span>
                                            {*promise}
                                        </li>
                                    }
                                }}
                            </ul>
                        </div>
                        {Card {
                            class: Some("border-white/10 bg-white/[0.07] p-6 text-zinc-50 shadow-2xl backdrop-blur md:p-8"),
                            children: Some(html_sync! {
                                <div class="flex size-11 items-center justify-center rounded-xl border border-white/10 bg-white/10 text-violet-200">
                                    <svg
                                        class="size-5"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="1.8"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        aria-hidden="true"
                                    >
                                        <rect width="18" height="14" x="3" y="5" rx="2" />
                                        <path d="m3 7 9 6 9-6" />
                                    </svg>
                                </div>
                                <h3 class="mt-6 text-xl font-semibold">
                                    "Join 4,800+ product builders"
                                </h3>
                                <p class="mt-2 text-sm leading-6 text-zinc-400">
                                    "Get the next issue and a compact server-rendering checklist in your inbox."
                                </p>
                                <div
                                    role="group"
                                    aria-labelledby="newsletter-email-label"
                                    class="mt-7 space-y-3"
                                >
                                    {Label {
                                        id: Some("newsletter-email-label"),
                                        for_id: Some("newsletter-email"),
                                        text: "Work email",
                                        class: Some("text-zinc-200"),
                                        ..Label::new()
                                    }}
                                    {Input {
                                        id: Some("newsletter-email"),
                                        name: Some("email"),
                                        input_type: InputType::Email,
                                        size: Some(InputSize::Lg),
                                        placeholder: Some("you@company.com"),
                                        autocomplete: Some("email"),
                                        required: true,
                                        class: Some("border-white/15 bg-zinc-950/70 text-zinc-50 placeholder:text-zinc-600"),
                                        ..Input::new()
                                    }}
                                    {Button {
                                        size: Some(ButtonSize::Lg),
                                        class: Some("w-full"),
                                        children: Some("Preview subscription  →"),
                                        ..Default::default()
                                    }}
                                </div>
                                <p class="mt-4 text-xs leading-5 text-zinc-500">
                                    "No spam. Unsubscribe in one click. By subscribing, you agree to receive product education from Proa."
                                </p>
                                <div class="mt-6 flex items-center gap-3 border-t border-white/10 pt-5">
                                    <div class="flex -space-x-2" aria-hidden="true">
                                        {for (initials, tint) in READERS {
                                            html_sync! {
                                                {Avatar { size: AvatarSize::Sm, shape: AvatarShape::Circle, class: Some("border-2 border-zinc-900"), image: None::<()>, fallback: Some(AvatarFallback { size: AvatarSize::Sm, class: Some(*tint), children: Some(*initials), }), }}
                                            }
                                        }}
                                    </div>
                                    <p class="text-xs text-zinc-400">
                                        "Read by teams building serious Rust products."
                                    </p>
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
