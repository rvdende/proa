# proa_ui mirror

A verbatim mirror of the public Free tier of the Proa UI registry
(`@proa`, served at <https://ui.proa.so/r/{name}.json>): every component and
block, plus the shared `foundation` support package. This keeps the source of
what `proa ui add` installs readable and reviewable in this repository.

Do not edit files here by hand. Refresh the mirror with:

```bash
python3 scripts/sync_proa_ui.py          # fetch, verify SHA-256, rewrite src/, public/, manifests/
python3 scripts/sync_proa_ui.py --check  # exit 1 if the mirror has drifted from the registry
```

## Layout

| Path | Contents |
|---|---|
| `src/components/` | Rust sources at their `@components` install targets; `mod.rs` lists every module from the manifests' `rustModules` |
| `src/components/_proa/` | `foundation`: shared support modules, license texts, third-party notices |
| `public/` | Assets at their `@assets` install targets (`proa-ui/proa-free.css`, `proa-ui/proa-free.js`) |
| `manifests/` | The registry manifests as served, one per slug |

The mirror is not a Cargo package or workspace member. The sources expect to
live at `crate::components` inside an application crate, the way
`proa ui add` installs them.

## Licensing

Free registry items are offered under `MIT OR Apache-2.0`; several blocks are
additionally adapted from MIT-licensed upstreams and say so in their headers.
See [`src/components/_proa/LICENSING.md`](src/components/_proa/LICENSING.md)
and [`src/components/_proa/THIRD_PARTY_NOTICES.md`](src/components/_proa/THIRD_PARTY_NOTICES.md).
The Pro tier (`@proa-pro`) is proprietary and authenticated, and is not
mirrored.
