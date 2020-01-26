build:
	cargo build --example withlist --release

install:
	cp ./target/release/examples/withlist ~/bin/.

install-stylesheet:
	cp ./resources/withlist.qss ~/bin/.

rcc:
	rcc -binary ./resources/withlist.qrc -o ./resources/withlist.rcc

install-rcc:
	cp ./resources/withlist.rcc ~/bin/. && rm ./resources/withlist.rcc

all: build install install-stylesheet rcc install-rcc