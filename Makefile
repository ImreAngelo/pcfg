.phony: all build clean test

all: build test

build:
	@cargo build --release

clean:
	@rm -r target

test: # TODO: Support non-UTF8 encoding
	@hyperfine './target/release/pcfg train data/rockyou-clean.txt'
# 	@hyperfine --warmup 3 './target/release/pcfg train data/rockyou-clean.txt'
