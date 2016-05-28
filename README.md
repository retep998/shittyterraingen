Compile this by doing `cargo rustc --release -- --crate-type=cdylib`. The incantation is only so complex because cargo doesn't support cdylib properly yet, it'll get simpler very soon.

Note that because cdylib is relatively new you will need the latest nightly version of Rust.
