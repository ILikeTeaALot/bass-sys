# About BASS-SYS Documentation

`/bass`, `/basscd`, `/bassloud`, and `/bassmix` were directly extracted from the corresponding `bass*.chm` files in this directory using 7zip (Keka on Mac).

These files are preprocessed by the build script `builld_src/doc_parser.rs` and turned into Rustdoc-compatible markdown.

<!-- Other than Rust-specific items like the [`AsDWORD`](crate::AsDWORD) trait, all the documentation for this crate is transposed from these files. -->

I'm unsure of the copyright implications of this, but it provides a significant degree of convenience for users of BASS-SYS (Assuming Rust Analyzer handles the `include_str!()` documentation properly...)

## Where to Get the Original Documentation

Online: [un4seen.com/doc](https://www.un4seen.com/doc/)

Included with the downloads for BASS modules ([un4seen.com downloads](https://www.un4seen.com/bass.html))