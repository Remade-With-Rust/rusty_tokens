# rusty_tokens

[![Remade With Rust](https://img.shields.io/badge/Remade%20With-Rust-000?logo=rust&logoColor=fff)](https://github.com/remade-with-rust)
[![By Mata Network](https://img.shields.io/badge/by-Mata%20Network-5b2be0)](https://www.mata.network)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE-MIT)
![Platforms: Windows · macOS · Linux · Web · WASM](https://img.shields.io/badge/platforms-Windows%20%C2%B7%20macOS%20%C2%B7%20Linux%20%C2%B7%20Web%20%C2%B7%20WASM-informational)
![MSRV: 1.73](https://img.shields.io/badge/MSRV-1.73-informational)

> **rusty_tokens** is an open-source design-token toolkit for any Rust UI --
> **semantic CSS custom-property names + neutral default values + an optional
> `:root` sheet emitter** -- pure Rust, zero dependencies. Application chrome
> gets one shared theme contract instead of scattered hex/rem literals.
> Sibling of [`thoth`](https://github.com/Remade-With-Rust/thoth) (glyphs) and
> [`rusty_a11y`](https://github.com/Remade-With-Rust/rusty_a11y) (ARIA helpers).

> **Status -- v0.1.0.** Pin
> `rusty_tokens = "0.1"` from crates.io (or git tag `v0.1.0`).
> Core is `no_std` / wasm-checked. Feature `css` needs `alloc`.

---

## The headline

> **One theme contract.** Token names are the CSS API (`--rt-color-fg`).
> Defaults are a small ASCII starter; apps override via CSS. Optional
> `css::root_sheet()` injects a full `:root` block into WebView / Dioxus.

| Dimension | Scattered hex/rem | **rusty_tokens** | Goal |
|---|:---:|:---:|:---:|
| Naming | ad-hoc | **semantic modules** | maintain |
| CSS contract | none | **`--rt-*` custom properties** | uniform |
| Defaults | copy-paste | **neutral ASCII starter** | portable |
| WebView inject | hand-written CSS | **`css::root_sheet`** | opt-in |
| Dependencies | -- | **none** | maintain |
| License | mixed | **MIT** | -- |

---

## Install

```toml
rusty_tokens = "0.1"
# CSS :root emitter:
# rusty_tokens = { version = "0.1", features = ["css"] }
# git:
# rusty_tokens = { git = "https://github.com/Remade-With-Rust/rusty_tokens.git", tag = "v0.1.0" }
```

| Feature | Default | Provides |
|---------|---------|----------|
| *(none)* | -- | `color` / `space` / `type_scale` / `radius` consts |
| `css` | no | `css::root_sheet` |

Always on: pure-ASCII source, `no_std` core, zero deps.

MSRV: **1.73**.

## Quick start

```rust
use rusty_tokens::color;

fn fg_var() -> &'static str {
    color::FG // "--rt-color-fg"
}

fn fg_default() -> &'static str {
    color::FG_VALUE // "#1a1a1a"
}
```

```rust
// feature = "css"
use rusty_tokens::css;

fn inject_theme() -> String {
    css::root_sheet()
}
```

```sh
cargo test
cargo test --features css
```

## Features

- **color** -- fg / bg / muted / accent / success / danger / warn / border (+ `_VALUE`).
- **space** -- xs / sm / md / lg / xl rem steps.
- **type_scale** -- caption / body / title / display sizes.
- **radius** -- sm / md / lg corner radii.
- **css** -- `:root` sheet emitter (`css` feature).
- **Guards** -- ASCII source self-test; value contract tests.

### Capability table

| Capability | Status |
|---|---|
| Semantic token name consts (`--rt-*`) | done |
| Neutral default values (hex / rem) | done |
| CSS `:root` emitter | done feature `css` |
| ASCII self-test | done |
| `no_std` + wasm | done (`css` needs `alloc`) |
| crates.io | done v0.1.0 |

## Architecture

```text
┌──────────────────────────────────────────────────────────┐
│  rusty_tokens                                            │
│                                                          │
│  color / space / type_scale / radius                  ✅ │
│  css::root_sheet  [feature css]                       ✅ │
└──────────────────────────────────────────────────────────┘
```

Northern star: one CSS custom-property contract apps can override without
rewriting Rust. Sibling glyph toolkit: [thoth](https://github.com/Remade-With-Rust/thoth).

## Platform support

| Platform | Status |
|---|---|
| Windows | yes |
| macOS | yes |
| Linux | yes |
| Web (Dioxus / browsers) | yes |
| WASM (`wasm32-unknown-unknown`) | yes (`no_std` core; `css` needs `alloc`) |

No OS APIs. Inject `root_sheet()` into WebView / Dioxus document head; override
any `--rt-*` property afterward.

## Remade With Rust

**Remade With Rust** ([Mata Network](https://www.mata.network)) rebuilds essential
tooling in Rust -- memory safety, predictable performance, permissive license.

-> **[github.com/remade-with-rust](https://github.com/remade-with-rust)**

Family: [thoth](https://github.com/Remade-With-Rust/thoth) ·
**rusty_tokens** ·
[rusty_a11y](https://github.com/Remade-With-Rust/rusty_a11y)

## License

MIT -- [LICENSE-MIT](LICENSE-MIT).

## Trademark

"Remade With Rust", "Mata", and "Mata Network" are marks of Mata Network.
