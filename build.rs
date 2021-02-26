#![deny(warnings)]

extern crate cc;

fn main() {
    #[allow(deprecated)]
    cc::Build::new()
        .file("extern/picohttpparser/picohttpparser.c")
        .opt_level(3)
        .flag("-funroll-loops")
        //.flag("-msse4")
        .compile("libpico.a");
    // cc::compile_library(
    //     "libpico.a",
    //     &["extern/picohttpparser/picohttpparser.c"]);
}
