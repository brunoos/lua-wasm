# Using https://github.com/WebAssembly/wasi-sdk

CC=$(WASI_SDK_PATH)/bin/clang --sysroot=$(WASI_SDK_PATH)/share/wasi-sysroot

CFLAGS=-O3 -nostdlib -Wl,--initial-memory=67108864 -Wl,--no-entry -Wl,--allow-undefined -Wl,--export=adder

.PHONY: all clean

all: module.wasm

module.wasm: module.c
	$(CC) $(CFLAGS) -o $@ $<

clean:
	rm -f *.wasm
