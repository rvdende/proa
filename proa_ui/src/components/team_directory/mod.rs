// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Provenance: adapted from `content/components/grid-list/grid-list-02.tsx` in
//! https://github.com/ephraimduncan/blocks at commit
//! fe4305a1147355f5cd1ca9cb03b6fa88f013bd6a. MIT License, Copyright (c) 2025
//! Ephraim Duncan; see the upstream `LICENSE.md`.

use proa_macros::html_sync;
use crate::components::{
    Avatar, AvatarFallback, AvatarImage, AvatarShape, AvatarSize, Badge, BadgeVariant, Button,
    ButtonSize, ButtonVariant, Card, Input, InputType,
};

/// Availability, as one value rather than a loose dot-colour/label pair: the two
/// always move together, and splitting them lets a card claim "Available" under
/// an amber dot.
#[derive(Clone, Copy)]
enum Presence {
    Available,
    Away,
    Offline,
}

impl Presence {
    fn dot(self) -> &'static str {
        match self {
            Self::Available => "bg-emerald-500",
            Self::Away => "bg-amber-500",
            Self::Offline => "bg-zinc-400",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Available => "Available",
            Self::Away => "Away",
            Self::Offline => "Offline",
        }
    }
}

struct Member {
    initials: &'static str,
    /// Avatar tint. Written out in full rather than composed from a colour name
    /// because Tailwind scans source for literal class names.
    avatar: &'static str,
    presence: Presence,
    name: &'static str,
    role: &'static str,
    team: &'static str,
    city: &'static str,
}

const MEMBERS: &[Member] = &[
    Member {
        initials: "AM",
        avatar: "bg-violet-500/15 font-semibold text-violet-700 dark:text-violet-300",
        presence: Presence::Available,
        name: "Avery Morgan",
        role: "VP of Product",
        team: "Product",
        city: "San Francisco",
    },
    Member {
        initials: "JL",
        avatar: "bg-sky-500/15 font-semibold text-sky-700 dark:text-sky-300",
        presence: Presence::Available,
        name: "Jordan Lee",
        role: "Staff Engineer",
        team: "Engineering",
        city: "New York",
    },
    Member {
        initials: "SK",
        avatar: "bg-amber-500/15 font-semibold text-amber-700 dark:text-amber-300",
        presence: Presence::Away,
        name: "Samira Khan",
        role: "Product Designer",
        team: "Design",
        city: "London",
    },
    Member {
        initials: "RC",
        avatar: "bg-emerald-500/15 font-semibold text-emerald-700 dark:text-emerald-300",
        presence: Presence::Available,
        name: "Rowan Chen",
        role: "Engineering Manager",
        team: "Engineering",
        city: "Toronto",
    },
    Member {
        initials: "MP",
        avatar: "bg-rose-500/15 font-semibold text-rose-700 dark:text-rose-300",
        presence: Presence::Offline,
        name: "Mina Patel",
        role: "Design Engineer",
        team: "Design",
        city: "Berlin",
    },
    Member {
        initials: "DT",
        avatar: "bg-fuchsia-500/15 font-semibold text-fuchsia-700 dark:text-fuchsia-300",
        presence: Presence::Available,
        name: "Dario Torres",
        role: "Developer Advocate",
        team: "Product",
        city: "Austin",
    },
];

const ROLE_FILTERS: &[&str] = &["Design", "Engineering", "Product"];

/// Source-installed `team_directory` block composition.
pub struct TeamDirectoryBlock;

