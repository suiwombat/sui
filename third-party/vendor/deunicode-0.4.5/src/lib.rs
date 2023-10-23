//! The `deunicode` library transliterates Unicode strings such as "Æneid" into pure
//! ASCII ones such as "AEneid."
//!
//! It started as a Rust port of [`Text::Unidecode`](http://search.cpan.org/~sburke/Text-Unidecode-1.30/lib/Text/Unidecode.pm) Perl module, and was extended to support emoji.
//!
//! See [README](https://github.com/kornelski/deunicode/blob/master/README.md) for more info.
//!
//! Examples
//! --------
#![cfg_attr(feature = "alloc", doc = "```rust")]
#![cfg_attr(not(feature = "alloc"), doc = "```rust,ignore")]
//! use deunicode::deunicode;
//!
//! assert_eq!(deunicode("Æneid"), "AEneid");
//! assert_eq!(deunicode("étude"), "etude");
//! assert_eq!(deunicode("北亰"), "Bei Jing");
//! assert_eq!(deunicode("ᔕᓇᓇ"), "shanana");
//! assert_eq!(deunicode("げんまい茶"), "genmaiCha");
#![doc = "```"] // to mollify some syntax highlighters

#![no_std]

#[cfg(not(feature = "force-upgrade"))]
mod old;
#[cfg(not(feature = "force-upgrade"))]
pub use old::*;

#[cfg(feature = "force-upgrade")]
pub use deunicode1::*;
