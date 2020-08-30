# IN DEVELOPMENT

![Rust](https://github.com/tontinton/tiger/workflows/Rust/badge.svg?branch=master)

## Introduction
The tiger language is a language made for exploitations.

It's main focus is **minimal code size** through code size 
optimizations and compiling directly to a **shellcode** (position independent code).

## What can it do currently? 
Currently, the tiger binary can only parse a single file to an AST representation.

When running the tiger compiler on test.tg
```bash
cargo build
./target/debug/tiger test.tg
```

when ``test.tg`` looks like:
```rust
fn fib(num: u32) -> u32 {
    if num <= 1 {
        return num;
    } else {
        return fib(num - 1) + fib(num - 2);
    }
}

fn mul(x: u32, y: u32) -> u32 {
    return x * y;
}

fn main() -> void {
    let a : u32 = 3 + 123 * 55;
    let b := 20; // Walrus operator for auto type inference
    let c := fib(b);
    if mul(a, 5) >= b + 2 {
        return c * 2;
    } else if a * 2 > 3 {
        return b + 1;
    }
}
```

The result is:
```yaml
[
  (
    declaration:
      expression:
        function:
          name:
            literal: fib
          variables:
            [
              (
                declaration:
                  expression:
                    literal: num
                  type:
                    literal: u32
                  value: empty
              ),
            ]
      type:
        literal: u32
      value:
        [
          (
            if:
              condition:
                <=:
                  left:
                    literal: num
                  right:
                    literal: 1
              then:
                [
                  (
                    return:
                        literal: num
                  ),
                ]
              else:
                [
                  (
                    return:
                        +:
                          left:
                            function:
                              name:
                                literal: fib
                              variables:
                                [
                                  (
                                    -:
                                      left:
                                        literal: num
                                      right:
                                        literal: 1
                                  ),
                                ]
                          right:
                            function:
                              name:
                                literal: fib
                              variables:
                                [
                                  (
                                    -:
                                      left:
                                        literal: num
                                      right:
                                        literal: 2
                                  ),
                                ]
                  ),
                ]
          ),
        ]
  ),
  (
    declaration:
      expression:
        function:
          name:
            literal: mul
          variables:
            [
              (
                declaration:
                  expression:
                    literal: x
                  type:
                    literal: u32
                  value: empty
              ),
              (
                declaration:
                  expression:
                    literal: y
                  type:
                    literal: u32
                  value: empty
              ),
            ]
      type:
        literal: u32
      value:
        [
          (
            return:
                *:
                  left:
                    literal: x
                  right:
                    literal: y
          ),
        ]
  ),
  (
    declaration:
      expression:
        function:
          name:
            literal: main
          variables: empty
      type:
        literal: void
      value:
        [
          (
            declaration:
              expression:
                literal: a
              type:
                literal: u32
              value:
                =:
                  left:
                    literal: a
                  right:
                    +:
                      left:
                        literal: 3
                      right:
                        *:
                          left:
                            literal: 123
                          right:
                            literal: 55
          ),
          (
            declaration:
              expression:
                literal: b
              type:
                literal: auto
              value:
                literal: 20
          ),
          (
            declaration:
              expression:
                literal: c
              type:
                literal: auto
              value:
                function:
                  name:
                    literal: fib
                  variables:
                    [
                      (
                        literal: b
                      ),
                    ]
          ),
          (
            if:
              condition:
                >=:
                  left:
                    function:
                      name:
                        literal: mul
                      variables:
                        [
                          (
                            literal: a
                          ),
                          (
                            literal: 5
                          ),
                        ]
                  right:
                    +:
                      left:
                        literal: b
                      right:
                        literal: 2
              then:
                [
                  (
                    return:
                        *:
                          left:
                            literal: c
                          right:
                            literal: 2
                  ),
                ]
              else:
                if:
                  condition:
                    >:
                      left:
                        *:
                          left:
                            literal: a
                          right:
                            literal: 2
                      right:
                        literal: 3
                  then:
                    [
                      (
                        return:
                            +:
                              left:
                                literal: b
                              right:
                                literal: 1
                      ),
                    ]
          ),
        ]
  ),
]
```