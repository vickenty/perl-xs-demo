use strict;
use warnings;

use Test::More;

require_ok("XSDemo");

is XSDemo::hello(), "Hello from Rust XS demo", "greeting";
my ($sum, $diff) = XSDemo::add_sub(3, 4);
is $sum, 7, "add_sub added";
is $diff, -1, "add_sub subtracted";

done_testing;
