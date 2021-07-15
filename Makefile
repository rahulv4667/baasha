CC  = clang++-12
CFLAGS	= -g -O3 -Xlinker --export-dynamic
LLVMCONFFLAGS = `llvm-config-12 -cxxflags --ldflags --system-libs --libs core`
LIB=-L/usr/bin -L/usr/lib/llvm-12/bin -L/usr/lib/llvm-12/lib -L/usr/lib/llvm-12/build/lib
INC=-I/usr/include/llvm-12 -I/usr/include/llvm-c-12 -I/usr/include/clang/12/include

default: main

main: main.cpp ast.cpp parser.cpp lexer.cpp globals.hpp
	$(CC) $(LIB) $(INC) $(CFLAGS) main.cpp $(LLVMCONFFLAGS) -o $@

clean:
	rm main
