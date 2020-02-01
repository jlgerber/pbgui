resource_target ?= ~/bin/pbgui-resources

install-stylesheet:
	cp ./resources/pbgui_logger.qss $(resource_target)/.

rcc:
	rcc -binary ./resources/pbgui_logger.qrc -o ./resources/pbgui_logger.rcc

install-rcc:
	cp ./resources/pbgui_logger.rcc $(resource_target)/. && rm ./resources/pbgui_logger.rcc

all: install-stylesheet rcc install-rcc