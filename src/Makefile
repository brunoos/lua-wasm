LUA_INC?=/usr/include/lua5.4
WASMTIME_INC?=/usr/include/wasmtime
WASMTIME_LIB?=/usr/lib

CFLAGS=-Wall -O2 -I$(LUA_INC) -I$(WASMTIME_INC)
LDFLAGS=-L$(WASMTIME_LIB) -shared
LIBS=-lwasmtime

T=core.so

OBJS=wasm.o

.PHONY: all clean

all: $(T)

$(T): $(OBJS)
	@echo "[LD] $^ -> $@"
	@$(CC) $(LDFLAGS) -o $@ $^ $(LIBS)

%.o: %.c
	@echo "[CC] $< -> $@"
	@$(CC) $(CFLAGS) -c -o $@ $<

clean:
	@echo "[CLEAN] Removing artefacts: *.o *.so"
	@rm -f *.so *.o
