// Partnered with Jacob Mulligan & Alexa Hong
/*
TASKS
1. Read in Binary File (check)
2. Convert Binary to Instr 
3. Implement Exec loop ( switch dispatch)
4. print last item in stack to stdout
*/
//use std::env;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::slice::Iter;
use std::io::prelude::*;
extern crate byteorder;
use byteorder::{ByteOrder, ReadBytesExt, BigEndian};



type Address = usize;



#[derive(Debug,Clone)]
pub struct State {
    pub halt: bool, //Has the machine halted?
    pub pc: u32, //The current program counter, a 32-bit unsigned integer
    pub fp: u32, //The current frame pointer
    pub stack: Vec<Val>, //The stack, with maximum size STACK_SIZE
    pub heap: Vec<Val>, //The heap
    pub program: Vec<Instr> //The program being executed, a list of instructions
}

#[derive(Debug,Clone,PartialEq)]
pub enum Val {
    //Value types that may appear in GrumpyVM programs:
    Vunit,          //The unit value
    Vi32(i32),      //32-bit signed integers
    Vbool(bool),    //Booleans
    Vloc(u32),      //Stack or instruction locations
    Vundef,         //The undefined value
    
    //Value types that are used internally by the language implementation, and may not appear in GrumpyVM programs:
    Vsize(i32),     //Metadata for heap objects that span multiple values
    Vaddr(Address), //Pointers to heap locations
}

#[derive(Debug,Clone)]
pub enum Instr {
    Push(Val),     //Push(v): Push value v onto the stack
    Pop,           //Pop a value from the stack, discarding it
    Peek(u32),     //Peek(i): Push onto the stack the ith value from the top
    Unary(Unop),   //Unary(u): Apply u to the top value on the stack
    Binary(Binop), //Binary(b): Apply b to the top two values on the stack, replacing them with the result
    Swap,          //Swap the top two values
    Alloc,         //Allocate an array on the heap
    Set,           //Write to a heap-allocated array
    Get,           //Read from a heap-allocated array
    Var(u32),      //Var(i): Get the value at stack position fp+i
    Store(u32),    //Store(i): Store a value at stack position fp+i
    SetFrame(u32), //SetFrame(i): Set fp = s.stack.len() - i
    Call,          //Function call
    Ret,           //Function return
    Branch,        //Conditional jump
    Halt           //Halt the machine
}

#[derive(Debug,Clone)]
pub enum Unop {
    Neg, //Boolean negation
}

#[derive(Debug,Clone)]
pub enum Binop {
    Add, //i32 addition
    Mul, //i32 multiplication
    Sub, //i32 subtraction
    Div, //i32 division (raises an error on divide by zero)
    Lt,  //Returns true if one i32 is less than another, otherwise false
    Eq,  //Returns true if one i32 is equal another, otherwise false
}
//need to reverse this to FromBin

// pub trait FromBin
// {
//     fn from_bin(binary: &mut Iter<u8>)-> Self;
// }


// impl FromBin for u32
// {
//     fn from_bin(binary: &mut Iter<u8>)-> u32

//     {
        
//         for j in binary.iter_mut(){
            
//         }
//        //BigEndian::read_u32(&v)


//     }
    
    
// }
// impl FromBin for Unary{
//     fn from_bin(self) -> Unop{
//         Instr::Unop::Neg
//     }
// }

trait FromBin 
{
    fn from_bin(binary: Iter<u8>) -> Self;
}

// impl FromBin for i32 
// {
//     fn from_bin(binary: Iter<u8>) -> i32
//     {
//         // let my_self: i32 = *binary as i32;
//          //let mut buf: Vec<u8> = [0, 0, 0, 0].to_vec();
//         byteorder::BigEndian::read_i32(&binary)
//        // buf
//     }
// }

// impl FromBin for u32 
// {
//     fn from_bin(binary: &mut Iter<u8>) -> u32
//     {
//         let my_self: u32 = *binary as u32;
//         let mut buf: Vec<u8> = [0, 0, 0, 0].to_vec();
//         BigEndian::read_u32(&mut buf);
//         buf
//     }
// }

impl FromBin for Val 
{
    fn from_bin(mut binary: Iter<u8>) -> Val
    {
        let mut b = binary.next();
        let vu = Some(vec![0b0000_0000]);
        let vbt = Some(vec![0b0000_0010]);
        let vbf = Some(vec![0b0000_0011]); 
        let vdf = Some(vec![0b0000_0101]);

        match b
        {
            vu => Val::Vunit,

            // Vi32(i) => { let mut v = vec![0b0000_0001];
            //     v.append(&mut<i32 as ToBin>::from_bin(i));
            //     v},
          // Vi32(<i32 as FromBin>::from_bin(binary),

            vbt => Val::Vbool(true),
            vbf => Val::Vbool(false),


            // Vloc(i) => {let mut v = vec![0b0000_0100];
            //     v.append(&mut<u32 as ToBin>::from_bin(i));
            //     v},

            vdf => Val::Vundef,


            _       => panic!("ERROR: Vundef"),
        }
    }
}

#[test]
fn test_val_from_bin() {
    let v1 = vec![0];
    let binary1 = v.iter();
    assert_eq!(Val::from_bin(binary1), Val::Vunit);
    // other test cases here
}

// impl FromBin for Unop
// {
//     fn from_bin(binary: &mut Iter<u8>) -> Unop
//     {
//         vec![0b0000_0000]
//     }
// }

