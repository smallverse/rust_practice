fn main() {
    cxx_build::bridge("src/lib.rs") // returns a cc::Build
        //.flag_if_supported("-std=c++14")
        .flag_if_supported("-std=c++11")
        .compile("cxx-bridge-demo");
}
