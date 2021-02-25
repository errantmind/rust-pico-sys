#![deny(warnings)]

extern crate gcc;

fn main() {
    #[allow(deprecated)]
    gcc::compile_library(
        "libpico.a",
        &["extern/picohttpparser/picohttpparser.c"]);
}
