// Partnered with Jacob Mulligan & Alexa Hong
/*
TASKS
1. Read in Binary File (check)
2. Convert Binary to Instr 
3. Implement Exec loop ( switch dispatch)
4. print last item in stack to stdout
*/
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::slice::Iter;
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

trait FromBin 
{
    fn from_bin(mut binary:&mut Iter<u8>) -> Self;
}

impl FromBin for i32 
{
    fn from_bin(mut binary:&mut Iter<u8>) -> i32
    {

        let mut buf: Vec<u8> = Vec::new();
        for x in 0..4 
        {
        buf.push(*binary.next().unwrap());
    
        }
        BigEndian::read_i32(&buf)

    }
}

impl FromBin for u32 
{
    fn from_bin(mut binary: &mut Iter<u8>) -> u32
    {
        let mut buf: Vec<u8> = Vec::new();
        for x in 0..4 
        {
        buf.push(*binary.next().unwrap());
      
        }
        BigEndian::read_u32(&buf)



    }
}

#[test]
fn test_u32_from_bin() {
    let v1 = vec![0, 0, 0 ,5];
    let mut b1 = v1.iter();
    assert_eq!(<u32 as FromBin>::from_bin(b1.by_ref()), 5);
    // More test cases like this
}
#[test]
fn test_i32_from_bin() {
    let v1 = vec![0, 0, 0 ,5];
    let mut b1 = v1.iter();
    assert_eq!(<i32 as FromBin>::from_bin(b1.by_ref()), 5);
    // More test cases like this
}

impl FromBin for Val 
{
    fn from_bin(mut binary:&mut Iter<u8>) -> Val
    {

        match binary.next()
        {
            Some(0b0000_0000) => Val::Vunit,
            Some(0b0000_0001) => Val::Vi32(<i32 as FromBin>::from_bin(&mut binary)),
            Some(0b0000_0100) => Val::Vloc(<u32 as FromBin>::from_bin(&mut binary)),
            Some(0b0000_0010) => Val::Vbool(true),
            Some(0b0000_0011) => Val::Vbool(false),
            Some(0b0000_0101) => Val::Vundef,
            _       => panic!("ERROR: Val"),
        }
    }
}

// fn test_val_from_bin() 
// {
//     let v1 = vec![0];
//     let mut buf = v1.iter();
//     assert_eq!(Val::from_bin(&mut buf),Val::Vi32(20));
//     // other test cases here
// }

impl FromBin for Unop
{
    fn from_bin(mut binary: &mut Iter<u8>) -> Unop
    {

       match binary.next()
       {
         Some(0b0000_0000) => Unop::Neg,
         _       => panic!("ERROR: Unop"),

       }
    }
}

impl FromBin for Binop
{
    fn from_bin(mut binary: &mut Iter<u8>) -> Binop
    {
     
        match binary.next()
        {
            Some(0b0000_0000) => Binop::Add,
            Some(0b0000_0010) => Binop::Sub,
            Some(0b0000_0001) => Binop::Mul,
            Some(0b0000_0011) => Binop::Div,
            Some(0b0000_0100) =>  Binop::Lt,
            Some(0b0000_0101) =>  Binop::Eq,
            _   => panic!("ERROR: Binop"),
        }
    }
}

impl FromBin for Instr
{
    fn from_bin(mut binary: &mut Iter<u8>) -> Instr
    {

      
        match binary.next()
        {

            Some(0b0000_0000)  => Instr::Push(<Val as FromBin>::from_bin(&mut binary)),
            Some(0b0000_0001) => Instr::Pop,
            Some(0b0000_0010) => Instr::Peek(<u32 as FromBin>::from_bin(&mut binary)),
            Some(0b0000_0011) => Instr::Unary(<Unop as FromBin>::from_bin(&mut binary)),
            Some(0b0000_0100) => Instr::Binary(<Binop as FromBin>::from_bin(&mut binary)),
            Some(0b0000_0101) => Instr::Swap,
            Some(0b0000_0110) => Instr::Alloc,
            Some(0b0000_0111) => Instr::Set,
            Some(0b0000_1000) => Instr::Get,
            Some(0b0000_1001) => Instr::Var(<u32 as FromBin>::from_bin(&mut binary)),
            Some(0b0000_1010) => Instr::Store(<u32 as FromBin>::from_bin(&mut binary)),
            Some(0b0000_1011) => Instr::SetFrame(<u32 as FromBin>::from_bin(&mut binary)),
            Some(0b0000_1100) => Instr::Call,
            Some(0b0000_1101) => Instr::Ret,
            Some(0b0000_1110) => Instr::Branch,
            Some(0b0000_1111) => Instr::Halt,
             _   => panic!("ERROR: Instr"),
  
        }
    }
}


fn main() {

    
    let args: Vec<_> = env::args().collect();
    let mut file = String::new();
    let file = args[1].to_string();


    let mut binvec: Vec<u8> = fs::read(file).unwrap(); // values from .o file 
    let mut instrvec: Vec<Instr> = Vec::new();// new vec to store instructions
    let mut byte_iterator = binvec.iter();
    let prog_len = <u32 as FromBin>::from_bin(byte_iterator.by_ref()); // first 4 bytes
    
    for i in 0..prog_len
    {

        instrvec.push(Instr::from_bin(byte_iterator.by_ref()));


    }


    for p in instrvec
    {
         println!("{:?}",p );
    }



}
