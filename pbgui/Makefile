resource_target ?= ~/bin/pbgui

build:
	cargo build --release

install:
	cp ../target/release/pbgui ~/bin/.

install-stylesheet:
	cp ./resources/pbgui.qss ~/bin/.

rcc:
	rcc -binary ./resources/pbgui.qrc -o ./resources/pbgui.rcc

install-rcc:
	mkdir -p $(resource_target)
	cp ./resources/pbgui.rcc $(resource_target)/. && rm ./resources/pbgui.rcc

all: build install install-stylesheet rcc install-rcc
