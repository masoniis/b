.PHONY: run clean

UNAME_S := $(shell uname -s)

UNAME_M := $(shell uname -m)

ifeq ($(UNAME_S),Linux)
ifeq ($(UNAME_M),aarch64)
	LIB := cbridge/libbcraft_linux_arm.a
else
	LIB := cbridge/libbcraft_linux.a
endif
else ifeq ($(UNAME_S),Darwin)
ifeq ($(UNAME_M),arm64)
	LIB := cbridge/libbcraft_macos_arm.a
else
	LIB := cbridge/libbcraft_macos.a
endif
else
	LIB := cbridge/libbcraft_windows.a
endif

run:
	gcc cbridge/main.c -o cbridge/main $(LIB) -Wl,-rpath,cbridge
	./main

clean:
	rm -f main
