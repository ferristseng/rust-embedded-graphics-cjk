SHELL               := /bin/bash
TARGET              := target/font
SOURCE_DIR          := src
PNG_DIR             := png
SARASA_VERSION      := $(shell cat SARASA_VERSION)
SARASA_ARCHIVE      := sarasa-gothic-ttf-$(SARASA_VERSION)
TTF2BITS            := ../target/debug/ttf2bits
TTF2BITS_PREFIX     := sarasa_mono_sc_light
TTF2BITS_SIZES      := 24 32 36
TARGET_PNGS         := $(addsuffix .png, $(addprefix $(PNG_DIR)/$(TTF2BITS_PREFIX)-, $(TTF2BITS_SIZES)))
TARGET_SOURCES      := $(addsuffix .rs, $(addprefix $(SOURCE_DIR)/$(TTF2BITS_PREFIX)_, $(TTF2BITS_SIZES)))

all: $(TARGET_PNGS) $(TARGET_SOURCES)

########## Setup ##########

$(TARGET):
	mkdir -p $(TARGET)

$(PNG_DIR):
	mkdir -p $(PNG_DIR)

$(SOURCE_DIR)/bin:
	mkdir -p $(SOURCE_DIR)/bin

clean:
	rm -r $(TARGET) $(TARGET_PNGS) $(TARGET_SOURCES) $(SOURCE_DIR)/bin

########## Download Font ##########

$(TARGET)/SARASA_ZIP: SARASA_VERSION | $(TARGET)
	wget \
		https://github.com/be5invis/Sarasa-Gothic/releases/download/v$(SARASA_VERSION)/sarasa-gothic-ttf-$(SARASA_VERSION).7z \
		-O $(TARGET)/$(SARASA_ARCHIVE).7z
	touch $(TARGET)/SARASA_ZIP

$(TARGET)/$(SARASA_ARCHIVE)/UNARCHIVE: $(TARGET)/SARASA_ZIP
	7z e -o$(TARGET)/$(SARASA_ARCHIVE) $(TARGET)/$(SARASA_ARCHIVE).7z sarasa-mono-sc-light.ttf
	touch $(TARGET)/$(SARASA_ARCHIVE)/UNARCHIVE

########## Build Source Files ##########

$(TARGET_PNGS): $(PNG_DIR)/$(TTF2BITS_PREFIX)-%.png: $(TARGET)/$(SARASA_ARCHIVE)/UNARCHIVE $(PNG_DIR)
	$(TTF2BITS) \
		$(TARGET)/$(SARASA_ARCHIVE)/sarasa-mono-sc-light.ttf \
		$(TTF2BITS_PREFIX) \
		png \
		-o $(PNG_DIR) \
		-s $*

$(TARGET_SOURCES): $(SOURCE_DIR)/$(TTF2BITS_PREFIX)_%.rs: $(TARGET)/$(SARASA_ARCHIVE)/UNARCHIVE $(SOURCE_DIR)
	$(TTF2BITS) \
		$(TARGET)/$(SARASA_ARCHIVE)/sarasa-mono-sc-light.ttf \
		$(TTF2BITS_PREFIX) \
		rs \
		-o $(SOURCE_DIR) \
		-s $*
