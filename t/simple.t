use strict;
use warnings;

use Test::More;

require_ok("XSDemo");

is XSDemo::hello(), "Hello from Rust XS demo", "greeting";

done_testing;
