#[macro_use]
extern crate perl_xs;

XS! {
    package XSDemo {
        sub hello (ctx) {
            ctx.prepush();
            ctx.push("Hello from Rust XS demo");
            ctx.putback();
        }
    }

    loader boot_XSDemo;
}
