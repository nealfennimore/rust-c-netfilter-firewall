CC = gcc

INCLUDE_PATHS := $(shell cat .vscode/c_cpp_properties.json | jq -r '.configurations[0].includePath[1:] | join(" ")')
INCLUDES=$(INCLUDE_PATHS:%=-I%)

# The CFLAGS variable sets compile flags for gcc:
#  -g          compile with debug information
#  -Wall       give verbose compiler warnings
CFLAGS += -g -Wall $(INCLUDES)

ccflags-y := $(ccflags-y) -xc -E -v

SRC_DIR = src
BUILD_DIR = build

SOURCES = $(SRC_DIR)/main.c
OBJECTS = $(SOURCES:.cpp=.o)

TARGET = firewall_hook
BUILD_TARGET = $(BUILD_DIR)/$(TARGET)

all: $(TARGET)

$(TARGET):
	$(CC) -o $(BUILD_TARGET) $(SOURCES) $(CFLAGS) -lnetfilter_queue

.PHONY: clean

clean:
	rm -f $(BUILD_TARGET) $(OBJECTS) core