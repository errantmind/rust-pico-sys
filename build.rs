extern crate cc;

fn main() {
    cc::Build::new()
        .file("extern/picohttpparser/picohttpparser.c")
        .include("extern/picohttpparser")
        .opt_level_str(&"fast")
        .flag("-funroll-loops")
        .flag("-msse4")
        //.flag("-flto")
        .flag("-march=native")
        .compile("libpicohttpparser.a");
}
