# just run
run *args:
    cargo run {{args}}

# Builds the c library
buildc:
	cargo build --release
	mv target/release/libbcraft.a cbridge/

runc *args:
    make
