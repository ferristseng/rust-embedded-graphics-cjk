SHELL              	:= /bin/bash
TARGET 							:= target/font
NOTO_SANS_VERSION		:= $(shell cat NOTO_SANS_VERSION)
NOTO_SANS_ARCHIVE		:= 13_NotoSansMonoCJKsc-$(NOTO_SANS_VERSION)

all: $(TARGET)/$(NOTO_SANS_ARCHIVE)/UNARCHIVE

########## Setup ##########

$(TARGET):
	mkdir -p $(TARGET)

clean:
	rm -r $(TARGET)

########## Download Font ##########

$(TARGET)/NOTO_SANS_ARCHIVE: NOTO_SANS_VERSION | $(TARGET)
	wget \
		https://github.com/googlefonts/noto-cjk/releases/download/Sans$(NOTO_SANS_VERSION)/13_NotoSansMonoCJKsc.zip \
		-O $(TARGET)/$(NOTO_SANS_ARCHIVE).zip
	touch $(TARGET)/NOTO_SANS_ARCHIVE

$(TARGET)/$(NOTO_SANS_ARCHIVE)/UNARCHIVE: $(TARGET)/NOTO_SANS_ARCHIVE
	unzip $(TARGET)/$(NOTO_SANS_ARCHIVE).zip -d $(TARGET)/$(NOTO_SANS_ARCHIVE)
	touch $(TARGET)/$(NOTO_SANS_ARCHIVE)/UNARCHIVE
