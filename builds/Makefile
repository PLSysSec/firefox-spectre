.NOTPARALLEL:
.PHONY : build clean test

.DEFAULT_GOAL := build

SHELL := /bin/bash

CURR_DIR := $(shell realpath ./)
ROOT_DIR := $(shell realpath ../)
OUT_DIR :=  $(shell realpath $(ROOT_DIR)/../out/firefox_builds/spectre)
# OUT_DIR := "/mnt/sata/ffbuilds/spectre"

bootstrap:
	cd $(ROOT_DIR) && ./mach bootstrap --no-interactive --application-choice browser
	touch ./bootstrap

$(OUT_DIR)/dist/bin/firefox:
	# cp $(ROOT_DIR)/mozconfig_spectre $(ROOT_DIR)/mozconfig
	cd $(ROOT_DIR) && ./mach build

build: $(OUT_DIR)/dist/bin/firefox

test:
	# cp $(ROOT_DIR)/mozconfig_spectre $(ROOT_DIR)/mozconfig
	cd $(ROOT_DIR) && ./mach run https://jfkthame.github.io/test/udhr_urd.html


clean:
	-rm -rf $(OUT_DIR)