// impl FromBin for Binop
// {
//     fn from_bin(binary: &mut Iter<u8>) -> Binop
//     {
//         match binary 
//         {
//             Add => vec![0b0000_0000],
//             Sub => vec![0b0000_0010],
//             Mul => vec![0b0000_0001],
//             Div => vec![0b0000_0011],
//             Lt  => vec![0b0000_0100],
//             Eq  => vec![0b0000_0101],
//         }
//     }
// }

// impl FromBin for Instr
// {
//     fn from_bin(binary: &mut Iter<u8>) -> Instr
//     {
//         match binary 
//         {
//             Push(i)     => { let mut v = vec![0b0000_0000];
//                 v.append(&mut<Val as FromBin>::from_bin(i));
//                 v},
//             Pop         => vec![0b0000_0001],
//             Peek(i)     => { let mut v = vec![0b0000_0000];
//                 v.append(&mut<u32 as FromBin>::from_bin(i));
//                 v},
//             Unary(i)    => { let mut v = vec![0b0000_0000];
//                 v.append(&mut<Unop as FromBin>::from_bin(i));
//                 v},
//             Binary(i)   => { let mut v = vec![0b0000_0000];
//                 v.append(&mut<Binop as FromBin>::from_bin(i));
//                 v},
//             Swap        => vec![0b0000_0101],
//             Alloc       => vec![0b0000_0110],
//             Set         => vec![0b0000_0111],
//             Get         => vec![0b0000_1000],
//             Var(i)      => { let mut v = vec![0b0000_0000];
//                 v.append(&mut<u32 as FromBin>::from_bin(i));
//                 v},
//             Store(i)    => { let mut v = vec![0b0000_0000];
//                 v.append(&mut<u32 as FromBin>::from_bin(i));
//                 v},
//             SetFrame(i) => { let mut v = vec![0b0000_0000];
//                 v.append(&mut<u32 as FromBin>::from_bin(i));
//                 v},
//             Call        => vec![0b0000_1100],
//             Ret         => vec![0b0000_1101],
//             Branch      => vec![0b0000_1110],
//             Halt        => vec![0b0000_1111],
//         }
//     }
// }


fn main() {

    
    //let args: Vec<String> = env::args().collect();

	let mut string = String::new(); // string of what is in the .s file
    let mut file = File::open("multi-arg.o").expect("file did not open");
    //let buf = fs::read(&args[1])?; //if ? doesn't work use unwrap()
   
    let mut binvec: Vec<u8> = Vec::new(); // values from .o file 
    file.read_to_end(&mut binvec).unwrap();
println!("bin vec{:?}", binvec );
    //file.read_to_string(&mut string).unwrap();



    //let prog_len = <Val as FromBin>::from_bin(binvec);
   // println!("bin vec{:?}", binvec );

   //  let mut sizeofvec = binvec[3] as usize;
   // // println!("size: {:?}", sizeofvec );
   //  let mut b = &binvec[0..4];//vector gets the first for bytes for size of 
   //  //println!("{:?}", b);//b is a u8 and needs to be a usize
   
   //  let mut instrvec: Vec<Instr> = Vec::with_capacity(sizeofvec); // new vec to store instructions

   //  let mut vec = instrvec;
   //  //  for i in binvec{
       
   //  // }

   //  //println!("vec_of_chars: {:?}", vec_of_chars);


   //  let mut rest = &binvec[4..]; //gets the rest of vector
   //  let mut five = &binvec[8..12]; //gets the rest of vector
   //  //println!("{:?}", five);
   //  let mut m = 0; 


   // // let prog_len = <u32 as FromBin>::from_bin(five.to_vec());//how to call it

   //  for i in 4..binvec.len() // i is the location in the binary vec, and binvec.len() is the total elements in binvec
   //  {
   //      println!("this is the first value {:?}",binvec[i]);

   //      if m > 0
   //      {
   //          m = m - 1;
   //      }
   //      else{
   //          //println!("{:?}", i);
   //                          println!("this is the first value {:?}",binvec[i]);

   //          match binvec[i]
   //          {

   //              11 =>//setframe
   //              {
   //                  println!("m value{:?}", m);
   //                  // println!(" i value {:?}",i );
   //                  // let set = &binvec[i+1..i+5];
   //                  // println!("{:?}", set);
   //                  // let B = BigEndian::read_u32(&set);
   //                  // println!("{:?}", B);
   //                  // vec.push(Instr::SetFrame(B));
   //                  // println!("inside of vec{:?}", vec);
   //                  //  m = m + 1;


   //              },
   //              15 => //halt
   //              {
                  
   //                  vec.push(Instr::Halt);
   //                  break;
   //                  println!("inside of vec{:?}", vec);
   //                                      m = m + 1;


   //              },
   //              // 0 => {// push
   //              //     match binvec[i]{
   //              //         0 => {//vunit
   //              //             //i = i + 1;
   //              //             m = m + 1;
   //              //            vec.push(&Instr::Push(Val::Vunit));
   //              //         },
   //              //         1 => {//vi32
   //              //            // i = i + 1;
   //              //             m = m + 1;
   //              //             vec.push(&Instr::Push(Val::Vi32(binvec[i] as i32)));
   //              //         },
   //              //         2 => {//vbooltrue

   //              //         },
   //              //         3 =>{//vboolfalse

   //              //         },
   //              //         4 => {//Vloc u32, Label
                            
   //              //         },
   //              //         5 => {//Vundef

   //              //         },
   //              //         _ => panic!("oh no"),    
   //              //     }

   //              _ => panic!("oh no"),  
   //              }

   //          }//end of match1
   //      }//end of else
   //  }
        
    

    
    //println!("this is what im looking for: {:?}\n",prog_len);


}
