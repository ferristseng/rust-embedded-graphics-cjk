SHELL              	:= /bin/bash
TARGET 							:= target/font
SARASA_VERSION			:= $(shell cat SARASA_VERSION)
SARASA_ARCHIVE			:= sarasa-gothic-ttf-$(SARASA_VERSION)

all: $(TARGET)/$(SARASA_ARCHIVE)/UNARCHIVE

########## Setup ##########

$(TARGET):
	mkdir -p $(TARGET)

clean:
	rm -r $(TARGET)

########## Download Font ##########

$(TARGET)/SARASA_ZIP: SARASA_VERSION | $(TARGET)
	wget \
		https://github.com/be5invis/Sarasa-Gothic/releases/download/v$(SARASA_VERSION)/sarasa-gothic-ttf-$(SARASA_VERSION).7z \
		-O $(TARGET)/$(SARASA_ARCHIVE).7z
	touch $(TARGET)/SARASA_ZIP

$(TARGET)/$(SARASA_ARCHIVE)/UNARCHIVE: $(TARGET)/SARASA_ZIP
	7z e -o$(TARGET)/$(SARASA_ARCHIVE) $(TARGET)/$(SARASA_ARCHIVE).7z
	touch $(TARGET)/$(SARASA_ARCHIVE)/UNARCHIVE
