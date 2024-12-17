clean:
	cargo clean

build:
	cargo build --release
	
test:
	cargo test

code-cov:
	cargo llvm-cov --open