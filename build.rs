#![deny(warnings)]

extern crate cc;

fn main() {
    #[allow(deprecated)]
    cc::Build::new()
        .file("extern/picohttpparser/picohttpparser.c")
        .include("extern/picohttpparser")
        .opt_level_str(&"fast")
        .flag("-funroll-loops")
        .flag("-msse4")
        //.flag("-flto")
        .flag("-march=native")
        .flag("-mtune=native")
        .compile("libpicohttpparser.a");
}
