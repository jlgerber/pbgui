run:
	cargo run --example pbgui_tree --release

rcc:
	rcc -binary ./resources/pbgui_tree.qrc -o ./resources/pbgui_tree.rcc

# install-rcc:
# 	cp ./resources/pbgui_tree.rcc ~/bin/. && rm ./resources/pbgui_tree.rcc

all: rcc run