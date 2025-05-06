CXX	      = g++
CXXFLAGS  = -std=c++17 -O2
SRC	      = $(wildcard src/*.cpp)
OBJ	      = $(SRC:.cpp=.o)
OUT       = b2a.out
PREFIX    = /usr/local

.PHONY: install debug-run

all: $(OUT)

$(OUT): $(OBJ)
	$(CXX) $(CXXFLAGS) -o $@ $^

clean:
	rm -f $(OBJ) $(OUT)

install: $(OUT)
	install -Dm755 $(OUT) $(DESTDIR)$(PREFIX)/bin/$(OUT:.out=)

debug-run: clean $(OUT)
	gdb ./$(OUT)
