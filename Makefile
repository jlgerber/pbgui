build:
	cargo build --release

install:
	cp ./target/release/pbgui ~/bin/.

install-stylesheet:
	cp ./resources/pbgui.qss ~/bin/.

rcc:
	rcc -binary ./resources/pbgui.qrc -o ./resources/pbgui.rcc

install-rcc:
	cp ./resources/pbgui.rcc ~/bin/. && rm ./resources/pbgui.rcc

all: build install install-stylesheet rcc install-rcc
