This is a test case to show the difference between Rust 1.46 and 1.47
in terms of how c-style doc comments are processed and passed to an
attribute macro.

To demonstrate:

* rustup install 1.46.0
* rustup install 1.47.0
* cargo +1.46.0 run --example test
* cargo +1.47.0 run --example test
