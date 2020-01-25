resource_target ?= ~/bin/pbgui-resources

build:
	cargo build --release

install:
	cp ./target/release/pbgui ~/bin/.

install-stylesheet:
	cd ./pbgui && make install-stylesheet

rcc:
	cd ./pbgui && make rcc

install-rcc:
	mkdir -p $(resource_target)
	cp ./pbgui/resources/pbgui.rcc $(resource_target)/. && rm ./pbgui/resources/pbgui.rcc

all: build install install-stylesheet rcc install-rcc
