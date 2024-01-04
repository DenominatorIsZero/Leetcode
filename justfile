work problem:
    cargo watch -x "check -p {{problem}}" -s "just test -p {{problem}}" -s "just lint {{problem}}" -s "just bench {{problem}}" -s "just flamegraph {{problem}}"
www-watch:
   RUST_LOG=info cargo +nightly leptos watch --project www
www-build:
   cargo +nightly leptos build --project www --release
lint problem:
    clippy-tracing --action check --exclude target --exclude benches --exclude www
    cargo clippy -p {{problem}}
test +FLAGS='-p problem-01':
    cargo nextest run {{FLAGS}} {{part}}
bench-all:
    cargo bench -q > benchmarks.txt
bench problem:
    cargo bench --bench {{problem}} >> {{problem}}.bench.txt
flamegraph problem:
    cargo flamegraph --profile flamegraph --root --package {{problem}} -o flamegraphs/{{problem}}.svg
dhat problem:
    cargo run --profile dhat --features dhat-heap --package {{problem}}
create problem:
    cargo generate --path ./problem-template --name {{problem}}