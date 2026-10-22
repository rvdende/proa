// SPDX-FileCopyrightText: 2026 Proa Labs, Inc.
// SPDX-License-Identifier: (MIT OR Apache-2.0) AND MIT

//! Provenance: adapted from `content/components/tables/table-03.tsx` in
//! https://github.com/ephraimduncan/blocks at commit
//! fe4305a1147355f5cd1ca9cb03b6fa88f013bd6a. MIT License, Copyright (c) 2025
//! Ephraim Duncan; see the upstream `LICENSE.md`.

use proa_macros::html_sync;
use crate::components::{
    Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Input, InputType, Table, TableBody,
    TableCell, TableHead, TableHeader, TableRow,
};

struct Metric {
    label: &'static str,
    value: &'static str,
    /// Tone applied to the value itself; low and out-of-stock counts read as
    /// warnings rather than neutral figures.
    value_tone: &'static str,
    note: &'static str,
    note_tone: &'static str,
}

const NEUTRAL_VALUE: &str =
    "mt-2 text-2xl font-semibold tracking-tight text-foreground tabular-nums";
const NEUTRAL_NOTE: &str = "mt-1 text-xs text-muted-foreground";

const METRICS: &[Metric] = &[
    Metric {
        label: "Stock value",
        value: "$428,650",
        value_tone: NEUTRAL_VALUE,
        note: "+6.2% this month",
        note_tone: "mt-1 text-xs font-medium text-emerald-600 dark:text-emerald-400",
    },
    Metric {
        label: "Units on hand",
        value: "14,382",
        value_tone: NEUTRAL_VALUE,
        note: "Across 1,248 products",
        note_tone: NEUTRAL_NOTE,
    },
    Metric {
        label: "Low stock",
        value: "18",
        value_tone: "mt-2 text-2xl font-semibold tracking-tight text-amber-600 tabular-nums dark:text-amber-400",
        note: "7 need attention today",
        note_tone: NEUTRAL_NOTE,
    },
    Metric {
        label: "Out of stock",
        value: "4",
        value_tone: "mt-2 text-2xl font-semibold tracking-tight text-rose-600 tabular-nums dark:text-rose-400",
        note: "2 purchase orders open",
        note_tone: NEUTRAL_NOTE,
    },
];

/// Stock status. The badge tint and the tone of the stock count are two views of
/// the same fact, so they derive from one value instead of being set by hand per
/// row — which is how a row ends up amber in one column and green in the next.
#[derive(Clone, Copy)]
enum StockStatus {
    Active,
    Low,
    Incoming,
    OutOfStock,
}

impl StockStatus {
    fn label(self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Low => "Low stock",
            Self::Incoming => "Incoming",
            Self::OutOfStock => "Out of stock",
        }
    }

    fn badge(self) -> &'static str {
        match self {
            Self::Active => {
                "border-0 bg-emerald-500/10 text-emerald-700 shadow-none dark:text-emerald-400"
            }
            Self::Low => "border-0 bg-amber-500/10 text-amber-700 shadow-none dark:text-amber-400",
            Self::Incoming => {
                "border-0 bg-blue-500/10 text-blue-700 shadow-none dark:text-blue-400"
            }
            Self::OutOfStock => {
                "border-0 bg-rose-500/10 text-rose-700 shadow-none dark:text-rose-400"
            }
        }
    }

    fn count_tone(self) -> &'static str {
        match self {
            Self::Active | Self::Incoming => "text-right font-medium tabular-nums",
            Self::Low => "text-right font-medium tabular-nums text-amber-600 dark:text-amber-400",
            Self::OutOfStock => {
                "text-right font-medium tabular-nums text-rose-600 dark:text-rose-400"
            }
        }
    }
}

struct Product {
    name: &'static str,
    sku: &'static str,
    category: &'static str,
    stock: &'static str,
    status: StockStatus,
    price: &'static str,
    restocked: &'static str,
}

const PRODUCTS: &[Product] = &[
    Product {
        name: "Studio Headphones",
        sku: "SKU-8472",
        category: "Audio",
        stock: "245",
        status: StockStatus::Active,
        price: "$249.00",
        restocked: "Jun 18",
    },
    Product {
        name: "Mechanical Keyboard",
        sku: "SKU-3391",
        category: "Accessories",
        stock: "89",
        status: StockStatus::Active,
        price: "$129.00",
        restocked: "Jun 17",
    },
    Product {
        name: "Task Chair, Graphite",
        sku: "SKU-7156",
        category: "Furniture",
        stock: "12",
        status: StockStatus::Low,
        price: "$399.00",
        restocked: "Jun 12",
    },
    Product {
        name: "USB-C Travel Hub",
        sku: "SKU-9204",
        category: "Accessories",
        stock: "456",
        status: StockStatus::Incoming,
        price: "$79.00",
        restocked: "Jun 19",
    },
    Product {
        name: "Walnut Desk Shelf",
        sku: "SKU-1638",
        category: "Furniture",
        stock: "0",
        status: StockStatus::OutOfStock,
        price: "$189.00",
        restocked: "May 28",
    },
];

