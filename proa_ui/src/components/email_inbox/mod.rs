// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Adapted from shadcn/ui
//! `apps/v4/registry/new-york-v4/blocks/sidebar-09/page.tsx` and
//! `apps/v4/registry/new-york-v4/blocks/sidebar-09/components/app-sidebar.tsx` at
//! https://github.com/shadcn-ui/ui, commit
//! fa4872c8ed948c111884e52ae23a649e83591b71. MIT License, Copyright (c) 2023
//! shadcn; see the upstream `LICENSE.md`.

use proa_core::{DataLoader, StaticRaw, WebContext, WebRenderSync, WriteBuf, WriteError};
use proa_macros::html_sync;
use crate::components::{
    Avatar, AvatarFallback, AvatarImage, AvatarShape, AvatarSize, Card, Input, InputType,
};
use rsjs_macros::rsjs;

struct Message {
    sender: &'static str,
    initials: &'static str,
    email: &'static str,
    /// Timestamp as the list shows it, beside the long form the reading pane
    /// uses. Two fields because the list column is width-constrained.
    list_time: &'static str,
    full_time: &'static str,
    subject: &'static str,
    preview: &'static str,
    category: &'static str,
    /// The two body paragraphs, between the greeting and the sign-off.
    body: [&'static str; 2],
    signoff: &'static str,
    unread: bool,
}

const MESSAGES: &[Message] = &[
    Message {
        sender: "Maya Chen",
        initials: "MC",
        email: "maya@northstar.dev",
        list_time: "9:34 AM",
        full_time: "Today, 9:34 AM",
        subject: "Design review tomorrow",
        preview: "I added the final interaction notes and linked the updated prototype for tomorrow's review.",
        category: "Design",
        body: [
            "I added the final interaction notes and linked the updated prototype for tomorrow's review. The navigation and empty states are ready for engineering feedback.",
            "Could you take one last pass through the mobile flow before 3 PM? I want to resolve any layout questions before the meeting.",
        ],
        signoff: "Thanks, Maya",
        unread: true,
    },
    Message {
        sender: "Jordan Lee",
        initials: "JL",
        email: "jordan@northstar.dev",
        list_time: "Yesterday",
        full_time: "Yesterday, 4:18 PM",
        subject: "Re: Edge rollout",
        preview: "The rollout is holding steady at one hundred percent. Error rates are below the target.",
        category: "Engineering",
        body: [
            "The rollout is holding steady at one hundred percent. Error rates are below the target, and the regional edge metrics are all inside the expected range.",
            "I will keep the dashboards open through tomorrow morning, but there is nothing in the current data that should block the next region.",
        ],
        signoff: "Thanks, Jordan",
        unread: false,
    },
    Message {
        sender: "Samira Khan",
        initials: "SK",
        email: "samira@northstar.dev",
        list_time: "Monday",
        full_time: "Monday, 11:02 AM",
        subject: "Research synthesis",
        preview: "Here are the recurring themes from the six customer conversations we completed last week.",
        category: "Research",
        body: [
            "I grouped the recurring themes from the six customer conversations we completed last week and added supporting quotes to the research board.",
            "The strongest pattern is that teams want clearer ownership during handoffs. I highlighted three opportunities for the next planning session.",
        ],
        signoff: "Best, Samira",
        unread: false,
    },
    Message {
        sender: "Dario Torres",
        initials: "DT",
        email: "dario@northstar.dev",
        list_time: "Friday",
        full_time: "Friday, 2:47 PM",
        subject: "Launch checklist",
        preview: "Documentation and migration examples are ready. I left two questions in the release brief.",
        category: "Launch",
        body: [
            "Documentation and migration examples are ready. I also checked the release steps against the latest staging build and noted the final owners.",
            "Two questions remain in the release brief: the final support window and who sends the customer announcement. Everything else is checked off.",
        ],
        signoff: "Thanks, Dario",
        unread: false,
    },
];

/// How one list row is drawn. Read state changes the dot, its screen-reader
/// label, and the sender weight together, so they are derived in one place
/// instead of re-testing `unread` at three points in the markup.
struct RowStyle {
    dot: &'static str,
    unread_label: &'static str,
    sender: &'static str,
    visibility: &'static str,
}

impl Message {
    fn row_style(&self) -> RowStyle {
        if self.unread {
            RowStyle {
                dot: "mt-1 size-2 shrink-0 rounded-full bg-primary",
                unread_label: "Unread",
                sender: "font-semibold text-foreground",
                visibility: "",
            }
        } else {
            // A read row keeps the dot in the tree but collapsed, so the flex
            // gap does not leave a hole where the indicator would be.
            RowStyle {
                dot: "hidden",
                unread_label: "",
                sender: "font-medium text-foreground",
                visibility: "group-data-[unread-only=true]/inbox:hidden",
            }
        }
    }
}

/// `MESSAGES.len()`, spelled as a `u32` because the selection signal and every
/// loop bound inside the island have to carry a proven fixed-width domain.
const MESSAGE_COUNT: u32 = 4;

const MAIL_ICON: &str = r#"<path d="M4 4h16v16H4z" /><path d="m4 7 8 6 8-6" />"#;

struct Folder {
    label: &'static str,
    /// Inner markup for the 24×24 icon.
    icon: &'static str,
    /// Unread count, shown only on the folder that has one.
    count: Option<&'static str>,
    current: bool,
}

const FOLDERS: &[Folder] = &[
    Folder {
        label: "Inbox",
        icon: MAIL_ICON,
        count: Some("12"),
        current: true,
    },
    Folder {
        label: "Drafts",
        icon: r#"<path d="M4 19h16" /><path d="M7 16V5h10v11" />"#,
        count: None,
        current: false,
    },
    Folder {
        label: "Sent",
        icon: r#"<path d="m3 11 18-8-8 18-2-8z" />"#,
        count: None,
        current: false,
    },
    Folder {
        label: "Archive",
        icon: r#"<path d="M5 7h14l-1 13H6z" /><path d="M9 7V4h6v3" />"#,
        count: None,
        current: false,
    },
    Folder {
        label: "Junk",
        icon: r#"<path d="M4 7h16" /><path d="m9 11 6 6" /><path d="m15 11-6 6" />"#,
        count: None,
        current: false,
    },
];

struct InboxMessages;

#[rsjs(component, client)]
impl<B: WriteBuf, L: DataLoader> WebRenderSync<L, B> for InboxMessages {
    fn render(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        // rsjs resolves identifiers inside an island against function-local
        // bindings; a module-level const is invisible to it. Binding the table
        // here is what lets the loops below read `messages[i]`.
        let messages = MESSAGES;
        let search_input = Input {
            input_type: InputType::Search,
            placeholder: Some("Search messages..."),
            aria_label: Some("Search messages"),
            ..Input::new()
        };
        let selected_message = rsjs::signal(0_u32);
        let unread_only = rsjs::signal(false);
        let select_message =
            rsjs::event_handler(|_: rsjs::MouseEvent<rsjs::elem::Button>, message: u32| {
                selected_message.set(message);
            });
        let filter_unread = rsjs::event_handler(|event: rsjs::ChangeEvent<rsjs::elem::Input>| {
            let next = event.current_checked();
            unread_only.set(next);
            if next {
                selected_message.set(0_u32);
            }
        });

        html_sync! {
            <div class="grid min-w-0 lg:grid-cols-[310px_minmax(0,1fr)]">
                <section
                    aria_labelledby="inbox-message-list-title"
                    class="min-w-0 border-b border-border lg:border-b-0 lg:border-r"
                >
                    <div class="border-b border-border p-4">
                        <div class="flex items-center justify-between gap-3">
                            <h2 id="inbox-message-list-title" class="font-semibold text-foreground">
                                "Inbox"
                            </h2>
                            <label class="flex cursor-pointer items-center gap-2 text-xs text-muted-foreground">
                                <span>"Unreads"</span>
                                <input
                                    type="checkbox"
                                    class="peer sr-only"
                                    aria_label="Show unread messages only"
                                    checked={unread_only.get()}
                                    on_change={filter_unread()}
                                />
                                <span
                                    aria_hidden="true"
                                    class="relative h-5 w-9 rounded-full bg-input transition-colors peer-checked:bg-primary after:absolute after:left-0.5 after:top-0.5 after:size-4 after:rounded-full after:bg-background after:shadow-xs after:transition-transform peer-checked:after:translate-x-4"
                                ></span>
                            </label>
                        </div>
                        <div class="mt-3">{search_input}</div>
                    </div>
                    <div class="h-[430px] overflow-y-auto lg:h-[592px]">
                        <div
                            class="group/inbox divide-y divide-border"
                            data_unread_only={unread_only.get()}
                        >
                            {for index in 0_u32..MESSAGE_COUNT {
                                let message = &messages[index as usize];
                                let row = message.row_style();

                                html_sync! {
                                    <div class={row.visibility}>
                                        <button
                                            type="button"
                                            class={if selected_message.get() == (index as u32) {
                                                "block w-full bg-accent p-4 text-left text-sm transition-colors hover:bg-accent focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-ring"
                                            } else {
                                                "block w-full p-4 text-left text-sm transition-colors hover:bg-accent focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-ring"
                                            }}
                                            aria_pressed={selected_message.get() == (index as u32)}
                                            aria_controls="inbox-reading-pane"
                                            on_click={select_message(index)}
                                        >
                                            <div class="flex items-start gap-3">
                                                <span class={row.dot}>
                                                    <span class="sr-only">
                                                        {row.unread_label}
                                                    </span>
                                                </span>
                                                <div class="min-w-0 flex-1">
                                                    <div class="flex items-center justify-between gap-3">
                                                        <span class={row.sender}>
                                                            {message.sender}
                                                        </span>
                                                        <time class="text-xs text-muted-foreground">
                                                            {message.list_time}
                                                        </time>
                                                    </div>
                                                    <p class="mt-1 truncate font-medium text-foreground">
                                                        {message.subject}
                                                    </p>
                                                    <p class="mt-1 line-clamp-2 text-xs leading-5 text-muted-foreground">
                                                        {message.preview}
                                                    </p>
                                                </div>
                                            </div>
                                        </button>
                                    </div>
                                }
                            }}
                        </div>
                    </div>
                </section>
                <section
                    id="inbox-reading-pane"
                    aria_labelledby="inbox-reading-title"
                    aria_live="polite"
                    class="min-w-0 bg-background"
                >
                    <div class="flex h-16 items-center gap-2 border-b border-border px-4">
                        <button
                            type="button"
                            aria_label="Archive message"
                            class="inline-flex size-9 items-center justify-center rounded-md text-foreground transition-colors hover:bg-accent focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-ring"
                        >
                            <svg
                                aria_hidden="true"
                                viewBox="0 0 24 24"
                                class="size-4"
                                fill="none"
                                stroke="currentColor"
                                stroke_width="2"
                            >
                                <path d="M5 7h14l-1 13H6z" />
                                <path d="M9 7V4h6v3" />
                            </svg>
                        </button>
                        <button
                            type="button"
                            aria_label="Mark as junk"
                            class="inline-flex size-9 items-center justify-center rounded-md text-foreground transition-colors hover:bg-accent focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-ring"
                        >
                            <svg
                                aria_hidden="true"
                                viewBox="0 0 24 24"
                                class="size-4"
                                fill="none"
                                stroke="currentColor"
                                stroke_width="2"
                            >
                                <path d="M4 7h16" />
                                <path d="m9 11 6 6" />
                                <path d="m15 11-6 6" />
                            </svg>
                        </button>
                        <span role="none" class="mx-1 h-5 w-px bg-border"></span>
                        <button
                            type="button"
                            aria_label="Delete message"
                            class="inline-flex size-9 items-center justify-center rounded-md text-foreground transition-colors hover:bg-accent focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-ring"
                        >
                            <svg
                                aria_hidden="true"
                                viewBox="0 0 24 24"
                                class="size-4"
                                fill="none"
                                stroke="currentColor"
                                stroke_width="2"
                            >
                                <path d="M3 6h18" />
                                <path d="M8 6V4h8v2" />
                                <path d="m6 6 1 14h10l1-14" />
                            </svg>
                        </button>
                        <span class="ml-auto inline-flex items-center rounded-full bg-secondary px-2.5 py-0.5 text-xs font-medium text-secondary-foreground">
                            {for index in 0_u32..MESSAGE_COUNT {
                                let message = &messages[index as usize];
                                html_sync! {
                                    <span class={if selected_message.get() == (index as u32) {
                                        ""
                                    } else {
                                        "hidden"
                                    }}>
                                        {message.category}
                                    </span>
                                }
                            }}
                        </span>
                    </div>
                    <article class="p-5 sm:p-7">
                        <div class="flex items-start gap-4">
                            <span class="relative flex size-12 shrink-0 overflow-hidden rounded-full">
                                <span class="flex h-full w-full items-center justify-center bg-violet-500/15 text-base font-semibold text-violet-700 dark:text-violet-300">
                                    {for index in 0_u32..MESSAGE_COUNT {
                                        let message = &messages[index as usize];
                                        html_sync! {
                                            <span class={if selected_message.get() == (index as u32) {
                                                ""
                                            } else {
                                                "hidden"
                                            }}>
                                                {message.initials}
                                            </span>
                                        }
                                    }}
                                </span>
                            </span>
                            <div class="min-w-0 flex-1">
                                <div class="flex flex-wrap items-start justify-between gap-2">
                                    <div>
                                        <p class="font-semibold text-foreground">
                                            {for index in 0_u32..MESSAGE_COUNT {
                                                let message = &messages[index as usize];
                                                html_sync! {
                                                    <span class={if selected_message.get() == (index as u32) {
                                                        ""
                                                    } else {
                                                        "hidden"
                                                    }}>
                                                        {message.sender}
                                                    </span>
                                                }
                                            }}
                                        </p>
                                        <p class="text-xs text-muted-foreground">
                                            {for index in 0_u32..MESSAGE_COUNT {
                                                let message = &messages[index as usize];
                                                html_sync! {
                                                    <span class={if selected_message.get() == (index as u32) {
                                                        ""
                                                    } else {
                                                        "hidden"
                                                    }}>
                                                        {message.email}
                                                    </span>
                                                }
                                            }}
                                        </p>
                                    </div>
                                    <time class="text-xs text-muted-foreground">
                                        {for index in 0_u32..MESSAGE_COUNT {
                                            let message = &messages[index as usize];
                                            html_sync! {
                                                <span class={if selected_message.get() == (index as u32) {
                                                    ""
                                                } else {
                                                    "hidden"
                                                }}>
                                                    {message.full_time}
                                                </span>
                                            }
                                        }}
                                    </time>
                                </div>
                                <h2
                                    id="inbox-reading-title"
                                    class="mt-6 text-xl font-semibold tracking-tight text-foreground"
                                >
                                    {for index in 0_u32..MESSAGE_COUNT {
                                        let message = &messages[index as usize];
                                        html_sync! {
                                            <span class={if selected_message.get() == (index as u32) {
                                                ""
                                            } else {
                                                "hidden"
                                            }}>
                                                {message.subject}
                                            </span>
                                        }
                                    }}
                                </h2>
                                <div class="mt-5 space-y-4 text-sm leading-7 text-muted-foreground">
                                    <p>"Hi Alex,"</p>
                                    <p>
                                        {for index in 0_u32..MESSAGE_COUNT {
                                            let message = &messages[index as usize];
                                            html_sync! {
                                                <span class={if selected_message.get() == (index as u32) {
                                                    ""
                                                } else {
                                                    "hidden"
                                                }}>
                                                    {message.body[0]}
                                                </span>
                                            }
                                        }}
                                    </p>
                                    <p>
                                        {for index in 0_u32..MESSAGE_COUNT {
                                            let message = &messages[index as usize];
                                            html_sync! {
                                                <span class={if selected_message.get() == (index as u32) {
                                                    ""
                                                } else {
                                                    "hidden"
                                                }}>
                                                    {message.body[1]}
                                                </span>
                                            }
                                        }}
                                    </p>
                                    <p>
                                        {for index in 0_u32..MESSAGE_COUNT {
                                            let message = &messages[index as usize];
                                            html_sync! {
                                                <span class={if selected_message.get() == (index as u32) {
                                                    ""
                                                } else {
                                                    "hidden"
                                                }}>
                                                {message.signoff}
                                                </span>
                                            }
                                        }}
                                    </p>
                                </div>
                            </div>
                        </div>
                        <div class="mt-8 flex flex-wrap gap-2 border-t border-border pt-5">
                            <button
                                type="button"
                                class="inline-flex h-9 items-center justify-center rounded-md bg-primary px-4 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90 focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
                            >
                                "Reply"
                            </button>
                            <button
                                type="button"
                                class="inline-flex h-9 items-center justify-center rounded-md border border-primary bg-background px-4 text-sm font-medium text-primary shadow-xs transition-colors hover:bg-primary/10 focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 dark:bg-transparent dark:hover:bg-primary/20"
                            >
                                "Forward"
                            </button>
                        </div>
                    </article>
                </section>
            </div>
        }
        .render(cx)
    }
}

/// Source-installed `email_inbox` block composition.
pub struct EmailInboxBlock;

impl<B: proa_core::WriteBuf, L: proa_core::DataLoader> proa_core::WebRenderSync<L, B> for EmailInboxBlock {
    fn render(
        self,
        cx: &mut proa_core::WebContext<L, B>,
    ) -> Result<(), proa_core::WriteError> {
        html_sync! {
            <div class="flex flex-col gap-6">
                {Card {
                    class: Some("overflow-hidden py-0 shadow-xs"),
                    children: Some(html_sync! {
                        <section
                            data-block="email-inbox"
                            aria-label="Email inbox"
                            class="grid min-h-[720px] lg:grid-cols-[190px_minmax(0,1fr)]"
                        >
                            <aside class="flex flex-col border-b border-border bg-muted/20 lg:border-b-0 lg:border-r">
                                <div class="flex h-16 items-center gap-3 border-b border-border px-4">
                                    <span class="flex size-9 items-center justify-center rounded-lg bg-primary text-primary-foreground">
                                        <svg
                                            aria-hidden="true"
                                            viewBox="0 0 24 24"
                                            class="size-4"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                        >
                                            {StaticRaw(MAIL_ICON)}
                                        </svg>
                                    </span>
                                    <div>
                                        <p class="text-sm font-semibold text-foreground">
                                            "Northstar"
                                        </p>
                                        <p class="text-xs text-muted-foreground">"Product team"</p>
                                    </div>
                                </div>
                                <nav
                                    aria-label="Mailbox folders"
                                    class="grid grid-cols-2 gap-1 p-3 sm:grid-cols-5 lg:grid-cols-1"
                                >
                                    {for folder in FOLDERS {
                                        let classes = if folder.current {
                                            "flex items-center gap-3 rounded-md bg-accent px-3 py-2 text-sm font-medium text-accent-foreground"
                                        } else {
                                            "flex items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
                                        };

                                        html_sync! {
                                            <a href="/blocks/email_inbox" aria-current={folder.current.then_some("page")} class={classes}>
                                                <svg aria-hidden="true" viewBox="0 0 24 24" class="size-4" fill="none" stroke="currentColor" stroke-width="2">
                                                    {StaticRaw(folder.icon)}
                                                </svg>
                                                <span>{folder.label}</span>
                                                {if let Some(count) = folder.count {
                                                    html_sync! {
                                                        <span class="ml-auto text-xs tabular-nums">{count}</span>
                                                    }.render(cx)?;
                                                }}
                                            </a>
                                        }
                                    }}
                                </nav>
                                <div class="mt-auto hidden border-t border-border p-4 lg:block">
                                    <div class="flex items-center gap-3">
                                        {Avatar {
                                            size: AvatarSize::Sm,
                                            shape: AvatarShape::Circle,
                                            class: None,
                                            image: None::<AvatarImage>,
                                            fallback: Some(AvatarFallback {
                                                size: AvatarSize::Sm,
                                                class: Some("bg-primary/10 font-semibold text-primary"),
                                                children: Some("AH"),
                                            }),
                                        }}
                                        <div class="min-w-0">
                                            <p class="truncate text-sm font-medium text-foreground">
                                                "Alex Harper"
                                            </p>
                                            <p class="truncate text-xs text-muted-foreground">
                                                "alex@northstar.dev"
                                            </p>
                                        </div>
                                    </div>
                                </div>
                            </aside>
                            <div class="min-w-0">{InboxMessages}</div>
                        </section>
                    }),
                }}
            </div>
        }
        .render(cx)
    }
}
