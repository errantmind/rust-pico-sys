#![deny(warnings)]

extern crate cc;

fn main() {
    #[allow(deprecated)]
    cc::Build::new()
        .file("extern/picohttpparser/picohttpparser.c")
        .opt_level_str(&"fast")
        .flag("-funroll-loops")
        .flag("-msse4")
        //.flag("-flto")
        .flag("-Ofast")
        .compile("libpico.a");
    // cc::compile_library(
    //     "libpico.a",
    //     &["extern/picohttpparser/picohttpparser.c"]);
}