/// Source-installed `inventory_management` block composition.
pub struct InventoryManagementBlock;

impl<B: proa_core::WriteBuf, L: proa_core::DataLoader> proa_core::WebRenderSync<L, B> for InventoryManagementBlock {
    fn render(
        self,
        cx: &mut proa_core::WebContext<L, B>,
    ) -> Result<(), proa_core::WriteError> {
        html_sync! {
            <div class="flex flex-col gap-6">
                <div class="overflow-hidden rounded-2xl border border-border bg-background shadow-xs">
                    <div class="border-b border-border p-5 sm:p-7">
                        <div class="flex flex-col gap-4 lg:flex-row lg:items-center lg:justify-between">
                            <div>
                                <div class="flex items-center gap-3">
                                    <h2 class="text-xl font-semibold tracking-tight text-foreground">
                                        "Inventory"
                                    </h2>
                                    {Badge {
                                        variant: BadgeVariant::Secondary,
                                        class: Some("rounded-full px-2.5 shadow-none"),
                                        children: Some("1,248 SKUs"),
                                    }}
                                </div>
                                <p class="mt-1 text-sm text-muted-foreground">
                                    "Monitor stock across three fulfillment locations."
                                </p>
                            </div>
                            <div class="flex flex-col gap-2 sm:flex-row sm:items-center">
                                <div class="w-full sm:w-64">
                                    {Input {
                                        input_type: InputType::Search,
                                        placeholder: Some("Search products..."),
                                        aria_label: Some("Search inventory"),
                                        ..Input::new()
                                    }}
                                </div>
                                {Button {
                                    size: Some(ButtonSize::Sm),
                                    variant: Some(ButtonVariant::Outline),
                                    children: Some("All categories"),
                                    ..Default::default()
                                }}
                                {Button {
                                    size: Some(ButtonSize::Sm),
                                    children: Some("Add product"),
                                    ..Default::default()
                                }}
                            </div>
                        </div>
                    </div>
                    <dl class="grid grid-cols-2 gap-px border-b border-border bg-border lg:grid-cols-4">
                        {for metric in METRICS {
                            html_sync! {
                                <div class="bg-card p-5">
                                    <dt class="text-xs font-medium uppercase tracking-wide text-muted-foreground">
                                        {metric.label}
                                    </dt>
                                    <dd class={metric.value_tone}>{metric.value}</dd>
                                    <p class={metric.note_tone}>{metric.note}</p>
                                </div>
                            }
                        }}
                    </dl>
                    <div class="p-5 sm:p-7">
                        <div class="overflow-hidden rounded-xl border border-border">
                            {Table {
                                class: None,
                                children: Some(html_sync! {
                                    {TableHeader {
                                        class: None,
                                        children: Some(html_sync! {
                                            {TableRow {
                                                class: Some("bg-muted/50 hover:bg-muted/50"),
                                                children: Some(html_sync! {
                                                    {TableHead {
                                                        class: None,
                                                        children: Some("Product"),
                                                    }}
                                                    {TableHead {
                                                        class: None,
                                                        children: Some("Category"),
                                                    }}
                                                    {TableHead {
                                                        class: Some("text-right"),
                                                        children: Some("Stock"),
                                                    }}
                                                    {TableHead {
                                                        class: None,
                                                        children: Some("Status"),
                                                    }}
                                                    {TableHead {
                                                        class: Some("text-right"),
                                                        children: Some("Unit price"),
                                                    }}
                                                    {TableHead {
                                                        class: Some("text-right"),
                                                        children: Some("Restocked"),
                                                    }}
                                                }),
                                            }}
                                        }),
                                    }}
                                    {TableBody {
                                        class: None,
                                        children: Some(html_sync! {
                                            {for product in PRODUCTS {
                                                html_sync! {
                                                    {TableRow { class: None, children: Some(html_sync! {
                                                        {TableCell { class: None, children: Some(html_sync! {
                                                            <p class="font-medium text-foreground">{product.name}</p>
                                                            <p class="mt-0.5 font-mono text-xs text-muted-foreground">{product.sku}</p>
                                                        }), }}
                                                        {TableCell { class: Some("text-muted-foreground"), children: Some(product.category) }}
                                                        {TableCell { class: Some(product.status.count_tone()), children: Some(product.stock) }}
                                                        {TableCell { class: None, children: Some(Badge { variant: BadgeVariant::Outline, class: Some(product.status.badge()), children: Some(product.status.label()), }), }}
                                                        {TableCell { class: Some("text-right font-medium tabular-nums"), children: Some(product.price) }}
                                                        {TableCell { class: Some("text-right text-muted-foreground"), children: Some(product.restocked) }}
                                                    }), }}
                                                }
                                            }}
                                        }),
                                    }}
                                }),
                            }}
                        </div>
                        <div class="mt-4 flex flex-col gap-3 text-sm text-muted-foreground sm:flex-row sm:items-center sm:justify-between">
                            <p>"Showing 1–5 of 1,248 products"</p>
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
            </div>
        }
        .render(cx)
    }
}
