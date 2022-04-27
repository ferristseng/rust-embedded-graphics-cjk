SHELL               := /bin/bash
TARGET              := target/font
SOURCE_DIR          := src
PNG_DIR             := png
FUSION_PIXEL_VERSION := $(shell cat FUSION_PIXEL_VERSION)
TTF2BITS            := ../target/debug/ttf2bits
TTF2BITS_PREFIX     := fusion_pixel
TTF2BITS_SIZES      := 12 24
TARGET_PNGS         := $(addsuffix .png, $(addprefix $(PNG_DIR)/$(TTF2BITS_PREFIX)-, $(TTF2BITS_SIZES)))
TARGET_SOURCES      := $(addsuffix .rs, $(addprefix $(SOURCE_DIR)/$(TTF2BITS_PREFIX)_, $(TTF2BITS_SIZES)))

all: $(TARGET_PNGS) $(TARGET_SOURCES)

########## Setup ##########

$(TARGET):
	mkdir -p $(TARGET)

$(TARGET)/fusion-pixel-$(FUSION_PIXEL_VERSION): $(TARGET)
	mkdir -p $(TARGET)/fusion-pixel-$(FUSION_PIXEL_VERSION)

$(PNG_DIR):
	mkdir -p $(PNG_DIR)

$(SOURCE_DIR)/bin:
	mkdir -p $(SOURCE_DIR)/bin

clean:
	rm -r $(TARGET) $(TARGET_PNGS) $(TARGET_SOURCES) $(SOURCE_DIR)/bin

########## Download Font ##########

$(TARGET)/FUSION_PIXEL_DOWNLOAD: FUSION_PIXEL_VERSION | $(TARGET)/fusion-pixel-$(FUSION_PIXEL_VERSION)
	wget \
		https://github.com/TakWolf/fusion-pixel-font/releases/download/$(FUSION_PIXEL_VERSION)/fusion-pixel.otf \
		-O $(TARGET)/fusion-pixel-$(FUSION_PIXEL_VERSION)/fusion-pixel.otf
	touch $(TARGET)/FUSION_PIXEL_DOWNLOAD

########## Build Source Files ##########

$(TARGET_PNGS): $(PNG_DIR)/$(TTF2BITS_PREFIX)-%.png: $(TARGET)/FUSION_PIXEL_DOWNLOAD $(PNG_DIR)
	$(TTF2BITS) \
		$(TARGET)/fusion-pixel-$(FUSION_PIXEL_VERSION)/fusion-pixel.otf \
		$(TTF2BITS_PREFIX) \
		png \
		-o $(PNG_DIR) \
		-s $*

$(TARGET_SOURCES): $(SOURCE_DIR)/$(TTF2BITS_PREFIX)_%.rs: $(TARGET)/FUSION_PIXEL_DOWNLOAD $(SOURCE_DIR)
	$(TTF2BITS) \
		$(TARGET)/fusion-pixel-$(FUSION_PIXEL_VERSION)/fusion-pixel.otf \
		$(TTF2BITS_PREFIX) \
		rs \
		-o $(SOURCE_DIR) \
		-s $*
