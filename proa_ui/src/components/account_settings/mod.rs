// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Adapted from shadcn/ui
//! `apps/v4/registry/new-york-v4/blocks/sidebar-13/components/settings-dialog.tsx` at
//! https://github.com/shadcn-ui/ui, commit
//! fa4872c8ed948c111884e52ae23a649e83591b71. MIT License, Copyright (c) 2023
//! shadcn; see the upstream `LICENSE.md`.

use proa_core::StaticRaw;
use proa_macros::html_sync;
use crate::components::{
    Button, ButtonColor, ButtonVariant, Card, CardContent, CardDescription, CardHeader, CardTitle,
    Input, InputType, Label, Switch, Textarea,
};

struct SettingsSection {
    href: &'static str,
    label: &'static str,
    /// Inner markup for the 24×24 icon.
    icon: &'static str,
    current: bool,
}

const SECTIONS: &[SettingsSection] = &[
    SettingsSection {
        href: "/blocks/account_settings",
        label: "Profile",
        icon: r#"<circle cx="12" cy="8" r="4" /><path d="M4 21a8 8 0 0 1 16 0" />"#,
        current: true,
    },
    SettingsSection {
        href: "/blocks/account_settings",
        label: "Notifications",
        icon: r#"<path d="M18 8a6 6 0 0 0-12 0c0 7-3 7-3 9h18c0-2-3-2-3-9" /><path d="M10 21h4" />"#,
        current: false,
    },
    SettingsSection {
        href: "/blocks/account_settings",
        label: "Appearance",
        icon: r#"<path d="M12 3a9 9 0 1 0 9 9" /><path d="M12 3v9h9" />"#,
        current: false,
    },
    SettingsSection {
        href: "/pricing",
        label: "Billing",
        icon: r#"<rect x="3" y="5" width="18" height="14" rx="2" /><path d="M3 10h18" />"#,
        current: false,
    },
    SettingsSection {
        href: "/blocks/account_settings",
        label: "Security",
        icon: r#"<rect x="5" y="11" width="14" height="10" rx="2" /><path d="M8 11V7a4 4 0 0 1 8 0v4" />"#,
        current: false,
    },
];

struct Preference {
    title: &'static str,
    summary: &'static str,
    enabled: bool,
}

const PREFERENCES: &[Preference] = &[
    Preference {
        title: "Deployment summaries",
        summary: "A weekly digest of production activity and performance.",
        enabled: true,
    },
    Preference {
        title: "Product announcements",
        summary: "New components, blocks, and Proa releases.",
        enabled: true,
    },
    Preference {
        title: "Research invitations",
        summary: "Occasional opportunities to shape upcoming features.",
        enabled: false,
    },
];

/// Source-installed `account_settings` block composition.
pub struct AccountSettingsBlock;

