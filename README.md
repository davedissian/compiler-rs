compile-rs
=========
A compiler (front end for now) for a simple strongly typed programming language, allowing safe code.
The syntax is inspired by curly brace languages like Rust, and C++.

Build using `cargo build`

The Language
---------
TODO

Examples
---------

Hello World
```
func main() {
    println("Hello World")
}
```

Declare a variable
```
func main() {
    var x = 4
    println(x)
}
```

Function
```
func add(int x, int y) -> int {
    return x + y
}

func main() {
    println(add(2, 3))
}
```

Arrays
```
func main() {
    int[] array = [1, 2, 3, 4]
    var i = 0
    while i < len array {
        println(array[i])
        i = i + 1
    }
}
```

Structures (UDTs)
```
struct Data {
    int[] list
}

func main() {
    var d = Data([1, 2, 3])
    println d.list
}
```

Modules
```
use random

func main() {
    random.seed()
    println random.gen()
}
```

Type alias
```
alias myint int

func doSomething(myint i) {
    return i * 2
}

func main() {
    var i = 1 as myint
    println(doSomething(i))
}
```

Type cast
```
func bloat(int32 v) -> int64 {
    return v as int64
}

func main() {
    int32 v = 5
    println(bloat(v))
}
```

External Calls
```
alias libc_charptr int32
alias libc_int int

extern libc_printf(libc_charptr fmt, libc_int i) = printf

// Type safe wrapper
func printf(string str, int i) {
    enable reinterpret_cast {
        libc_printf(str as libc_charptr, i as libc_int)
    }
}
```

Cool extras
---------

Generics
```
struct Heap<T> {
    T data
}

func modify(Heap<int> h) {
    h.data = 6
}

func main() {
    var on_heap = Heap<int>(5)
    modify(on_heap)
    println on_heap.data
}
```

Lambda
```
func main() {
    var f = func(int x, int y) -> int {
        return x + y
    }

    println(f(2, 3))
}
```

Lambda #2
```
func map(func(int, int) -> int f, int[] list) -> int[] {
    for i = 0; i < len list; i++ {
        list[i] = f(list[i])
    }
    return list
}

func reduce(func(int, int) -> int f, int[] list) -> int {
    var acc = 0
    for i = 0; i < (len list) - 1; i++ {
        acc += f(list[i], list[i + 1])
    }
    return acc
}

func main() {
    var list = [1, 2, 3]
    var add = func(int x, int y) { return x + y }
    var sq = func(int x) { return x * x }
    println reduce(add, map(sq, list))
}
```

Closure
```
func main() {
    var acc = 0

    // type layout: func[<capture>](<type signature>)
    // 'capture' means how variables outside the closure scope are captured. Either 'ref' or 'copy'.
    var increment = func[ref](int x) {
        acc += x
    }
    increment(3)
    println(acc)
}
```