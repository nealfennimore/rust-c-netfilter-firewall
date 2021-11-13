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

.PHONY: build clean install uninstall _run run

all: build run

build:
	$(CC) -o $(BUILD_TARGET) $(SOURCES) $(CFLAGS) -lnetfilter_queue

clean:
	rm -f $(BUILD_TARGET) $(OBJECTS) core

install:
	sudo iptables -A OUTPUT -d 127.0.0.1 -p tcp --dport 9999 -j NFQUEUE --queue-num 0

uninstall:
	-sudo iptables -D OUTPUT -d 127.0.0.1 -p tcp --dport 9999 -j NFQUEUE --queue-num 0

_run:
	-sudo ./$(BUILD_TARGET)

run: uninstall install _run

rust: build_rust run_rust

_run_rust:
	-sudo ./target/release/rust_packet_filter

run_rust: uninstall install _run_rust

build_rust:
	-cargo build --release

run_rust: uninstall install _run_rust