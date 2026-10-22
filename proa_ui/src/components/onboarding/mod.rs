// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Provenance: adapted from `content/components/onboarding/onboarding-02.tsx` in
//! https://github.com/ephraimduncan/blocks at commit
//! fe4305a1147355f5cd1ca9cb03b6fa88f013bd6a. MIT License, Copyright (c) 2025
//! Ephraim Duncan; see the upstream `LICENSE.md`.

use proa_core::{StaticRaw, WebRenderSync};
use proa_macros::html_sync;
use crate::components::{Button, ButtonSize, Card, Progress};

/// Where a checklist step sits relative to the user's progress. Card framing,
/// the leading marker, and the row layout all follow from this one value.
#[derive(Clone, Copy, PartialEq)]
enum StepState {
    Complete,
    Current,
    Upcoming,
}

impl StepState {
    fn card(self) -> &'static str {
        match self {
            Self::Current => {
                "border-primary/40 bg-primary/[0.04] py-0 shadow-xs ring-1 ring-primary/10"
            }
            _ => "border-border/70 bg-background py-0 shadow-xs",
        }
    }
}

struct Step {
    numeral: &'static str,
    title: &'static str,
    summary: &'static str,
    state: StepState,
    /// Shell command shown under the active step. Only the current step has one.
    command: Option<&'static str>,
    /// Trailing affordance icon on a not-yet-started step: raw inner SVG markup
    /// for a 24×24 viewBox.
    icon: Option<&'static str>,
}

const STEPS: &[Step] = &[
    Step {
        numeral: "1",
        title: "Create your workspace",
        summary: "Name your workspace and choose the team defaults.",
        state: StepState::Complete,
        command: None,
        icon: None,
    },
    Step {
        numeral: "2",
        title: "Install the Proa CLI",
        summary:
            "Connect your project, verify the toolchain, and render your first component locally.",
        state: StepState::Current,
        command: Some("cargo proa add button"),
        icon: None,
    },
    Step {
        numeral: "3",
        title: "Connect the private registry",
        summary: "Add your access token and choose which teams can install premium components.",
        state: StepState::Upcoming,
        command: None,
        icon: Some(
            r#"<rect width="18" height="11" x="3" y="11" rx="2" /><path d="M7 11V7a5 5 0 0 1 10 0v4" />"#,
        ),
    },
    Step {
        numeral: "4",
        title: "Invite your team",
        summary: "Bring designers and engineers into one shared component workflow.",
        state: StepState::Upcoming,
        command: None,
        icon: Some(
            r#"<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" /><circle cx="9" cy="7" r="4" /><path d="M19 8v6M22 11h-6" />"#,
        ),
    },
];

/// Source-installed `onboarding` block composition.
pub struct OnboardingBlock;

