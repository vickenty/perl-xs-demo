use strict;
use warnings;

use Test::More;

require_ok("XSDemo");

is XSDemo::hello(), "Hello from Rust XS demo", "greeting";
my ($sum, $diff) = XSDemo::add_sub(3, 4);
is $sum, 7, "add_sub added";
is $diff, -1, "add_sub subtracted";

is XSDemo::array_head(), -1, "array_head with non-existant array";

eval '@XSDemo::array = ()';
is XSDemo::array_head(), -1, "array_head with empty array";

eval '@XSDemo::array = (42..52)';
is XSDemo::array_head(), 42, "array_head with filled array";

done_testing;
