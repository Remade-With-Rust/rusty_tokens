#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Design tokens for Rust UIs -- semantic CSS custom-property names + neutral defaults.
//!
//! Token *names* are the CSS contract (`--rt-*`); default *values* are a small ASCII
//! starter theme. Optional feature `css` emits a `:root` stylesheet.
//!
//! By default installs [`rusty_alloc`](https://github.com/Remade-With-Rust/rusty_alloc)
//! via [`rusty_alloc_default`](https://crates.io/crates/rusty_alloc_default)
//! (opt out with `default-features = false`). Safe to combine with
//! `rusty_symbols` / `rusty_a11y` defaults -- one shared allocator link.

/// Whether this build pulled in the default `rusty_alloc` install.
pub const fn rusty_alloc_enabled() -> bool {
    cfg!(feature = "rusty-alloc")
}

/// Whether the hardened `secure` allocator profile is compiled in.
pub const fn secure_allocator_enabled() -> bool {
    cfg!(feature = "secure")
}

pub mod color;
pub mod radius;
pub mod space;
pub mod type_scale;

#[cfg(feature = "css")]
#[cfg_attr(docsrs, doc(cfg(feature = "css")))]
pub mod css;
