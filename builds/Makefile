.NOTPARALLEL:
.PHONY : build clean test

.DEFAULT_GOAL := build

SHELL := /bin/bash

CURR_DIR := $(shell realpath ./)
ROOT_DIR := $(shell realpath ../)
OUT_DIR :=  $(shell realpath $(ROOT_DIR)/../out/firefox_builds/)

bootstrap:
	cd $(ROOT_DIR) && ./mach bootstrap --no-interactive --application-choice browser
	touch ./bootstrap

$(OUT_DIR)/stock/dist/bin/firefox:
	cp $(ROOT_DIR)/mozconfig_stock $(ROOT_DIR)/mozconfig
	cd $(ROOT_DIR) && ./mach build

build: $(OUT_DIR)/stock/dist/bin/firefox

clean:
	-rm -rf $(OUT_DIR)/stock