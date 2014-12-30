compiler-rs
=========

A simple compiler for a simple toy language written in Rust.

Build using `cargo build`

The Language
---------
The language syntax is a simple imperative langauge with functional constructs
such as higher-order functions and lambdas. It is inspired by the WACC
language developed at Imperial College London for the second year BEng
Computing compilers project.

Changes from WACC include:
- Blocks use C style braces instead
- Types of variables in declarations can be derived from the right hand side
- Functions specify the return type after the arguments
- Functions can omit a return type
- Calling functions no longer requires the "call" keyword
- Functions can be called without storing the result
- Semi-colons are optional
- Functions can be declared inline (but are NOT closures... yet)
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

Lambda
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
func main() {
  var list = [1, 2, 3]
  var add = func(int x, int y) { return x + y }
  var sq = func(int x) { return x * x }
  println reduce(add, map(sq, list))
}
```
