SHELL              	:= /bin/bash
TARGET 							:= target/font
ZPIX_VERSION				:= $(shell cat ZPIX_VERSION)

all: $(TARGET)/ZPIX_DOWNLOAD

########## Setup ##########

$(TARGET):
	mkdir -p $(TARGET)

$(TARGET)/zpix-$(ZPIX_VERSION):
	mkdir -p $(TARGET)/zpix-$(ZPIX_VERSION)

clean:
	rm -r $(TARGET)

########## Download Font ##########

$(TARGET)/ZPIX_DOWNLOAD: ZPIX_VERSION | $(TARGET)/zpix-$(ZPIX_VERSION)
	wget \
		https://github.com/SolidZORO/zpix-pixel-font/releases/download/v$(ZPIX_VERSION)/zpix.ttf \
		-O $(TARGET)/zpix-$(ZPIX_VERSION)/zpix.ttf
	touch $(TARGET)/ZPIX_DOWNLOAD
