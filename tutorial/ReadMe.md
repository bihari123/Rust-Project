## Cargo

```
$ cargo new hello // new rust project

$ tree --noreport hello

hello
├── Cargo.toml // a config file in TOML format (TOML = tom's obvious minimal language)
└── src
    └── main.rs

```

here is what Cargo.toml looks like

```
[package]
name = "hello"  // name of the project
version = "0.1.0" // version
edition = "2021"
// you can add the author's name or if it is a github project then it will automatically search your global git config and fill it
authors = [ "Tarun Thakur <tarun.thakur@gmail.com>"]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

let's write a hello world program

```
fn main(){
        println!("hello, world"); // semi-colon bitches !
    }
```

Now lets rin the program

```
$ cargo run

// cargo will recompile only if there is a change in the rust code
// it saves the executable in /target/debug/hello

$ cargo run --release
// sends the output to /target/release/hello
```

## Variable

```
fn main(){
        let bunnies =2; // use let keyword to declare a variable
    }
```

Rust is a strongly typed language. So where is the type annotation?
Once Rust figures out the type annotation , it keep the data-type of the the variable same throughout.
Also you can give your own type

```

let bunnies: i32 =4;
```

let keyword can also help us define multiple variables at once

```
let (bunnies, carrots)= (8,50);
```

variable are immutable by default in Rust.

But why did Rust choose to make the variables immutable by default? **_Safety, Concurrency, Speed_**

If the data is not changing, then there is a very less chance of bugs, also data that is not changing can be shared by threads and the compiler can also do extra optimisation on data that it knows will never change.

### mutable variables

```
let mut bunnies =32; // mut keyword
bunnies=2;
```

### Constants

There are some rules of defining the constants in rust:

- the keyword **const**
- the variable name should be uppercase separated by underscore
- the type annotation is required
- the value must be a constant expression that can be determined at the compliled time
- teh constant name is all caps with underscore.

### Why do you bother to use a constant?

- you can put a constant outside the scope of the functions.
- they are really fast.

### Isn't the constant and immutable variable same ?

- Constants can be defined outside a function scope.
- constant data is **_required_** to be defined at the compile time. However, with variable it is not necessary. You can either define your varibale as v = 5 or v = Sum(4,5)

## Scope

The scope of a variable begins where it is created and ends at the end of the block. On the way, it is accessible in the nested blocks. A block is a space inside curly braces. In rust, there is no garbage collector, the valuues are immediately dropped after we go out of scope.

### variable shadowing

Variables are always local to their scope. let's take the following example.

```
fn main(){
        let x =5 ;
        {
                let x =99;
                println!("{}",x); // Prints "99"
            } // the moment this scope ends, the inner x drops and the outer x is accessible
        println!("{}",x);    // Prints "5"
    }
```

You can also shadow a variable in the same scope.

```
fn main(){
        let mut x = 5; // x is mutable
        let x = x; // x is now immutable, this is just changing the mutability of the variable x
    } // the compiler will optimise the above operation while compiling
```

You can also shadow a varibale in different data-type in the same scope.

```
fn main(){
        let meme = "More cowbell!";
        let meme=make_image(meme);
    }
```

## Memory Safety

- You cannot use a variable until you have defined it.
- let's take teh following code

```
fn main(){
                let enigma :i32;
                if true{
                                enigma =42;
                       }
               println!("enigma is {}", enigma);
        }
```

the above code will give us error. conditional evaluation is handled at run time. So the conpiler does not know what is the value of enigma. Hence it will give an error.
However, the following code will run :

```
fn main(){
                let enigma :i32;
                if true{
                                enigma =42;
                       }else{
                                enigma = 7;
                               }
               println!("enigma is {}", enigma);
        }
```

The compiler can guarantee that the variable is guaranteed to be defined before it is called.

## function

- naming convention: underscores

```
fn do_stuff (qty: f64, oz: f64) -> f64 {

  return qty *ox;
}

fn other_studd() - i32{
        1  // if you leave the semi colon at the last of the function, then it is returned
}
```

rust doesn't support variadic arguments (variable number of arguments). But the macros like println!() do.

```
println!("{} {}-oz sarsaparillas(s)!",arg1, arg2);
```

## Module System

let's say that we add a **lib.rs** file in our **src** directory.
src
|**_ lib.rs
|_** main.rs

```
src/lib.rs

pub fn greet(){
  println!("Hi!!!");
}

-----------------
src/main.rs

use hello::greet;

fn main(){
  greet ();
}
```

### third party package

The way to include the third party package is to do the following:

- go to the Cargo.toml
- go to the dependecies section

```
[dependencies]
rand = "0.6.5"  // name = "version"
```

## Scalar types

- Unsigned: u8, u16, u32, u64, u128, usize // usize is the size of the platform pointer type
- Signed: i8, i16, i32, i64, i128, isize

if you are confused about which one to use, go with i32 as it is generally the fastest even on amd64 architecture.
All types are not supported on every platform. Ex a 16 bit micro-controller will only support u8,u16, usize, i8, i16, isize etc.

d = 100_000 is also correct // the underscores are ignored

for float, we have f64 and f32. f64 is default bcoz it has precision but it can be really slow for 32 bit architectures.

```
let x = 5_u16;
let y = 3.14_f32; // type annotation with value
```

Similarly, we have char and bool.
