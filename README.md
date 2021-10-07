## Baasha

Yet another programming language inspired from Rust, Golang and C++.

The name is inpisred from *Rajinikanth* cult film `Baasha` because it is a semi-homophonic variant of the hindi word `Bhasha` which means language.

Supported features:

- Types: `int8`, `int16`, `int32`, `int64`, `uint8`, `uint16`, `uint32`, `uint64`, `float32`, `float64`, `bool`
- Aggregate or object oriented features similar to `Rust`: `struct`, `impl` & `trait`
- Number literals are `int64` by default.

Future support:
- Arrays, strings, characters
- Green threads (similar to goroutines in golang)

## How to use

- Download the relevant zip folder for your system from [Releases](https://github.com/rahulv4667/baasha/releases/tag/0.0.1)
- Add the `baasha` executable to your system `PATH`
- create a `.bs` file and write your program in it.
- Run `baasha -f <filename>`

**Other commmand line options**
```
baasha [OPTIONS] 

    --filename, -f          Takes the input .bs file
    --emit-tokens, -t       Emits a list of lexical tokens
    --emit-parse-tree, -p   Emits AST after initial parsing
    --emit-typed-tree, -d   Emits AST after type checking
    --emit-llvm, -l         Emits LLVM IR of the given code
    --output, -o            The filename of executable file name
```


## Project setup

- Install `Python3`, `Rust`, `LLVM 12`
- Run `git clone https://github.com/rahulv4667/baasha`


### Example

```
struct Point {
    x: float32,
    y: float64
}

impl Point {
    func print() -> int64 {
        printf32(self.x);
        println();
        printf64(self.y);
        println();
        return 0;
    }
}

func main() -> int64 {
    var x: int32;
    var p: Point;

    x = scani32();
    printi32(x);
    println();

    p = Point {
        x: 20.2 as float32,
        y: 20.5 as float64,
    };

    p.print();

    p.x = scanf32();
    p.y = scanf64();
    
    p.print();

    printf32(p.x + p.y as float32);
    println();
    
    return 0;
}
```

**Output**
```
inp> 10
out> 10
out> 20.200001
out> 20.500000
inp> 20.5
inp> 20.5
out> 20.500000
out> 20.500004
out> 41.000004
```