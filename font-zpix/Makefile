SHELL               := /bin/bash
TARGET              := target/font
SOURCE_DIR          := src
PNG_DIR             := png
ZPIX_VERSION        := $(shell cat ZPIX_VERSION)
TTF2BITS            := ../target/debug/ttf2bits
TTF2BITS_PREFIX     := zpix
TTF2BITS_SIZES      := 12 24
TARGET_PNGS         := $(addsuffix .png, $(addprefix $(PNG_DIR)/$(TTF2BITS_PREFIX)-, $(TTF2BITS_SIZES)))
TARGET_SOURCES      := $(addsuffix .rs, $(addprefix $(SOURCE_DIR)/$(TTF2BITS_PREFIX)_, $(TTF2BITS_SIZES)))

all: $(TARGET_PNGS) $(TARGET_SOURCES)

########## Setup ##########

$(TARGET):
	mkdir -p $(TARGET)

$(TARGET)/zpix-$(ZPIX_VERSION): $(TARGET)
	mkdir -p $(TARGET)/zpix-$(ZPIX_VERSION)

$(PNG_DIR):
	mkdir -p $(PNG_DIR)

$(SOURCE_DIR)/bin:
	mkdir -p $(SOURCE_DIR)/bin

clean:
	rm -r $(TARGET) $(TARGET_PNGS) $(TARGET_SOURCES) $(SOURCE_DIR)/bin

########## Download Font ##########

$(TARGET)/ZPIX_DOWNLOAD: ZPIX_VERSION | $(TARGET)/zpix-$(ZPIX_VERSION)
	wget \
		https://github.com/SolidZORO/zpix-pixel-font/releases/download/v$(ZPIX_VERSION)/zpix.ttf \
		-O $(TARGET)/zpix-$(ZPIX_VERSION)/zpix.ttf
	touch $(TARGET)/ZPIX_DOWNLOAD

########## Build Source Files ##########

$(TARGET_PNGS): $(PNG_DIR)/$(TTF2BITS_PREFIX)-%.png: $(TARGET)/ZPIX_DOWNLOAD $(PNG_DIR)
	$(TTF2BITS) \
		$(TARGET)/zpix-$(ZPIX_VERSION)/zpix.ttf \
		$(TTF2BITS_PREFIX) \
		png \
		-o $(PNG_DIR) \
		-s $*

$(TARGET_SOURCES): $(SOURCE_DIR)/$(TTF2BITS_PREFIX)_%.rs: $(TARGET)/ZPIX_DOWNLOAD $(SOURCE_DIR)
	$(TTF2BITS) \
		$(TARGET)/zpix-$(ZPIX_VERSION)/zpix.ttf \
		$(TTF2BITS_PREFIX) \
		rs \
		-o $(SOURCE_DIR) \
		-s $*
