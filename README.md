wacc-rs
=========

A compiler for a simple stringly typed programming language, heavily inspired by WACC. This compiler (will) have two front-ends, the new enhanced syntax and the syntax which matches the original WACC specification.

Build using `cargo build`

The Language
---------
The language syntax is a simple imperative langauge with functional constructs such as higher-order functions and lambdas. It is inspired by the WACC programming language developed at Imperial College London for the second year Computing compilers project.

Changes from WACC include:
- Blocks use C style braces instead
- Types of variables in declarations can be derived from the right hand side
- Functions specify the return type after the arguments
- Functions can omit a return type
- Calling functions no longer requires the "call" keyword
- Functions can be called without storing the result
- Semi-colons are optional
- Lambdas (which cannot capture variables) can be declared inline
- Pair types can be fully nested
- Comments now use // rather than #

Examples
---------

Hello World
```
func main() {
  println "Hello World"
}
```

Declare a variable
```
func main() {
  var x = 4
  println x
}
```

Function
```
func add(int x, int y) -> int {
  return x + y
}

func main() {
  println add(2, 3)
}
```

Pairs
```
func squish_pair(pair<int, int> p) {
  return (fst p) + (snd p)
}

func main() {
  pair<int, int> p = newpair(2, 3)

  // Print pair
  print fst p
  print " + "
  print snd p
  print " = "
  println squish_pair(p)

  free p
}
```

Arrays
```
func main() {
  int[] array = [1, 2, 3, 4]
  var i = 0
  while i < len array {
    println array[i]
    i = i + 1
  }
}
```

Extension Ideas
---------

Structs
```
struct Data {
  int[] list
}

func main() {
  var d = Data([1, 2, 3])
  println d.list
}
```

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

Modules
```
import random

func main() {
  random.seed()
  println random.gen()
}
```

Built-ins
```
struct Heap<T> {
  T data
}

func __assign__(Heap<int> h, int d) {
  h.data = d
}

func __show__(Heap<int> h) {
  print h.data
}

func main() {
  var x = Heap<int>(3)
  println x
  x = 4
  println x
}
```

Lambdas
```
func main() {
  var f = func(int x, int y) -> int {
    return x + y
  }

  println f(2, 3)
}
```

Higher Order Functions
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
