target/release/a2: src/main.rs
	target/debug/a2 $F
		cargo build --release

.PHONY: build

build: target/release/a2

.PHONY: run

run: target/release/a2
	cargo run $F

.PHONY: clean

clean:
	cargo clean
