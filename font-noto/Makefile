SHELL               := /bin/bash
TARGET              := target/font
SOURCE_DIR          := src
PNG_DIR             := png
NOTO_SANS_VERSION   := $(shell cat NOTO_SANS_VERSION)
NOTO_SANS_ARCHIVE   := 13_NotoSansMonoCJKsc-$(NOTO_SANS_VERSION)
TTF2BITS            := ../target/debug/ttf2bits
TTF2BITS_PREFIX     := noto_sans_mono_sc_regular
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

$(TARGET)/NOTO_SANS_ARCHIVE: NOTO_SANS_VERSION | $(TARGET)
	wget \
		https://github.com/googlefonts/noto-cjk/releases/download/Sans$(NOTO_SANS_VERSION)/13_NotoSansMonoCJKsc.zip \
		-O $(TARGET)/$(NOTO_SANS_ARCHIVE).zip
	touch $(TARGET)/NOTO_SANS_ARCHIVE

$(TARGET)/$(NOTO_SANS_ARCHIVE)/UNARCHIVE: $(TARGET)/NOTO_SANS_ARCHIVE
	unzip $(TARGET)/$(NOTO_SANS_ARCHIVE).zip -d $(TARGET)/$(NOTO_SANS_ARCHIVE)
	touch $(TARGET)/$(NOTO_SANS_ARCHIVE)/UNARCHIVE

########## Build Source Files ##########

$(TARGET_PNGS): $(PNG_DIR)/$(TTF2BITS_PREFIX)-%.png: $(TARGET)/$(NOTO_SANS_ARCHIVE)/UNARCHIVE $(PNG_DIR)
	$(TTF2BITS) \
		$(TARGET)/$(NOTO_SANS_ARCHIVE)/NotoSansMonoCJKsc-Regular.otf \
		$(TTF2BITS_PREFIX) \
		png \
		-o $(PNG_DIR) \
		-s $*

$(TARGET_SOURCES): $(SOURCE_DIR)/$(TTF2BITS_PREFIX)_%.rs: $(TARGET)/$(NOTO_SANS_ARCHIVE)/UNARCHIVE $(SOURCE_DIR)
	$(TTF2BITS) \
		$(TARGET)/$(NOTO_SANS_ARCHIVE)/NotoSansMonoCJKsc-Regular.otf \
		$(TTF2BITS_PREFIX) \
		rs \
		-o $(SOURCE_DIR) \
		-s $*
