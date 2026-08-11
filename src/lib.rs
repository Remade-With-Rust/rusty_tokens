#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Design tokens for Rust UIs -- semantic CSS custom-property names + neutral defaults.
//!
//! Product twin of the tokens module in [`thoth`](https://github.com/Remade-With-Rust/thoth).
//! Token *names* are the CSS contract (`--rt-*`); default *values* are a small ASCII
//! starter theme. Optional feature `css` emits a `:root` stylesheet.

pub mod color;
pub mod radius;
pub mod space;
pub mod type_scale;

#[cfg(feature = "css")]
#[cfg_attr(docsrs, doc(cfg(feature = "css")))]
pub mod css;
