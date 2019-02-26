// Partnered with Jacob Mulligan & Alexa Hong
/*
TASKS
1. Read in Binary File (check)
2. Convert Binary to Instr 
3. Implement Exec loop ( switch dispatch)
4. print last item in stack to stdout
*/
//use std::env;
use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::slice;


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

// fn FromBin()->{

// }

fn main() {

    
    //let args: Vec<String> = env::args().collect();

	let mut string = String::new(); // string of what is in the .s file
    let mut file = File::open("applam.o").expect("file did not open");
    //let buf = fs::read(&args[1])?; //if ? doesn't work use unwrap()
   
    let mut binvec: Vec<u8> = Vec::new(); // values from .o file 
    file.read_to_end(&mut binvec).unwrap();

    //file.read_to_string(&mut string).unwrap();

   // let mut sizeofvec = binvec[3];
    let b = &binvec[0..4];//vector gets the first for bytes for size of 
    println!("{:?}", b[3]);//b is a u8 and needs to be a usize
    
    let mut instrvec: Vec<&Instr> = Vec::capacity(b); // new vec to store instructions

    // println!("{:?}", sizeofvec);
    //  for i in binvec{
       
    // }

    //println!("vec_of_chars: {:?}", vec_of_chars);

    let mut instrvec: Vec<&char> = Vec::new(); // new vec to store instructions

    



    
    println!("{:?}\n",binvec );



}
