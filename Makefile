build:
	cargo build --release

install:
	cp ./target/release/pbgui ~/bin/.

install-stylesheet:
	cp ./stylesheet/pbgui.qss ~/bin/.

rcc:
	rcc -binary pbgui.qrc -o pbgui.rcc

install-rcc:
	cp ./pbgui.rcc ~/bin/.

all: build install install-stylesheet rcc install-rcc
