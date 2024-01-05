work problem:
    cargo watch -x "check -p {{problem}}" -s "just test -p {{problem}}" -s "just lint {{problem}}" -s "just bench {{problem}}"
www-watch:
   RUST_LOG=info cargo +nightly leptos watch --project www
www-build:
   cargo +nightly leptos build --project www --release
lint problem:
    clippy-tracing --path "{{problem}}" --action check --exclude target --exclude benches
    cargo clippy -p {{problem}}
test +FLAGS='-p problem-01':
    cargo nextest run {{FLAGS}}
bench-all:
    cargo bench -q > benchmarks.txt
bench problem:
    cargo bench --bench {{problem}} >> {{problem}}.bench.txt
flamegraph problem:
    cargo flamegraph --profile flamegraph --package {{problem}} -o flamegraphs/{{problem}}.svg
dhat problem:
    cargo run --profile dhat --features dhat-heap --package {{problem}}
create problem:
    cargo generate --path ./problem-template --name {{problem}}