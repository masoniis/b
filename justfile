# just run
run *args:
    cargo run {{args}}

# just runc
# TODO: Runc runs the project with clang using the zig code as a lib
runc *args:
    zig run src/main.zig {{args}}
