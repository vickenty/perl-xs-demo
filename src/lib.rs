#[macro_use]
extern crate perl_xs;

use std::ffi::CString;
use perl_xs::{ SV, IV, AV, Scalar, Array };

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

        sub array_head (ctx) {
            let nn = &CString::new("XSDemo::array").unwrap();

            let av: Option<AV> = ctx.get_av(nn);
            let iv = av.and_then(|av| av.fetch(0)).unwrap_or(-1);

            xs_return!(ctx, iv);
        }

        sub call_dies (ctx) {
            use perl_xs::{ G_DISCARD, G_VOID };
            ctx.call_pv(cstr!("XSDemo::dies"), G_DISCARD | G_VOID);
            xs_return!(ctx, 42 as IV);
        }
    }

    loader boot_XSDemo;
}
