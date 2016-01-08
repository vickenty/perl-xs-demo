#[macro_use]
extern crate perl_xs;

XS! {
    package XSDemo {
        sub hello (ctx) {
            xs_return!(ctx, "Hello from Rust XS demo");
        }
    }

    loader boot_XSDemo;
}
