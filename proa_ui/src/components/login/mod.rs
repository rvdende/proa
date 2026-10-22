// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Login block — centered card with email/password form, social logins

use proa_core::UnsafeRaw;
use proa_macros::html_sync;
use crate::components::{
    Button, Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, Checkbox, Input,
    InputType, Label, Separator,
};

const SOCIAL_BUTTONS: &str = r##"<div class="grid grid-cols-2 gap-4"><button type="button" data-button data-variant="outline" class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-9 px-4 py-2 border border-input bg-background shadow-xs hover:bg-accent hover:text-accent-foreground transition-colors w-full"><svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>GitHub</button><button type="button" data-button data-variant="outline" class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium h-9 px-4 py-2 border border-input bg-background shadow-xs hover:bg-accent hover:text-accent-foreground transition-colors w-full"><svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"><path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92a5.06 5.06 0 0 1-2.2 3.32v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.1z"/><path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/><path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/><path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/></svg>Google</button></div>"##;

/// Source-installed `login` block composition.
pub struct LoginBlock;

impl<B: proa_core::WriteBuf, L: proa_core::DataLoader> proa_core::WebRenderSync<L, B> for LoginBlock {
    fn render(
        self,
        cx: &mut proa_core::WebContext<L, B>,
    ) -> Result<(), proa_core::WriteError> {
        html_sync! {
            <div class="flex flex-col gap-6">
                <div class="flex items-center justify-center py-12">
                    <div class="w-full max-w-md">
                        {Card {
                            class: None,
                            children: Some(html_sync! {
                                {CardHeader {
                                    class: Some("text-center"),
                                    children: Some(html_sync! {
                                        {CardTitle {
                                            class: Some("text-2xl"),
                                            children: Some("Login"),
                                        }}
                                        {CardDescription {
                                            class: None,
                                            children: Some("Enter your email below to login to your account"),
                                        }}
                                    }),
                                }}
                                {CardContent {
                                    class: None,
                                    children: Some(html_sync! {
                                        <form class="space-y-4">
                                            <div class="space-y-2">
                                                {Label {
                                                    for_id: Some("email"),
                                                    text: "Email",
                                                    ..Label::new()
                                                }}
                                                {Input {
                                                    id: Some("email"),
                                                    input_type: InputType::Email,
                                                    placeholder: Some("m@example.com"),
                                                    ..Input::new()
                                                }}
                                            </div>
                                            <div class="space-y-2">
                                                <div class="flex items-center justify-between">
                                                    {Label {
                                                        for_id: Some("password"),
                                                        text: "Password",
                                                        ..Label::new()
                                                    }}
                                                    <a
                                                        href="##"
                                                        class="text-sm text-muted-foreground underline-offset-4 hover:underline hover:text-foreground"
                                                    >
                                                        "Forgot password?"
                                                    </a>
                                                </div>
                                                {Input {
                                                    id: Some("password"),
                                                    input_type: InputType::Password,
                                                    placeholder: Some("Enter your password"),
                                                    ..Input::new()
                                                }}
                                            </div>
                                            <div class="flex items-center gap-2">
                                                {Checkbox {
                                                    id: Some("remember"),
                                                    ..Checkbox::new()
                                                }}
                                                {Label {
                                                    for_id: Some("remember"),
                                                    text: "Remember me",
                                                    class: Some("font-normal"),
                                                    ..Label::new()
                                                }}
                                            </div>
                                            {Button {
                                                class: Some("w-full"),
                                                children: Some("Sign in"),
                                                ..Default::default()
                                            }}
                                            <div class="relative flex items-center justify-center">
                                                {Separator::new()}
                                                <span class="absolute bg-card px-2 text-xs text-muted-foreground">
                                                    "or continue with"
                                                </span>
                                            </div>
                                            {UnsafeRaw(SOCIAL_BUTTONS)}
                                        </form>
                                    }),
                                }}
                                {CardFooter {
                                    class: Some("justify-center"),
                                    children: Some(html_sync! {
                                        <p class="text-sm text-muted-foreground">
                                            "Don\u{2019}t have an account? "
                                            <a
                                                href="##"
                                                class="text-foreground underline-offset-4 hover:underline font-medium"
                                            >
                                                "Sign up"
                                            </a>
                                        </p>
                                    }),
                                }}
                            }),
                        }}
                    </div>
                </div>
            </div>
        }
        .render(cx)
    }
}
