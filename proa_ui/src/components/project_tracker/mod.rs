// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Provenance: adapted from `content/components/tables/table-04.tsx` in
//! https://github.com/ephraimduncan/blocks at commit
//! fe4305a1147355f5cd1ca9cb03b6fa88f013bd6a. MIT License, Copyright (c) 2025
//! Ephraim Duncan; see the upstream `LICENSE.md`.

use proa_core::WebRenderSync;
use proa_macros::html_sync;
use crate::components::{
    Avatar, AvatarFallback, AvatarImage, AvatarShape, AvatarSize, Badge, BadgeVariant, Button,
    ButtonSize, ButtonVariant, Progress, Table, TableBody, TableCell, TableHead, TableHeader,
    TableRow,
};

/// How many columns a group heading has to pad out. Kept next to the header
/// definition so adding a column cannot leave the group rows short and collapse
/// the table's cell alignment.
const COLUMNS: usize = 5;

#[derive(Clone, Copy)]
enum Status {
    InProgress,
    Completed,
    Planning,
    AtRisk,
}

impl Status {
    fn label(self) -> &'static str {
        match self {
            Self::InProgress => "In progress",
            Self::Completed => "Completed",
            Self::Planning => "Planning",
            Self::AtRisk => "At risk",
        }
    }

    fn variant(self) -> BadgeVariant {
        match self {
            Self::InProgress => BadgeVariant::Outline,
            Self::Completed => BadgeVariant::Success,
            Self::Planning => BadgeVariant::Secondary,
            Self::AtRisk => BadgeVariant::Warning,
        }
    }

    fn class(self) -> &'static str {
        match self {
            Self::InProgress => {
                "border-0 bg-blue-500/10 text-blue-700 shadow-none dark:text-blue-400"
            }
            _ => "shadow-none",
        }
    }
}

struct Owner {
    initials: &'static str,
    /// Avatar fill. Written out in full so Tailwind's source scan sees it.
    tint: &'static str,
}

impl Owner {
    fn avatar(&self) -> Avatar<AvatarImage, AvatarFallback<&'static str>> {
        Avatar {
            size: AvatarSize::Sm,
            shape: AvatarShape::Circle,
            class: Some("border-2 border-background"),
            image: None,
            fallback: Some(AvatarFallback {
                size: AvatarSize::Sm,
                class: Some(self.tint),
                children: Some(self.initials),
            }),
        }
    }
}

struct Project {
    name: &'static str,
    summary: &'static str,
    budget: &'static str,
    deadline: &'static str,
    owners: &'static [Owner],
    status: Status,
}

