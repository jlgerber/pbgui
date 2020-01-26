resource_target ?= ~/bin/pbgui-resources

build:
	cargo build --example withlist --release

install:
	cp ./target/release/examples/withlist ~/bin/.

install-stylesheet:
	cp ./resources/withlist.qss $(resource_target)/.

rcc:
	rcc -binary ./resources/pbgui_withlist.qrc -o ./resources/pbgui_withlist.rcc

install-rcc:
	cp ./resources/pbgui_withlist.rcc $(resource_target)/. && rm ./resources/pbgui_withlist.rcc

all: build install install-stylesheet rcc install-rcc