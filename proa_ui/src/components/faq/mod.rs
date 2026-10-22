// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

// Adapted from MIT-licensed sources:
// - Tailark, `packages/mist-kit/blocks/faqs/two/index.tsx`
//   https://github.com/tailark/blocks @ 8eadeb3389ccccc4bfc4a10bca959c1b39e47b71 (MIT License)
// - TWBlocks, `blocks/faq/faq1.tsx`
//   https://github.com/tommyjepsen/twblocks @ ff88cb951dab0dc01198b6570ec5f5cf21250d45 (MIT License)
//! FAQ block — split introduction and a Proa CSS-only accordion.

use proa_macros::html_sync;
use crate::components::{
    AccordionCSSOnly, AccordionCSSOnlyItem, AccordionCSSOnlyMode, Badge, BadgeVariant, Button,
    ButtonVariant,
};

/// The `name` every item shares. In multiple mode it groups the disclosures
/// without making them mutually exclusive, and it has to match the accordion's
/// own `name` or the group breaks apart.
const GROUP: &str = "faq-block";

struct Entry {
    id: &'static str,
    question: &'static str,
    answer: &'static str,
}

const ENTRIES: &[Entry] = &[
    Entry {
        id: "faq-block-difference",
        question: "What makes Proa different from a React component library?",
        answer: "Proa renders typed Rust components directly to HTML. You keep the familiar composition and styling model while avoiding a mandatory client runtime for static product surfaces.",
    },
    Entry {
        id: "faq-block-source",
        question: "Can I change the component source?",
        answer: "Yes. Source delivered under the applicable Free or Pro UI Pro license lives with your application, so your team can change markup, recipes, tokens, and behavior as the product evolves.",
    },
    Entry {
        id: "faq-block-islands",
        question: "Can I add interactivity without hydrating the whole page?",
        answer: "Yes. Keep static content on the synchronous rendering path, then introduce focused rsjs islands for stateful controls such as pickers, menus, and complex forms.",
    },
    Entry {
        id: "faq-block-design-system",
        question: "Will Proa fit an existing design system?",
        answer: "The components use semantic theme tokens and typed recipes, making it straightforward to map your colors, radius, spacing, and interaction states without rebuilding each primitive.",
    },
    Entry {
        id: "faq-block-accessibility",
        question: "How are accessibility and updates handled?",
        answer: "Accessible semantics are part of each component's API and tests. Active subscribers receive ongoing component improvements, fixes, new blocks, and implementation guidance.",
    },
    Entry {
        id: "faq-block-team",
        question: "Can my whole team use it?",
        answer: "Your commercial license defines the covered team and products. The source-first workflow keeps everyone aligned in the same repository and review process.",
    },
];

/// Source-installed `faq` block composition.
pub struct FaqBlock;

impl<B: proa_core::WriteBuf, L: proa_core::DataLoader> proa_core::WebRenderSync<L, B> for FaqBlock {
    fn render(
        self,
        cx: &mut proa_core::WebContext<L, B>,
    ) -> Result<(), proa_core::WriteError> {
        html_sync! {
            <div class="flex flex-col gap-6">
                <section
                    data-block="faq"
                    class="rounded-3xl border border-border bg-background px-5 py-14 shadow-xs md:px-8 md:py-20 lg:px-12"
                >
                    <div class="mx-auto grid max-w-5xl gap-12 lg:grid-cols-[0.75fr_1.25fr] lg:gap-20">
                        <div class="lg:sticky lg:top-24 lg:self-start">
                            {Badge { variant: BadgeVariant::Outline, class: Some("rounded-full"), children: Some("Questions, answered"), }}
                            <h2 class="mt-5 text-balance text-3xl font-semibold tracking-tight text-foreground md:text-5xl">
                                "Everything you need to build with confidence."
                            </h2>
                            <p class="mt-5 text-pretty text-lg leading-8 text-muted-foreground">
                                "The short version: Proa gives your team polished primitives, the source code, and a Rust-first path to production."
                            </p>
                            <div class="mt-8">
                                {Button { variant: Some(ButtonVariant::Outline), children: Some("Still curious? Talk to us  →"), ..Default::default() }}
                            </div>
                        </div>
                        {AccordionCSSOnly { mode: AccordionCSSOnlyMode::Multiple, name: GROUP, class: Some("border-y border-border"), children: Some(html_sync! {
                            {for (index, entry) in ENTRIES.iter().enumerate() {
                                html_sync! {
                                    {AccordionCSSOnlyItem {
                                        id: entry.id,
                                        name: GROUP,
                                        single_mode: false,
                                        // The first answer starts open so the panel is not a
                                        // wall of closed rows on first paint.
                                        open: index == 0,
                                        disabled: false,
                                        class: Some("py-1"),
                                        trigger: Some(entry.question),
                                        children: Some(entry.answer),
                                    }}
                                }
                            }}
                        }), }}
                    </div>
                </section>
            </div>
        }
        .render(cx)
    }
}
