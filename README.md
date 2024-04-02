# Zoid Language

A simple programming language that compiles to machine code.

## Simplest Example

```zoid
fn main(): i32 {
    return 12 * 4 - 18 / 3;
}
```

## Installation

> Not yet supported

## Features

- Types

  The currently supported types are:

  | Type  | Literals                     | Description                |
  | ----- | ---------------------------- | -------------------------- |
  | `i32` | `[0-9]+`                     | Signed 32-bit integer type |
  | `f32` | `[0-9]+.[0-9]+` (and others) | 32-bit floating point type |

- Functions

  Defined by using the `fn` keyword with the function name and its arguments.
  The return type is specified after the `:` symbol. The arguments are specified
  with the identifier first followed by the type separated by a `:`.

  ```zoid
  fn <function name>(<arg1>: <type>, <arg2>: <type>): <return type> {
      // function body
  }
  ```

- Variables

  Defined by using the `let` keyword. This is followed by the identifier and the
  type of the variable separated by a `:`. These must have initialization which
  occur after the `=`.

  ```zoid
  let a: i32 = 5 * 2;
  ```

- Control Flow

  - `if` statements

    ```zoid
    if <condition> {
        // code block
    } else {
        // code block
    }
    ```

    Can also be used as `else if <condition>`

  - `while` loops

    ```zoid
    while <condition> {
        // code block
    }
    ```
