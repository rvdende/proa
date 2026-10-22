// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Provenance: adapted from `content/components/file-upload/file-upload-06.tsx` in
//! https://github.com/ephraimduncan/blocks at commit
//! fe4305a1147355f5cd1ca9cb03b6fa88f013bd6a. MIT License, Copyright (c) 2025
//! Ephraim Duncan; see the upstream `LICENSE.md`.

use proa_macros::html_sync;
use crate::components::{
    Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Card, Input, InputType, Progress,
    Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
};

struct Asset {
    extension: &'static str,
    /// File-type chip tint, spelled out for Tailwind's source scan.
    tint: &'static str,
    name: &'static str,
    kind: &'static str,
    owner: &'static str,
    size: &'static str,
    updated: &'static str,
}

const RECENT_FILES: &[Asset] = &[
    Asset {
        extension: "FIG",
        tint: "bg-violet-500/10 text-violet-700 dark:text-violet-300",
        name: "component-library.fig",
        kind: "Design source",
        owner: "Mina Patel",
        size: "18.4 MB",
        updated: "8 min ago",
    },
    Asset {
        extension: "PNG",
        tint: "bg-sky-500/10 text-sky-700 dark:text-sky-300",
        name: "dashboard-preview.png",
        kind: "Product image",
        owner: "Samira Khan",
        size: "3.2 MB",
        updated: "2 hr ago",
    },
    Asset {
        extension: "ZIP",
        tint: "bg-amber-500/10 text-amber-700 dark:text-amber-300",
        name: "brand-assets-v4.zip",
        kind: "Archive",
        owner: "Avery Morgan",
        size: "42.7 MB",
        updated: "Yesterday",
    },
];

struct Upload {
    extension: &'static str,
    name: &'static str,
    percent: u8,
    display: &'static str,
    aria_label: &'static str,
    transferred: &'static str,
}

const IN_FLIGHT: &[Upload] = &[
    Upload {
        extension: "PNG",
        name: "checkout-flow.png",
        percent: 72,
        display: "72%",
        aria_label: "checkout-flow.png upload: 72 percent",
        transferred: "5.8 MB of 8.1 MB",
    },
    Upload {
        extension: "MOV",
        name: "product-tour.mov",
        percent: 38,
        display: "38%",
        aria_label: "product-tour.mov upload: 38 percent",
        transferred: "18.6 MB of 49.0 MB",
    },
];

/// Source-installed `file_manager` block composition.
pub struct FileManagerBlock;

