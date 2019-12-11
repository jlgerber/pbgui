build:
	cargo build --release

install:
	cp ./target/release/pbgui ~/bin/.

all: build install