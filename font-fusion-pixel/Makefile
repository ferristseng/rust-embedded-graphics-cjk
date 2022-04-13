SHELL                := /bin/bash
TARGET               := target/font
FUSION_PIXEL_VERSION := $(shell cat FUSION_PIXEL_VERSION)

all: $(TARGET)/FUSION_PIXEL_DOWNLOAD

########## Setup ##########

$(TARGET):
	mkdir -p $(TARGET)

$(TARGET)/fusion-pixel-$(FUSION_PIXEL_VERSION):
	mkdir -p $(TARGET)/fusion-pixel-$(FUSION_PIXEL_VERSION)

clean:
	rm -r $(TARGET)

########## Download Font ##########

$(TARGET)/FUSION_PIXEL_DOWNLOAD: FUSION_PIXEL_VERSION | $(TARGET)/fusion-pixel-$(FUSION_PIXEL_VERSION)
	wget \
		https://github.com/TakWolf/fusion-pixel-font/releases/download/$(FUSION_PIXEL_VERSION)/fusion-pixel.otf \
		-O $(TARGET)/fusion-pixel-$(FUSION_PIXEL_VERSION)/fusion-pixel.otf
	touch $(TARGET)/FUSION_PIXEL_DOWNLOAD