impl<B: proa_core::WriteBuf, L: proa_core::DataLoader> proa_core::WebRenderSync<L, B> for FileManagerBlock {
    fn render(
        self,
        cx: &mut proa_core::WebContext<L, B>,
    ) -> Result<(), proa_core::WriteError> {
        html_sync! {
            <div class="flex flex-col gap-6">
                <div class="overflow-hidden rounded-2xl border border-border bg-background shadow-xs">
                    <div class="flex flex-col gap-4 border-b border-border p-5 sm:flex-row sm:items-center sm:justify-between sm:p-7">
                        <div>
                            <div class="flex items-center gap-2 text-sm text-muted-foreground">
                                <a href="/blocks" class="hover:text-foreground">"Workspace"</a>
                                <span aria-hidden="true">"/"</span>
                                <span class="font-medium text-foreground">"Product assets"</span>
                            </div>
                            <h2 class="mt-2 text-xl font-semibold tracking-tight text-foreground">
                                "Product assets"
                            </h2>
                            <p class="mt-1 text-sm text-muted-foreground">
                                "24 files · Updated 8 minutes ago"
                            </p>
                        </div>
                        <div class="flex items-center gap-2">
                            {Button { size: Some(ButtonSize::Sm), variant: Some(ButtonVariant::Outline), children: Some("New folder"), ..Default::default() }}
                            {Button { size: Some(ButtonSize::Sm), children: Some("Upload files"), ..Default::default() }}
                        </div>
                    </div>
                    <div class="grid gap-px bg-border lg:grid-cols-[minmax(0,1.45fr)_minmax(320px,0.7fr)]">
                        <div class="bg-card p-5 sm:p-7">
                            {Card { class: Some("border-2 border-dashed border-border bg-muted/20 py-0 shadow-none transition-colors hover:border-primary/40 hover:bg-primary/[0.03]"), children: Some(html_sync! {
                                <div class="flex min-h-60 flex-col items-center justify-center p-6 text-center">
                                    <div class="flex size-12 items-center justify-center rounded-2xl border border-border bg-background text-muted-foreground shadow-xs">
                                        <svg class="size-6" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                                            <path d="M12 16V4M7 9l5-5 5 5" />
                                            <path d="M20 15v4a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-4" />
                                        </svg>
                                    </div>
                                    <h3 class="mt-4 font-semibold text-foreground">
                                        "Drop files to upload"
                                    </h3>
                                    <p class="mt-1 max-w-sm text-sm leading-6 text-muted-foreground">
                                        "Add product imagery, documents, or source archives. Individual files can be up to 50 MB."
                                    </p>
                                    <div class="mt-4 w-full max-w-xs">
                                        {Input { input_type: InputType::File, aria_label: Some("Choose files to upload"), class: Some("cursor-pointer bg-background text-sm file:mr-3 file:border-0 file:bg-transparent file:font-medium"), ..Input::new() }}
                                    </div>
                                </div>
                            }), }}
                            <div class="mt-7 flex items-center justify-between gap-4">
                                <div>
                                    <h3 class="font-semibold text-foreground">"Recent files"</h3>
                                    <p class="mt-1 text-sm text-muted-foreground">
                                        "Files changed by your team this week"
                                    </p>
                                </div>
                                {Button { size: Some(ButtonSize::Sm), variant: Some(ButtonVariant::Ghost), children: Some("View all"), ..Default::default() }}
                            </div>
                            <div class="mt-4 overflow-hidden rounded-xl border border-border">
                                {Table { class: None, children: Some(html_sync! {
                                    {TableHeader { class: None, children: Some(html_sync! {
                                        {TableRow { class: Some("bg-muted/50 hover:bg-muted/50"), children: Some(html_sync! {
                                            {TableHead { class: None, children: Some("Name") }}
                                            {TableHead { class: None, children: Some("Owner") }}
                                            {TableHead { class: Some("text-right"), children: Some("Size") }}
                                            {TableHead { class: Some("text-right"), children: Some("Updated") }}
                                        }), }}
                                    }), }}
                                    {TableBody { class: None, children: Some(html_sync! {
                                        {for asset in RECENT_FILES {
                                            html_sync! {
                                                {TableRow { class: None, children: Some(html_sync! {
                                                    {TableCell { class: None, children: Some(html_sync! {
                                                        <div class="flex items-center gap-3">
                                                            <span class={proa_macros::text!("flex size-9 items-center justify-center rounded-lg text-xs font-semibold {}", asset.tint)}>
                                                                {asset.extension}
                                                            </span>
                                                            <div>
                                                                <p class="font-medium text-foreground">{asset.name}</p>
                                                                <p class="mt-0.5 text-xs text-muted-foreground">{asset.kind}</p>
                                                            </div>
                                                        </div>
                                                    }), }}
                                                    {TableCell { class: Some("text-muted-foreground"), children: Some(asset.owner) }}
                                                    {TableCell { class: Some("text-right text-muted-foreground tabular-nums"), children: Some(asset.size) }}
                                                    {TableCell { class: Some("text-right text-muted-foreground"), children: Some(asset.updated) }}
                                                }), }}
                                            }
                                        }}
                                    }), }}
                                }), }}
                            </div>
                        </div>
                        <aside class="bg-card p-5 sm:p-7" aria-label="Upload and storage status">
                            <div class="flex items-center justify-between gap-3">
                                <h3 class="font-semibold text-foreground">"Uploads"</h3>
                                {Badge { variant: BadgeVariant::Secondary, class: Some("rounded-full shadow-none"), children: Some("2 active"), }}
                            </div>
                            <div class="mt-5 space-y-4">
                                {for upload in IN_FLIGHT {
                                    html_sync! {
                                        {Card { class: Some("gap-0 bg-background p-4 py-4 shadow-none"), children: Some(html_sync! {
                                            <div class="flex items-start gap-3">
                                                <span class="flex size-9 shrink-0 items-center justify-center rounded-lg bg-muted text-xs font-semibold text-muted-foreground">
                                                    {upload.extension}
                                                </span>
                                                <div class="min-w-0 flex-1">
                                                    <div class="flex items-start justify-between gap-3">
                                                        <p class="truncate text-sm font-medium text-foreground">
                                                            {upload.name}
                                                        </p>
                                                        <span class="text-xs tabular-nums text-muted-foreground">
                                                            {upload.display}
                                                        </span>
                                                    </div>
                                                    <div class="mt-3">
                                                        {Progress { value: upload.percent, aria_label: Some(upload.aria_label), class: Some("h-1.5"), ..Progress::new() }}
                                                    </div>
                                                    <p class="mt-2 text-xs text-muted-foreground">
                                                        {upload.transferred}
                                                    </p>
                                                </div>
                                            </div>
                                        }), }}
                                    }
                                }}
                                // The finished upload is a different card, not a
                                // hundred-percent variant of the two above: no bar, no
                                // byte count, and a success frame.
                                {Card { class: Some("gap-0 border-emerald-500/20 bg-emerald-500/[0.04] p-4 py-4 shadow-none"), children: Some(html_sync! {
                                    <div class="flex items-center gap-3">
                                        <span class="flex size-9 shrink-0 items-center justify-center rounded-lg bg-emerald-500/10 text-emerald-700 dark:text-emerald-400">
                                            <svg class="size-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                                                <path d="m5 12 4 4L19 6" />
                                            </svg>
                                        </span>
                                        <div class="min-w-0 flex-1">
                                            <p class="truncate text-sm font-medium text-foreground">
                                                "logo-marks.svg"
                                            </p>
                                            <p class="mt-0.5 text-xs text-emerald-700 dark:text-emerald-400">
                                                "Upload complete"
                                            </p>
                                        </div>
                                    </div>
                                }), }}
                            </div>
                            <div class="mt-7 border-t border-border pt-6">
                                <div class="flex items-center justify-between text-sm">
                                    <span class="font-medium text-foreground">"Storage"</span>
                                    <span class="tabular-nums text-muted-foreground">
                                        "42.6 GB of 100 GB"
                                    </span>
                                </div>
                                <div class="mt-3">
                                    {Progress { value: 43, aria_label: Some("Storage used: 43 percent"), class: Some("h-2"), ..Progress::new() }}
                                </div>
                                <p class="mt-2 text-xs text-muted-foreground">
                                    "57.4 GB available on your workspace plan."
                                </p>
                            </div>
                        </aside>
                    </div>
                </div>
            </div>
        }
        .render(cx)
    }
}
