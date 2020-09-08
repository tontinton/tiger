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
    }
    return fib(num - 1) + fib(num - 2);
}

fn mul(x: u32, y: u32) -> u32 {
    return x * y;
}

fn main() -> u32 {
    let a : u32 = 3 + 123 * 55;
    let b := 20; // Walrus operator for auto type inference
    let c := fib(b);
    if mul(a, 5) >= b + 2 {
        return c * 2;
    } else if a * 2 > 3 {
        return b + 1;
    }
    return 0;
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
            ident: fib
          variables: 
            [            
              (            
                declaration:
                  expression: 
                    ident: num
                  type: 
                    ident: u32
                  value: empty
              ),
            ]
      type: 
        ident: u32
      value: 
        [        
          (        
            if:
              condition: 
                <=:
                  left: 
                    ident: num
                  right: 
                    literal: 1
              then: 
                [                
                  (                
                    return: 
                        ident: num
                  ),
                ]
          ),        
          (        
            return: 
                +:
                  left: 
                    function:
                      name: 
                        ident: fib
                      variables: 
                        [                        
                          (                        
                            -:
                              left: 
                                ident: num
                              right: 
                                literal: 1
                          ),
                        ]
                  right: 
                    function:
                      name: 
                        ident: fib
                      variables: 
                        [                        
                          (                        
                            -:
                              left: 
                                ident: num
                              right: 
                                literal: 2
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
            ident: mul
          variables: 
            [            
              (            
                declaration:
                  expression: 
                    ident: x
                  type: 
                    ident: u32
                  value: empty
              ),            
              (            
                declaration:
                  expression: 
                    ident: y
                  type: 
                    ident: u32
                  value: empty
              ),
            ]
      type: 
        ident: u32
      value: 
        [        
          (        
            return: 
                *:
                  left: 
                    ident: x
                  right: 
                    ident: y
          ),
        ]
  ),
  (
    declaration:
      expression: 
        function:
          name: 
            ident: main
          variables: empty
      type: 
        ident: u32
      value: 
        [        
          (        
            declaration:
              expression: 
                ident: a
              type: 
                ident: u32
              value: 
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
                ident: b
              type: 
                ident: auto
              value: 
                literal: 20
          ),        
          (        
            declaration:
              expression: 
                ident: c
              type: 
                ident: auto
              value: 
                function:
                  name: 
                    ident: fib
                  variables: 
                    [                    
                      (                    
                        ident: b
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
                        ident: mul
                      variables: 
                        [                        
                          (                        
                            ident: a
                          ),                        
                          (                        
                            literal: 5
                          ),
                        ]
                  right: 
                    +:
                      left: 
                        ident: b
                      right: 
                        literal: 2
              else: 
                if:
                  condition: 
                    >:
                      left: 
                        *:
                          left: 
                            ident: a
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
                                ident: b
                              right: 
                                literal: 1
                      ),
                    ]
              then: 
                [                
                  (                
                    return: 
                        *:
                          left: 
                            ident: c
                          right: 
                            literal: 2
                  ),
                ]
          ),        
          (        
            return: 
                literal: 0
          ),
        ]
  ),
]
```