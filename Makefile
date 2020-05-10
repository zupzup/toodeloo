build:
	@cargo build

clean:
	@cargo clean

TESTS = ""
test:
	@cargo test $(TESTS) --offline --lib -- --color=always --nocapture

docs: build
	@cargo doc --no-deps

mongostart:
	@sudo docker run -d -p 27017:27017 -v `pwd`/data/db:/data/db --name toodeloodb mongo

mongostop:
	@sudo docker stop toodeloodb && sudo docker rm toodeloodb

style-check:
	@rustup component add rustfmt 2> /dev/null
	cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	cargo clippy --all-targets --all-features -- -D warnings

dev:
	cargo run

.PHONY: build test docs style-check lint
