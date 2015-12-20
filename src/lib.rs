#[macro_use]
extern crate perl_xs;

XS! {
    package XSDemo (boot_XSDemo) {
        sub hello(xs) {
            xs.prepush();
            xs.push_string("Hello from Rust XS demo");
            xs.putback();
        }
    }
}