impl<B: proa_core::WriteBuf, L: proa_core::DataLoader> proa_core::WebRenderSync<L, B> for TeamDirectoryBlock {
    fn render(
        self,
        cx: &mut proa_core::WebContext<L, B>,
    ) -> Result<(), proa_core::WriteError> {
        html_sync! {
            <div class="flex flex-col gap-6">
                <div class="rounded-2xl border border-border bg-background p-5 shadow-xs sm:p-7">
                    <div class="flex flex-col gap-5 border-b border-border pb-6 lg:flex-row lg:items-end lg:justify-between">
                        <div>
                            <div class="flex items-center gap-3">
                                <h2 class="text-xl font-semibold tracking-tight text-foreground">
                                    "People"
                                </h2>
                                {Badge {
                                    variant: BadgeVariant::Secondary,
                                    class: Some("rounded-full px-2.5 shadow-none"),
                                    children: Some("24 members"),
                                }}
                            </div>
                            <p class="mt-1 text-sm text-muted-foreground">
                                "Everyone with access to the Proa workspace."
                            </p>
                        </div>
                        <div class="flex flex-col gap-3 sm:flex-row sm:items-center">
                            <div class="relative min-w-0 sm:w-72">
                                <svg
                                    class="pointer-events-none absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground"
                                    xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    aria-hidden="true"
                                >
                                    <circle cx="11" cy="11" r="8" />
                                    <path d="m21 21-4.3-4.3" />
                                </svg>
                                {Input {
                                    input_type: InputType::Search,
                                    placeholder: Some("Search people..."),
                                    aria_label: Some("Search team members"),
                                    class: Some("pl-9"),
                                    ..Input::new()
                                }}
                            </div>
                            {Button {
                                size: Some(ButtonSize::Sm),
                                children: Some("Invite member"),
                                ..Default::default()
                            }}
                        </div>
                    </div>
                    <div class="mt-5 flex flex-wrap items-center gap-2">
                        {Button {
                            size: Some(ButtonSize::Sm),
                            children: Some("All"),
                            ..Default::default()
                        }}
                        {for filter in ROLE_FILTERS {
                            html_sync! {
                                {Button { size: Some(ButtonSize::Sm), variant: Some(ButtonVariant::Outline), children: Some(*filter), ..Default::default() }}
                            }
                        }}
                        <span class="ml-auto hidden text-xs text-muted-foreground sm:block">
                            "Sorted by name"
                        </span>
                    </div>
                    <ul class="mt-5 grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3">
                        {for member in MEMBERS {
                            html_sync! {
                                <li>
                                    {Card { class: Some("group relative h-full py-0 shadow-xs transition-[border-color,box-shadow,transform,translate] duration-200 hover:-translate-y-0.5 hover:border-primary/30 hover:shadow-md dark:border-white/10"), children: Some(html_sync! {
                                        <div class="flex items-start gap-4 p-5">
                                            <div class="relative shrink-0">
                                                {Avatar { size: AvatarSize::Lg, shape: AvatarShape::Circle, class: None, image: None::<AvatarImage>, fallback: Some(AvatarFallback { size: AvatarSize::Lg, class: Some(member.avatar), children: Some(member.initials), }), }}
                                                <span class={proa_macros::text!("absolute bottom-0 right-0 size-3 rounded-full border-2 border-card {}", member.presence.dot())} aria-hidden="true"></span>
                                                <span class="sr-only">
                                                    {member.presence.label()}
                                                </span>
                                            </div>
                                            <div class="min-w-0 flex-1">
                                                <a href="/blocks/team_directory" class="font-semibold text-foreground outline-hidden after:absolute after:inset-0 focus-visible:underline">
                                                    {member.name}
                                                </a>
                                                <p class="mt-0.5 truncate text-sm text-muted-foreground">
                                                    {member.role}
                                                </p>
                                                <div class="mt-3 flex items-center gap-2">
                                                    {Badge { variant: BadgeVariant::Outline, class: Some("font-normal"), children: Some(member.team) }}
                                                    <span class="text-xs text-muted-foreground">
                                                        {member.city}
                                                    </span>
                                                </div>
                                            </div>
                                            <svg class="size-4 shrink-0 text-muted-foreground transition-transform group-hover:translate-x-0.5 group-hover:text-foreground" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                                                <path d="m9 18 6-6-6-6" />
                                            </svg>
                                        </div>
                                    }), }}
                                </li>
                            }
                        }}
                    </ul>
                    <div class="mt-6 flex flex-col gap-3 border-t border-border pt-5 text-sm text-muted-foreground sm:flex-row sm:items-center sm:justify-between">
                        <p>"Showing 6 of 24 members"</p>
                        <div class="flex gap-2">
                            {Button {
                                size: Some(ButtonSize::Sm),
                                variant: Some(ButtonVariant::Outline),
                                children: Some("Previous"),
                                ..Default::default()
                            }}
                            {Button {
                                size: Some(ButtonSize::Sm),
                                variant: Some(ButtonVariant::Outline),
                                children: Some("Next"),
                                ..Default::default()
                            }}
                        </div>
                    </div>
                </div>
            </div>
        }
        .render(cx)
    }
}
