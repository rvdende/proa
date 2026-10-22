// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Adapted from shadcn/ui `apps/v4/registry/new-york-v4/blocks/signup-03/page.tsx`
//! and `components/signup-form.tsx` at https://github.com/shadcn-ui/ui, commit
//! fa4872c8ed948c111884e52ae23a649e83591b71. MIT License, Copyright (c) 2023
//! shadcn; see the upstream `LICENSE.md`.

use proa_macros::html_sync;
use crate::components::{
    Button, ButtonType, ButtonVariant, Card, CardContent, CardDescription, CardHeader, CardTitle,
    Checkbox, Input, InputType, Label,
};

/// Reassurances under the form. All three render identically, so they are copy
/// rather than markup.
const TRUST_SIGNALS: &[&str] = &["No credit card", "SOC 2 controls", "Cancel anytime"];

/// Source-installed `signup` block composition.
pub struct SignupBlock;

impl<B: proa_core::WriteBuf, L: proa_core::DataLoader> proa_core::WebRenderSync<L, B> for SignupBlock {
    fn render(
        self,
        cx: &mut proa_core::WebContext<L, B>,
    ) -> Result<(), proa_core::WriteError> {
        html_sync! {
            <div class="flex flex-col gap-6">
                <div class="relative isolate overflow-hidden rounded-xl border border-border bg-muted/35 px-5 py-12 sm:px-8 sm:py-16">
                    <div
                        aria-hidden="true"
                        class="absolute left-1/2 top-0 -z-10 h-72 w-72 -translate-x-1/2 -translate-y-1/2 rounded-full bg-primary/10 blur-3xl"
                    ></div>
                    <div class="mx-auto flex w-full max-w-md flex-col gap-6">
                        <a
                            href="/blocks/signup"
                            class="flex items-center justify-center gap-2.5 text-sm font-semibold text-foreground"
                        >
                            <span class="flex size-8 items-center justify-center rounded-lg bg-primary text-primary-foreground shadow-xs">
                                <svg
                                    aria-hidden="true"
                                    viewBox="0 0 24 24"
                                    class="size-4"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                >
                                    <path d="M4 18 10 6l4 8 2-4 4 8" />
                                </svg>
                            </span>
                            "Proa Cloud"
                        </a>
                        {Card {
                            class: Some("border-border/80 shadow-lg shadow-black/5"),
                            children: Some(html_sync! {
                                {CardHeader {
                                    class: Some("text-center"),
                                    children: Some(html_sync! {
                                        {CardTitle {
                                            class: Some("text-2xl"),
                                            children: Some("Create your workspace"),
                                        }}
                                        {CardDescription {
                                            class: Some("text-balance"),
                                            children: Some("Ship your first Proa app with a secure team workspace."),
                                        }}
                                    }),
                                }}
                                {CardContent {
                                    children: Some(html_sync! {
                                        <form class="space-y-5">
                                            <div class="grid gap-4 sm:grid-cols-2">
                                                <div class="space-y-2">
                                                    {Label {
                                                        for_id: Some("signup-first-name"),
                                                        text: "First name",
                                                        ..Label::new()
                                                    }}
                                                    {Input {
                                                        id: Some("signup-first-name"),
                                                        name: Some("first_name"),
                                                        placeholder: Some("Alex"),
                                                        autocomplete: Some("given-name"),
                                                        required: true,
                                                        ..Input::new()
                                                    }}
                                                </div>
                                                <div class="space-y-2">
                                                    {Label {
                                                        for_id: Some("signup-last-name"),
                                                        text: "Last name",
                                                        ..Label::new()
                                                    }}
                                                    {Input {
                                                        id: Some("signup-last-name"),
                                                        name: Some("last_name"),
                                                        placeholder: Some("Harper"),
                                                        autocomplete: Some("family-name"),
                                                        required: true,
                                                        ..Input::new()
                                                    }}
                                                </div>
                                            </div>
                                            <div class="space-y-2">
                                                {Label {
                                                    for_id: Some("signup-email"),
                                                    text: "Work email",
                                                    ..Label::new()
                                                }}
                                                {Input {
                                                    id: Some("signup-email"),
                                                    name: Some("email"),
                                                    input_type: InputType::Email,
                                                    placeholder: Some("alex@company.com"),
                                                    autocomplete: Some("email"),
                                                    required: true,
                                                    ..Input::new()
                                                }}
                                            </div>
                                            <div class="space-y-2">
                                                <div class="flex items-center justify-between gap-3">
                                                    {Label {
                                                        for_id: Some("signup-password"),
                                                        text: "Password",
                                                        ..Label::new()
                                                    }}
                                                    <span class="text-xs text-muted-foreground">
                                                        "8+ characters"
                                                    </span>
                                                </div>
                                                {Input {
                                                    id: Some("signup-password"),
                                                    name: Some("password"),
                                                    input_type: InputType::Password,
                                                    autocomplete: Some("new-password"),
                                                    minlength: Some("8"),
                                                    required: true,
                                                    ..Input::new()
                                                }}
                                            </div>
                                            <div class="flex items-start gap-3 rounded-lg border border-border bg-muted/30 p-3.5">
                                                {Checkbox {
                                                    id: Some("signup-terms"),
                                                    name: Some("terms"),
                                                    value: Some("accepted"),
                                                    required: true,
                                                    ..Checkbox::new()
                                                }}
                                                <div class="text-sm leading-5 text-muted-foreground">
                                                    {Label {
                                                        for_id: Some("signup-terms"),
                                                        text: "I agree to the Terms and Privacy Policy",
                                                        class: Some("font-normal leading-5"),
                                                        ..Label::new()
                                                    }}
                                                    <p class="mt-0.5 text-xs">
                                                        "You can manage product emails in settings."
                                                    </p>
                                                </div>
                                            </div>
                                            {Button {
                                                class: Some("w-full"),
                                                children: Some("Create workspace"),
                                                ..Default::default()
                                            }}
                                        </form>
                                        <div class="mt-6 flex items-center gap-3">
                                            <span class="h-px flex-1 bg-border"></span>
                                            <span class="text-xs uppercase tracking-wider text-muted-foreground">
                                                "or"
                                            </span>
                                            <span class="h-px flex-1 bg-border"></span>
                                        </div>
                                        {Button {
                                            size: None,
                                            color: None,
                                            variant: Some(ButtonVariant::Outline),
                                            radius: None,
                                            disabled: false.into(),
                                            loading: false.into(),
                                            button_type: ButtonType::Button,
                                            class: Some("mt-6 w-full"),
                                            id: None,
                                            children: Some(html_sync! {
                                                <svg
                                                    aria-hidden="true"
                                                    viewBox="0 0 24 24"
                                                    class="size-4"
                                                    fill="currentColor"
                                                >
                                                    <path d="M12 2a10 10 0 0 0-3.16 19.49c.5.09.68-.22.68-.48v-1.87c-2.78.6-3.37-1.18-3.37-1.18-.45-1.16-1.11-1.47-1.11-1.47-.91-.62.07-.61.07-.61 1 .07 1.53 1.03 1.53 1.03.9 1.53 2.35 1.09 2.92.83.09-.65.35-1.09.64-1.34-2.22-.25-4.56-1.11-4.56-4.94 0-1.09.39-1.98 1.03-2.68-.1-.25-.45-1.27.1-2.64 0 0 .84-.27 2.75 1.02A9.6 9.6 0 0 1 12 7.88c.85 0 1.71.12 2.5.34 1.91-1.29 2.75-1.02 2.75-1.02.55 1.37.2 2.39.1 2.64.64.7 1.03 1.59 1.03 2.68 0 3.84-2.34 4.68-4.57 4.93.36.31.68.92.68 1.85v1.71c0 .27.18.58.69.48A10 10 0 0 0 12 2Z" />
                                                </svg>
                                                "Continue with GitHub"
                                            }),
                                            onclick: None,
                                            onblur: None,
                                            onfocus: None,
                                        }}
                                    }),
                                    ..Default::default()
                                }}
                            }),
                        }}
                        <div class="flex items-center justify-center gap-3 text-sm text-muted-foreground">
                            <span>"Already have an account?"</span>
                            <a
                                href="/blocks/login"
                                class="font-medium text-foreground underline-offset-4 hover:underline"
                            >
                                "Sign in"
                            </a>
                        </div>
                        <div class="flex flex-wrap items-center justify-center gap-x-4 gap-y-2 text-xs text-muted-foreground">
                            {for signal in TRUST_SIGNALS {
                                html_sync! {
                                    <span class="inline-flex items-center gap-1.5">
                                        <svg
                                            aria-hidden="true"
                                            viewBox="0 0 24 24"
                                            class="size-3.5 text-emerald-500"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2.5"
                                        >
                                            <path d="m5 12 4 4L19 6" />
                                        </svg>
                                        {*signal}
                                    </span>
                                }
                            }}
                        </div>
                    </div>
                </div>
            </div>
        }
        .render(cx)
    }
}
