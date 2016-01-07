#[macro_use]
extern crate perl_xs;

use perl_xs::{ Context };
use perl_xs::raw::{ PerlContext, IV, CV };

#[no_mangle]
pub extern "C" fn hello(pthx: PerlContext, _: *mut CV) {
    let mut ctx = Context::new(pthx);
    ctx.prepush();
    ctx.push("Hello from Rust XS demo");
    ctx.putback();
}

#[no_mangle]
pub extern "C" fn boot_XSDemo(pthx: PerlContext, _: *mut CV) {
    let mut ctx = Context::new(pthx);
    ctx.new_xs("XSDemo::hello", hello);
    ctx.prepush();
    ctx.push(1 as IV);
    ctx.putback();
}
