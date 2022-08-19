use std::{io, cmp::Ordering};
//use rand::Rng;

//use std::io::{Write, BufReader, BufRead, ErrorKind};
//use std::fs::File;
//use std::cmp::Ordering;

fn main() {
    // println!("What is your name?");
    // let mut name=String::new();
    // let greeting ="Nice to meet you";
    //
    // io::stdin().
    //     read_line(&mut name)
    //     .expect("Didn't recieve a valid input");
    //
    // println!("Hello, {}!, {greeting}",name.trim_end());

    // const ONE_MIL: u32=1_000_000;
    // const Pi:f32=3.141592;
    // let age:&str="47";
    // let mut age:u32=age.trim().parse()
    //     .expect("Age wasn't assigned a number");
    //
    // age =age+1;

    // println!("Max u32: {}", u32::MAX) // max value of a data type

    // let is_true: bool=true;

    // let my_grage:char='a';

    // let age=8;
    //
    // if (age>=1) && age<=18{
    //     println!("Important Birthday");
    // }else if (age ==21)|| (age==50){
    //     println!("Important Birthday")
    // }

    // let mut age = 47;
    //
    // let can_vote: bool = if (age >= 18) { true } else { false };
    //
    // let age2 =8;
    //
    // match age2{
    //     1..=18=> println!("Important Birthday"),
    //     _=> println!("Not Important Birthday"), // _ is for default
    // };


    // let mut my_age:i32=18;
    // let voting_age:i32=18;
    //
    // match my_age.cmp(&voting_age){
    //     Ordering::Less => println!("you dont have the right to vote"),
    //     Ordering:: Greater => println!("Can vote"),
    //     Ordering::Equal=> println!("You got the right to vote"),
    // };
    //
    
     // let arr_1:[i32;4]=[1,2,3,4];
    //
    // println!("1st: {}",arr_1[0]);
    //
    //
   //  let mut loop_idx=0;
    //
    // loop{
    //     if arr_1[loop_idx]%2==0{
    //         loop_idx+=1;
    //         continue;
    //     }
    //     if arr_1[loop_idx]==9{
    //         break;
    //     }
    // }
    //
    //
    // while loop_idx<arr_1.len(){
    //     println!("Array element is {}",arr_1[loop_idx])
    // }
    //
    // 
    // for val in arr_1.iter(){
    //     println!("val: {val}")
    // }
    
  // let my_tuple:(u8,String , f32)=(47,"Derek".to_string(),50_000.00);
  //
  // let (v1,v2,v3)=my_tuple;

 //  let mut st1=String::new();
 //
 //  st1.push('A');
 //  st1.push('B');
 //  st1.push_str("word");
 //
 //
 //  for word in st1.split_whitespace(){
 //      println!("{}",word);
 //  }
 //
 //  let st2=st1.replace("A","Another");
 //
 //  println!("{}",st2);
 //
 //  let st3=String::from("x r t b h k k a m c");
 //
 //  let mut v1: Vec<char>= st3.chars().collect();
 //
 //  v1.sort();
 //  v1.dedup(); //removes duplicates
 //
 //  for char in v1{
 //      println!("{}",char);
 //  }
 //
 //  let st4: &str="Random Strinbg";
 //  let mut st5: String=st4.to_string();
 // 
 //  let st6=String::from("Just some");
 //  let st7 = String::from(" words");
 // 
 //  let st8=st6 + &st7; // if we don't use the & sign, the the variable st6 and st7 gets deleted snd
 //                       // their value is stored in the st8
 //
 //  for char in st8.bytes(){
 //      println!("{}",char);
 //  }
 //     
 //  let int_u8:u8=5;
 //
 //  let int32_u8:u8=4;
 //
 //  let int3_32:u32=(int_u8 as u32);
 //
 // enum Day{
 //     Monday,
 //     Tuesdat,
 //     Wednesday,
 //     Thursday,
 //     Friday,
 //     Saturday,
 //     Sunday
 // }
 //
 // impl  Day{
 //     fn is_weekend(&self)->bool{
 //         match self{
 //             Day::Saturday | Day::Sunday =>true,
 //             _=> false,
 //         }
 //     }
 // }
 //
 // let today:Day=Day::Monday;
 //
 // println!("Is today the weekend {}", today.is_weekend())
 //
 //
 //
 

    let mut vec1:Vec<i32>=Vec::new();

    vec1.push(5);

    let second: &i32 = &vec1[1];

    match vec1.get(1){
        Some(second)=>println!("2nd: {}",second),
        None=>println!("No 2nd value"),
    }


    for i in &mut vec2{

    }


}
