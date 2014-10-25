simple-rs
=========

A simple compiler for a simple toy language written in Rust.

Build using `cargo build`

To run a simple program:

  ./simplei hello.sp

Simple
---------

Simple is a C like language based on a language called Mini, developed as part of the Topics 2013/14 project at Imperial College London (https://github.com/ICTeam28/ProgramAnalysis).

Changes from Mini include:
- Operators are syntatic sugar for functions
- Internal hard coded functions are defined using ~

Conditional
```
func main() {
  a = 1;
  b = 2;
  return a == b;
}
```

```
// This will be defined in some standard library somewhere
func (+)(a, b) {
  ~add(a, b);
}

func (mod)(a, b) {
  ~mod(a, b);
}

func main() {
  a = 2 + 3;    // equiv. to (+)(2, 3)
  r = a mod 3;  // equiv. to (mod)(a, 3);
  return r;
}
```
