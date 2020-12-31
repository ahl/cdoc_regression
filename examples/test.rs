use fun::return_as_is;

fn main() {
    rust_style();
    c_style();
}

/// This is a multi-
/// line comment.
#[return_as_is]
fn rust_style() {}

/**
 * This is another multi-
 * line comment.
 */
#[return_as_is]
fn c_style() {}