/// The table interleaves team headings with the projects under them, so one
/// ordered list of rows keeps that grouping readable in source order.
enum Row {
    Group(&'static str),
    Project(Project),
}

const ROWS: &[Row] = &[
    Row::Group("Engineering · 3"),
    Row::Project(Project {
        name: "Registry v2 rollout",
        summary: "Unified package delivery",
        budget: "$84,000",
        deadline: "Jul 28",
        owners: &[
            Owner {
                initials: "JL",
                tint: "bg-sky-500 text-[10px] font-semibold text-white",
            },
            Owner {
                initials: "AM",
                tint: "bg-violet-500 text-[10px] font-semibold text-white",
            },
        ],
        status: Status::InProgress,
    }),
    Row::Project(Project {
        name: "Edge cache migration",
        summary: "Reduce global render latency",
        budget: "$46,500",
        deadline: "Aug 12",
        owners: &[Owner {
            initials: "RC",
            tint: "bg-emerald-500 text-[10px] font-semibold text-white",
        }],
        status: Status::Completed,
    }),
    Row::Group("Product & Design · 2"),
    Row::Project(Project {
        name: "Theme Studio",
        summary: "Token editing and previews",
        budget: "$62,000",
        deadline: "Aug 30",
        owners: &[
            Owner {
                initials: "SK",
                tint: "bg-amber-500 text-[10px] font-semibold text-white",
            },
            Owner {
                initials: "MP",
                tint: "bg-rose-500 text-[10px] font-semibold text-white",
            },
        ],
        status: Status::Planning,
    }),
    Row::Project(Project {
        name: "Accessibility audit",
        summary: "Priority component review",
        budget: "$28,000",
        deadline: "Sep 10",
        owners: &[Owner {
            initials: "DT",
            tint: "bg-fuchsia-500 text-[10px] font-semibold text-white",
        }],
        status: Status::AtRisk,
    }),
];

/// Source-installed `project_tracker` block composition.
pub struct ProjectTrackerBlock;

impl<B: proa_core::WriteBuf, L: proa_core::DataLoader> proa_core::WebRenderSync<L, B> for ProjectTrackerBlock {
    fn render(
        self,
        cx: &mut proa_core::WebContext<L, B>,
    ) -> Result<(), proa_core::WriteError> {
        html_sync! {
            <div class="flex flex-col gap-6">
                <div class="overflow-hidden rounded-2xl border border-border bg-background shadow-xs">
                    <div class="border-b border-border p-5 sm:p-7">
                        <div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
                            <div>
                                <div class="flex items-center gap-3">
                                    <h2 class="text-xl font-semibold tracking-tight text-foreground">
                                        "Q3 delivery plan"
                                    </h2>
                                    {Badge { variant: BadgeVariant::Outline, class: Some("rounded-full bg-primary/5 text-primary shadow-none"), children: Some("12 projects"), }}
                                </div>
                                <p class="mt-1 text-sm text-muted-foreground">
                                    "Cross-functional initiatives due before September 30."
                                </p>
                            </div>
                            <div class="flex items-center gap-2">
                                {Button { size: Some(ButtonSize::Sm), variant: Some(ButtonVariant::Outline), children: Some("Filter"), ..Default::default() }}
                                {Button { size: Some(ButtonSize::Sm), children: Some("New project"), ..Default::default() }}
                            </div>
                        </div>
                    </div>
                    <div class="grid gap-px border-b border-border bg-border md:grid-cols-[1fr_1fr_1.4fr]">
                        <div class="bg-card p-5">
                            <p class="text-sm text-muted-foreground">"Portfolio budget"</p>
                            <p class="mt-1 text-2xl font-semibold tracking-tight text-foreground tabular-nums">
                                "$684,000"
                            </p>
                            <p class="mt-1 text-xs text-muted-foreground">"$412,300 committed"</p>
                        </div>
                        <div class="bg-card p-5">
                            <p class="text-sm text-muted-foreground">"On track"</p>
                            <p class="mt-1 text-2xl font-semibold tracking-tight text-emerald-600 tabular-nums dark:text-emerald-400">
                                "8 of 12"
                            </p>
                            <p class="mt-1 text-xs text-muted-foreground">"2 need a decision"</p>
                        </div>
                        <div class="bg-card p-5">
                            <div class="flex items-center justify-between gap-4">
                                <p class="text-sm text-muted-foreground">"Quarter progress"</p>
                                <span class="text-sm font-semibold text-foreground tabular-nums">
                                    "68%"
                                </span>
                            </div>
                            <div class="mt-3">
                                {Progress { value: 68, aria_label: Some("Quarter progress: 68 percent"), class: Some("h-2"), ..Progress::new() }}
                            </div>
                            <p class="mt-2 text-xs text-muted-foreground">"42 days remaining"</p>
                        </div>
                    </div>
                    <div class="p-5 sm:p-7">
                        <div class="overflow-hidden rounded-xl border border-border">
                            {Table { class: None, children: Some(html_sync! {
                                {TableHeader { class: None, children: Some(html_sync! {
                                    {TableRow { class: Some("bg-muted/50 hover:bg-muted/50"), children: Some(html_sync! {
                                        {TableHead { class: Some("w-[32%]"), children: Some("Project") }}
                                        {TableHead { class: None, children: Some("Budget") }}
                                        {TableHead { class: None, children: Some("Deadline") }}
                                        {TableHead { class: None, children: Some("Owners") }}
                                        {TableHead { class: None, children: Some("Status") }}
                                    }), }}
                                }), }}
                                {TableBody { class: None, children: Some(html_sync! {
                                    {for row in ROWS {
                                        match row {
                                            Row::Group(team) => html_sync! {
                                                {TableRow { class: Some("bg-muted/30 hover:bg-muted/30"), children: Some(html_sync! {
                                                    {TableCell { class: Some("font-semibold text-foreground"), children: Some(*team) }}
                                                    {for _ in 1..COLUMNS {
                                                        html_sync! {
                                                            {TableCell { class: None, children: None::<&str> }}
                                                        }
                                                    }}
                                                }), }}
                                            }.render(cx)?,
                                            Row::Project(project) => html_sync! {
                                                {TableRow { class: None, children: Some(html_sync! {
                                                    {TableCell { class: None, children: Some(html_sync! {
                                                        <p class="font-medium text-foreground">{project.name}</p>
                                                        <p class="mt-0.5 text-xs text-muted-foreground">{project.summary}</p>
                                                    }), }}
                                                    {TableCell { class: Some("font-medium tabular-nums"), children: Some(project.budget) }}
                                                    {TableCell { class: Some("text-muted-foreground"), children: Some(project.deadline) }}
                                                    {TableCell { class: None, children: Some(html_sync! {
                                                        {if let [owner] = project.owners {
                                                            // A lone owner sits directly in the cell.
                                                            // Wrapping one avatar in a flex row would
                                                            // change its flow for no visual gain, and
                                                            // `-space-x-2` on a stack of one reads as a
                                                            // mistake.
                                                            owner.avatar().render(cx)?;
                                                        } else {
                                                            html_sync! {
                                                                <div class="flex -space-x-2">
                                                                    {for owner in project.owners {
                                                                        html_sync! { {owner.avatar()} }
                                                                    }}
                                                                </div>
                                                            }.render(cx)?;
                                                        }}
                                                    }), }}
                                                    {TableCell { class: None, children: Some(Badge { variant: project.status.variant(), class: Some(project.status.class()), children: Some(project.status.label()) }) }}
                                                }), }}
                                            }.render(cx)?,
                                        }
                                    }}
                                }), }}
                            }), }}
                        </div>
                    </div>
                </div>
            </div>
        }
        .render(cx)
    }
}
