package XSDemo;
require XSLoader;

sub dies {
    die "ima dead";
}

XSLoader::load();
