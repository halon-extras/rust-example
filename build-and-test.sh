# https://doc.rust-lang.org/rustc/codegen-options/index.html#relro-level
RUSTFLAGS "-C relro-level=partial" cargo build && hsh --plugin hello hello.hsl
