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

## Compound types

### Tuple

```
let info = (1,3.3,999)
let info : (u8,f64, i32)=(1,3.3,999)
```

If you want to access the members of Tuples, rust let's you to access them using dots, instead of brackets.

```
let info = (1, 3.3, 999);
let jets = info.0;
let fuel= info.1;
let ammo = info.2;
```

The second way of accessing the members of a tuple is all at once.

```
let info = (1,3.3,999);
let (jets,fuel,ammo)=info;
```

**_Arity_** mean how many items are in a tuple. So the following tuple type has an arity of 4.

```
(u8,u8,i32,u64)
```

The maximum arity of tuple in rust is 12.

## Array

arrays are defined as follows:

```
let buf=[1,2,3];

let buf =[0;3]; // [0,0,0] , three times zero

let buf : [u8;3]=[1,2,3]; // len=3, type = u8

buf[0]
```

Arrays are limited to a size of 32, above which they lose most of their functionality. Arrays live on the stack by default and are fized size, so you will usually use vectors or slices of vectors instead of arrays.

## Control Flow

```
if num == 5 {
  msg = "five";
}else if num==4 {
   msg = "four";
}else {
   msg = "thers";
 } // no need for semi-colon here (see the next block, for reference)
```

If we want to give values to the **msg** variable based on if/else condition, we can also do the following :

```
msg = if num ==5 {
    "five" // there is no semi-colon, so that the values are immediately returned from the block
  }else if num ==4 {
      "four" // all the blocks return the same type. Rust is strongly type
    }else{
        "other"
      }; // there is a semi-colon at the end
```

There are no ternary operators in rust.

## loop

loop in Rust is written like this:

```
loop{

    break;
  }
```

You just write **_loop_** and do the operations inside the block and then break. But what happens when you have nested loops like the following:

```
loop {
    loop {
        loop {

          }
      }
  }
```

we give a name to each loop and then we use that name in the break statement. See the following code:

```
'bob: loop{  // naming convention: use ' then write the name
    'tom:loop{
        break 'bob;
      }
  }
```

same goes for continue

```
'bob: loop{  // naming convention: use ' then write the name
    'tom:loop{
        continue 'bob;
      }
  }
```

### iterate over a collection

```
for num in [5,6,7].iter() {
    // do stuff with num
  }
```

```
let array=[(1,2), (3,4)];

for (x,y) in array.iter(){
    // do stuff with x and y
  }




// range

for num in 0..50 { // start is 0 which is included, end is 50 which is excluded

  }

for num in 0..=50 { // start is 0 which is included, end is 50 which is included as well bcoz we have used the equal to sign

  }

```

## String

There are six types of strings in the rust library but we only care about the two which overlap with each other. The first is called a string slice **str**, which we mostly see in the form of a borrowed string slice **_&str_**.
The literal string is always a borrowed string slice. We will talk about borrowing later.
The other string type is a **String** with a capital S.
The differece between the two is that the data in the borrowed string **&str** type can't be modified while the data in the **String** can be modified.

You will often create a string by calling the **to_string()** method on the borrowed string slice (**&str**).

```
let msg = "abc".to_string();
```

or by passing a borrowed string slice to **String::from**. A borrowed string slice is internally made up of a pointer to

A borrowed string slice looks like the following:

```
&str
ptr -> [a,b] // pointer to some bytes
len =2
```

Where as the **String** is made of a pointer , length and a capacity that may be greater than the length. In short, a borrowed string **&str** is a subset of the **String** which is why they share a bunch of characteristics.

Strings can not be indexed by character position. Because english is not the only language in the world. You see, you can index english characters take one byte however characters in different languages take more than one byte. Thai for example take three bytes. So it gets complicated. In languages like hindi or thai, various unicode utf8 bytes combine to make a grapheme (word having "maatra") and it is the grapheme that we care about.

If you really wnat to index, you can use bytes method as follows:

```
words.bytes();
words.chars(); //gives you the iterator
```

At the end, you can use unicode-segmentation package that has various methods to deal with graphemes

## Ownership

Rust ownership model is the reason why people like to chose rust in system programming. Ownership is what makes those crazy safety guarantee possible and makes Rust different from others.
There are 3 rules to ownership:

- Each value has an owner.: Each data has a variable that owns it.
- There is only one owner of the value. Now two variables share the ownership, other variables may borrow the value.
- When the owner get out of scope the value get dropped

let's consider the following code:

```
let s1= Strings::from("abc");
let s2=s1;
println!("{}",s1) // Error!
```

The moment you do s2=s1, the ownership of the value "abc" is transferred from s1 to s2. After that the compiler considers s1 as uninitialized and won't let you use it.
