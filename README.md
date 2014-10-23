simple-rs
=========

A very simple compiler for a very simple language, written in Rust

Simple
---------

A C like language based on a language called Mini, developed as part of the Topics 2013/14 project at Imperial College London (https://github.com/ICTeam28/ProgramAnalysis).

Changes from Mini include:
- Float, string and pointer types using #, $ and * respectively

Hello World
```
func write(str$) {
  syscall(1, str$);
}

func main() {
  write("Hello World\n");
}

```

Conditional
```
func main() {
  a = 1;
  b = 2;
  if a == b {
    write("They're the same!");
  } else {
    write("They're different!");
  }
}
```

Pointer
```
func main() {
  // Pointers! Yay!
  a = 1
  ptr* = ref a;
  deref ptr* = 2;
  
  // Test
  $str = tostring(a);
  write($str);
}
```

Return Values
```
func getInt() { return 3; }
func getFloat#() { return 3.0; }
func getString$() { return "Hello"; }
func getPtr*() { return 0x0; }

func main() {
  three# = getFloat();
  write(three#);
}
```

Operator Fuctions
```
// This will be defined in some standard library somewhere
op +(a, b) {
  result = __asm(i386) {
    ADD %0, %1
  }
  return result;
}

func main() {
  write(tostring(a + b))
}
```

Custom Operators
```
op mod(a, b) {
  ...
}

func main() {
  m = a mod b;
  write(tostring(m))
}
```
