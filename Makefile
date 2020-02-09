resource_target ?= ~/bin/pbgui-resources

build:
	cargo build --release

install:
	cp ./target/release/pbgui ~/bin/.

install-stylesheet:
	cd ./pbgui && make install-stylesheet

install-prefs:
	cp ./etc/pbgui_preferences.yaml ~/etc/.

rcc:
	cd ./pbgui && make rcc
	cd ./pbgui-withs && make rcc
	cd ./pbgui-logger && make rcc

install-rcc:
	mkdir -p $(resource_target)
	cp ./pbgui/resources/pbgui.rcc $(resource_target)/. && rm ./pbgui/resources/pbgui.rcc
	cp ./pbgui-logger/resources/pbgui_logger.rcc $(resource_target)/. && rm ./pbgui-logger/resources/pbgui_logger.rcc
	cd ./pbgui-withs && make install-rcc


all: build install install-prefs install-stylesheet rcc install-rcc