impl<B: proa_core::WriteBuf, L: proa_core::DataLoader> proa_core::WebRenderSync<L, B> for AccountSettingsBlock {
    fn render(
        self,
        cx: &mut proa_core::WebContext<L, B>,
    ) -> Result<(), proa_core::WriteError> {
        html_sync! {
            <div class="flex flex-col gap-6">
                <div class="overflow-hidden rounded-xl border border-border bg-background shadow-xs">
                    <div class="border-b border-border px-5 py-5 sm:px-8">
                        <div class="flex items-center gap-2 text-sm text-muted-foreground">
                            <a
                                href="/blocks/dashboard_overview"
                                class="transition-colors hover:text-foreground"
                            >
                                "Workspace"
                            </a>
                            <span aria-hidden="true">"/"</span>
                            <span class="text-foreground">"Settings"</span>
                        </div>
                        <div class="mt-3 flex flex-col justify-between gap-4 sm:flex-row sm:items-end">
                            <div>
                                <h2 class="text-2xl font-semibold tracking-tight text-foreground">
                                    "Settings"
                                </h2>
                                <p class="mt-1 text-sm text-muted-foreground">
                                    "Manage your profile and workspace preferences."
                                </p>
                            </div>
                            <span class="inline-flex w-fit items-center gap-2 rounded-full border border-border bg-muted/50 px-3 py-1 text-xs text-muted-foreground">
                                <span class="size-1.5 rounded-full bg-emerald-500"></span>
                                "All changes saved"
                            </span>
                        </div>
                    </div>
                    <div class="grid md:grid-cols-[220px_minmax(0,1fr)]">
                        <aside class="border-b border-border bg-muted/20 p-3 md:border-b-0 md:border-r md:p-5">
                            <nav
                                aria-label="Settings sections"
                                class="flex gap-1 overflow-x-auto md:flex-col"
                            >
                                {for section in SECTIONS {
                                    let classes = if section.current {
                                        "flex shrink-0 items-center gap-3 rounded-md bg-accent px-3 py-2 text-sm font-medium text-accent-foreground"
                                    } else {
                                        "flex shrink-0 items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
                                    };

                                    html_sync! {
                                        <a href={section.href} aria-current={section.current.then_some("page")} class={classes}>
                                            <svg aria-hidden="true" viewBox="0 0 24 24" class="size-4" fill="none" stroke="currentColor" stroke-width="2">
                                                {StaticRaw(section.icon)}
                                            </svg>
                                            {section.label}
                                        </a>
                                    }
                                }}
                            </nav>
                        </aside>
                        <div class="min-w-0 bg-muted/10 p-4 sm:p-8">
                            <div class="mx-auto max-w-3xl space-y-6">
                                {Card {
                                    class: Some("shadow-none"),
                                    children: Some(html_sync! {
                                        {CardHeader {
                                            children: Some(html_sync! {
                                                {CardTitle {
                                                    class: None,
                                                    children: Some("Public profile"),
                                                }}
                                                {CardDescription {
                                                    class: None,
                                                    children: Some("This information appears across your team workspace."),
                                                }}
                                            }),
                                            ..Default::default()
                                        }}
                                        {CardContent {
                                            class: Some("space-y-6"),
                                            children: Some(html_sync! {
                                                <div class="flex flex-col gap-5 sm:flex-row sm:items-center">
                                                    <div class="flex size-20 shrink-0 items-center justify-center rounded-full bg-primary/10 text-xl font-semibold text-primary ring-4 ring-background">
                                                        "AH"
                                                    </div>
                                                    <div class="space-y-2">
                                                        <div class="flex flex-wrap gap-2">
                                                            {Button {
                                                                variant: Some(ButtonVariant::Outline),
                                                                children: Some("Upload photo"),
                                                                ..Default::default()
                                                            }}
                                                            {Button {
                                                                variant: Some(ButtonVariant::Ghost),
                                                                class: Some("text-muted-foreground"),
                                                                children: Some("Remove"),
                                                                ..Default::default()
                                                            }}
                                                        </div>
                                                        <p class="text-xs text-muted-foreground">
                                                            "JPG, GIF, or PNG. 2 MB maximum."
                                                        </p>
                                                    </div>
                                                </div>
                                                <div class="grid gap-5 sm:grid-cols-2">
                                                    <div class="space-y-2">
                                                        {Label {
                                                            for_id: Some("settings-name"),
                                                            text: "Full name",
                                                            ..Label::new()
                                                        }}
                                                        {Input {
                                                            id: Some("settings-name"),
                                                            name: Some("name"),
                                                            value: Some("Alex Harper"),
                                                            autocomplete: Some("name"),
                                                            ..Input::new()
                                                        }}
                                                    </div>
                                                    <div class="space-y-2">
                                                        {Label {
                                                            for_id: Some("settings-role"),
                                                            text: "Role",
                                                            ..Label::new()
                                                        }}
                                                        {Input {
                                                            id: Some("settings-role"),
                                                            name: Some("role"),
                                                            value: Some("Product engineer"),
                                                            ..Input::new()
                                                        }}
                                                    </div>
                                                    <div class="space-y-2 sm:col-span-2">
                                                        {Label {
                                                            for_id: Some("settings-email"),
                                                            text: "Email address",
                                                            ..Label::new()
                                                        }}
                                                        {Input {
                                                            id: Some("settings-email"),
                                                            name: Some("email"),
                                                            input_type: InputType::Email,
                                                            value: Some("alex@proa.dev"),
                                                            autocomplete: Some("email"),
                                                            ..Input::new()
                                                        }}
                                                        <p class="text-xs text-muted-foreground">
                                                            "We use this address for account notifications."
                                                        </p>
                                                    </div>
                                                    <div class="space-y-2 sm:col-span-2">
                                                        {Label {
                                                            for_id: Some("settings-bio"),
                                                            text: "Bio",
                                                            ..Label::new()
                                                        }}
                                                        {Textarea {
                                                            id: Some("settings-bio"),
                                                            name: Some("bio"),
                                                            value: Some("Building fast, accessible products with Rust and Proa."),
                                                            rows: Some("4"),
                                                            class: Some("resize-none"),
                                                            ..Textarea::new()
                                                        }}
                                                        <p class="text-xs text-muted-foreground">
                                                            "A short introduction for teammates who visit your profile."
                                                        </p>
                                                    </div>
                                                </div>
                                            }),
                                        }}
                                        <div class="flex justify-end border-t border-border px-6 py-4">
                                            {Button {
                                                children: Some("Save profile"),
                                                ..Default::default()
                                            }}
                                        </div>
                                    }),
                                }}
                                {Card {
                                    class: Some("shadow-none"),
                                    children: Some(html_sync! {
                                        {CardHeader {
                                            children: Some(html_sync! {
                                                {CardTitle {
                                                    class: None,
                                                    children: Some("Email preferences"),
                                                }}
                                                {CardDescription {
                                                    class: None,
                                                    children: Some("Choose which workspace updates reach your inbox."),
                                                }}
                                            }),
                                            ..Default::default()
                                        }}
                                        {CardContent {
                                            class: Some("divide-y divide-border"),
                                            children: Some(html_sync! {
                                                {for (index, preference) in PREFERENCES.iter().enumerate() {
                                                    // First and last rows trim the padding that
                                                    // `divide-y` would otherwise double up against
                                                    // the card's own edges.
                                                    let spacing = if index == 0 {
                                                        "flex items-center justify-between gap-6 pb-5"
                                                    } else if index + 1 == PREFERENCES.len() {
                                                        "flex items-center justify-between gap-6 pt-5"
                                                    } else {
                                                        "flex items-center justify-between gap-6 py-5"
                                                    };

                                                    html_sync! {
                                                        <div class={spacing}>
                                                            <div>
                                                                <p class="text-sm font-medium text-foreground">
                                                                    {preference.title}
                                                                </p>
                                                                <p class="mt-1 text-sm text-muted-foreground">
                                                                    {preference.summary}
                                                                </p>
                                                            </div>
                                                            {Switch {
                                                                checked: preference.enabled,
                                                                aria_label: Some(preference.title),
                                                                ..Switch::new()
                                                            }}
                                                        </div>
                                                    }
                                                }}
                                            }),
                                        }}
                                    }),
                                }}
                                {Card {
                                    class: Some("border-destructive/30 shadow-none"),
                                    children: Some(html_sync! {
                                        {CardHeader {
                                            children: Some(html_sync! {
                                                {CardTitle {
                                                    class: None,
                                                    children: Some("Delete account"),
                                                }}
                                                {CardDescription {
                                                    class: None,
                                                    children: Some("Permanently remove your personal account and revoke its access."),
                                                }}
                                            }),
                                            ..Default::default()
                                        }}
                                        {CardContent {
                                            class: Some("flex flex-col items-start justify-between gap-4 sm:flex-row sm:items-center"),
                                            children: Some(html_sync! {
                                                <p class="max-w-xl text-sm text-muted-foreground">
                                                    "Your shared projects stay with the workspace. This action cannot be undone."
                                                </p>
                                                {Button {
                                                    color: Some(ButtonColor::Destructive),
                                                    class: Some("shrink-0"),
                                                    children: Some("Delete account"),
                                                    ..Default::default()
                                                }}
                                            }),
                                        }}
                                    }),
                                }}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
        .render(cx)
    }
}
