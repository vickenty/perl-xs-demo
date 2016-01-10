#[macro_use]
extern crate perl_xs;

use perl_xs::{ SV, IV, Scalar };

XS! {
    package XSDemo {
        sub hello (ctx) {
            xs_return!(ctx, "Hello from Rust XS demo");
        }

        sub add_sub (ctx) {
            let a: IV = ctx.st_fetch(0);
            let v: SV = ctx.st_fetch(1);
            let b = v.to_iv();
            xs_return!(ctx, a + b, a - b);
        }
    }

    loader boot_XSDemo;
}
