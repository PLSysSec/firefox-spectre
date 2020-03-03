.NOTPARALLEL:
.PHONY : build build_debug clean test test_debug

.DEFAULT_GOAL := build

SHELL := /bin/bash

CURR_DIR := $(shell realpath ./)
ROOT_DIR := $(shell realpath ../)
OUT_DIR :=  $(shell realpath $(ROOT_DIR)/../out/firefox_builds/)

bootstrap:
	cd $(ROOT_DIR) && ./mach bootstrap --no-interactive --application-choice browser
	touch ./bootstrap

build:
	cp $(ROOT_DIR)/mozconfig_spectre $(ROOT_DIR)/mozconfig
	cd $(ROOT_DIR) && ./mach build

build_debug:
	cp $(ROOT_DIR)/mozconfig_spectre_debug $(ROOT_DIR)/mozconfig
	cd $(ROOT_DIR) && ./mach build

test:
	cp $(ROOT_DIR)/mozconfig_spectre $(ROOT_DIR)/mozconfig
	cd $(ROOT_DIR) && ./mach run https://jfkthame.github.io/test/udhr_urd.html

test_debug:
	cp $(ROOT_DIR)/mozconfig_spectre_debug $(ROOT_DIR)/mozconfig
	cd $(ROOT_DIR) && ./mach run https://jfkthame.github.io/test/udhr_urd.html

clean:
	-rm -rf $(OUT_DIR)/spectre