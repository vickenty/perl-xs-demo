use inc::Module::Install;

name "XSDemo";
version "0.01";
abstract "Demonstration of Perl XS module written in Rust";

configure_requires "Module::Install::Rust" => 0;
build_requires "Test::More" => 0;
requires "XSLoader" => 0;

rust_use_perl_xs;
rust_write;

WriteAll;
