fn main() {
    let bridge = cxx_build::bridge("src/lib.rs")
        .include("src")
        .file("src/lib.cpp")
        .compile("cxx-enum-namespace");
}