impl<B: proa_core::WriteBuf, L: proa_core::DataLoader> proa_core::WebRenderSync<L, B> for OnboardingBlock {
    fn render(
        self,
        cx: &mut proa_core::WebContext<L, B>,
    ) -> Result<(), proa_core::WriteError> {
        html_sync! {
            <div class="flex flex-col gap-6">
                <div class="flex min-h-[720px] items-center justify-center rounded-2xl border border-border bg-muted/30 p-4 sm:p-8">
                    <div class="w-full max-w-xl">
                        <div class="mb-6 flex items-start justify-between gap-4">
                            <div>
                                <p class="text-sm font-medium text-primary">
                                    "Welcome to Proa Cloud"
                                </p>
                                <h2 class="mt-1 text-2xl font-semibold tracking-tight text-foreground">
                                    "Set up your workspace"
                                </h2>
                                <p class="mt-1 text-sm leading-6 text-muted-foreground">
                                    "Finish these steps to ship your first Proa-powered experience."
                                </p>
                            </div>
                            <div class="shrink-0 rounded-full border border-border bg-background px-3 py-1.5 text-xs font-medium text-muted-foreground shadow-xs">
                                <span class="font-semibold text-foreground">"1"</span>
                                " of 4"
                            </div>
                        </div>
                        <div class="mb-6">
                            {Progress {
                                value: 25,
                                aria_label: Some("Workspace setup progress: 25 percent"),
                                class: Some("h-1.5"),
                                ..Progress::new()
                            }}
                        </div>
                        <div class="space-y-3">
                            {for step in STEPS {
                                html_sync! {
                                    {Card { class: Some(step.state.card()), children: Some(html_sync! {
                                        <div class="flex gap-4 p-4 sm:p-5">
                                            {match step.state {
                                                StepState::Complete => html_sync! {
                                                    <div class="flex size-8 shrink-0 items-center justify-center rounded-full bg-emerald-500 text-white shadow-xs">
                                                        <svg class="size-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                                                            <path d="m5 12 4 4L19 6" />
                                                        </svg>
                                                    </div>
                                                }.render(cx)?,
                                                StepState::Current => html_sync! {
                                                    <div class="flex size-8 shrink-0 items-center justify-center rounded-full bg-primary text-sm font-semibold text-primary-foreground shadow-xs">
                                                        {step.numeral}
                                                    </div>
                                                }.render(cx)?,
                                                StepState::Upcoming => html_sync! {
                                                    <div class="flex size-8 shrink-0 items-center justify-center rounded-full border border-border bg-muted text-sm font-semibold text-muted-foreground">
                                                        {step.numeral}
                                                    </div>
                                                }.render(cx)?,
                                            }}
                                            <div class="min-w-0 flex-1">
                                                {if step.state == StepState::Complete {
                                                    html_sync! {
                                                        <div class="flex items-center justify-between gap-3">
                                                            <p class="font-medium text-muted-foreground line-through decoration-muted-foreground/40">
                                                                {step.title}
                                                            </p>
                                                            <span class="text-xs font-medium text-emerald-600 dark:text-emerald-400">
                                                                "Complete"
                                                            </span>
                                                        </div>
                                                        <p class="mt-1 text-sm leading-5 text-muted-foreground/80">
                                                            {step.summary}
                                                        </p>
                                                    }.render(cx)?;
                                                } else if step.state == StepState::Current {
                                                    html_sync! {
                                                        <div class="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
                                                            <div>
                                                                <p class="font-semibold text-foreground">
                                                                    {step.title}
                                                                </p>
                                                                <p class="mt-1 max-w-sm text-sm leading-5 text-muted-foreground">
                                                                    {step.summary}
                                                                </p>
                                                            </div>
                                                            {Button { size: Some(ButtonSize::Sm), class: Some("shrink-0"), children: Some("View setup"), ..Default::default() }}
                                                        </div>
                                                    }.render(cx)?;
                                                } else {
                                                    html_sync! {
                                                        <p class="font-medium text-foreground">{step.title}</p>
                                                        <p class="mt-1 text-sm leading-5 text-muted-foreground">
                                                            {step.summary}
                                                        </p>
                                                    }.render(cx)?;
                                                }}
                                                {if let Some(command) = step.command {
                                                    html_sync! {
                                                        <div class="mt-4 rounded-lg border border-border bg-background px-3 py-2 font-mono text-xs text-foreground shadow-inner">
                                                            <span class="select-none text-muted-foreground">"$ "</span>
                                                            {command}
                                                        </div>
                                                    }.render(cx)?;
                                                }}
                                            </div>
                                            {if let Some(icon) = step.icon {
                                                html_sync! {
                                                    <svg class="mt-1 size-5 shrink-0 text-muted-foreground/60" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                                                        {StaticRaw(icon)}
                                                    </svg>
                                                }.render(cx)?;
                                            }}
                                        </div>
                                    }), }}
                                }
                            }}
                        </div>
                        <p class="mt-5 text-center text-xs text-muted-foreground">
                            "Setup progress is saved automatically."
                        </p>
                    </div>
                </div>
            </div>
        }
        .render(cx)
    }
}
