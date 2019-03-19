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
use std::string::ToString;



type Address = usize;



#[derive(Debug,Clone)]
pub struct State {
    pub halt: bool, //Has the machine halted?
    pub pc: u32, //The current prog counter, a 32-bit unsigned integer
    pub fp: u32, //The current frame pointer
    pub stack: Vec<Val>, //The stack, with maximum size STACK_SIZE
    pub heap: Vec<Val>, //The heap
    pub program: Vec<Instr> //The prog being executed, a list of instructions
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

pub enum Debug {
    DEBUG,
    NODEBUG
}

let MAX_HEAP_SIZE = 1024;

fn main() {

    
    let args: Vec<_> = env::args().collect();
    let mut file = String::new();
    let file = args[1].to_string();


    let mut binvec: Vec<u8> = fs::read(file).unwrap(); // values from .o file 
    let mut prog: Vec<Instr> = Vec::new();// new vec to store instructions
    let mut byte_iterator = binvec.iter();
    let prog_len = <u32 as FromBin>::from_bin(byte_iterator.by_ref()); // first 4 bytes
    
   
    for i in 0..prog_len
    {

     prog.push(Instr::from_bin(byte_iterator.by_ref()));//pushes list of instr
    //println!("{:?}", byte_iterator.by_ref());

    }
    //println!("{:?}", prog);
    //run(&mut init_state,  &prog);// execution loop
     let mut s = State{halt: false, pc:0, fp:0, stack: Vec::new(), heap: Vec::new(), program: prog};
    'mainloop:loop { 
        if s.halt{break 'mainloop}
        let mut pc = s.pc;
        s.pc = pc + 1;
        //println!("{:?}", s.program.len() );
        //println!("{:?}", pc);
        
        if pc >= s.program.len() as u32{
            panic!("pc is out to bounds");
        }
        let i : &Instr = &s.program[pc as usize].clone();
        match i {
            Instr::Push(val) => {
                //pc = pc+1;
                match val{
                    Val::Vunit => {//The unit value
                        s.stack.push(Val::Vunit);
                        //s.pc = s.pc+1;
                    },          
                    Val::Vi32(num) => {
                        s.stack.push(Val::Vi32(*num));      //32-bit signed integers
                        //s.pc = s.pc+1;
                    },
                    Val::Vbool(boolean) =>{ 
                        s.stack.push(Val::Vbool(*boolean));      //Booleans
                        //s.pc = s.pc+1;
                    },
                    Val::Vloc(num)=> {
                        s.stack.push(Val::Vloc(*num));      //Stack or instruction locations
                        //s.pc = s.pc + 1;
                    },
                    Val::Vundef => {
                        s.stack.push(Val::Vundef);          //The unit value
                        //s.pc = s.pc+1;  
                    },
                    Val::Vsize(num) => {
                        s.stack.push(Val::Vsize(*num));
                        //s.pc = s.pc+1;
                    },
                    Val::Vaddr(ad) => {
                        s.stack.push(Val::Vaddr(*ad));
                        //s.pc = s.pc +1;
                    },

                }
                

            },
            Instr::Pop => {
                s.stack.pop().unwrap();
               // s.pc = s.pc +1;
            },
            Instr::Peek(num32) => {
               let v = s.stack[*num32 as usize].clone();
               s.stack.push(v);
            },
            Instr::Unary(U)=> {
                Unop::Neg;
                
            },
            Instr::Binary(Bin) => {
                match Bin {
                    Binop::Add =>{
                        let x = s.stack.pop().unwrap();
                        let y = s.stack.pop().unwrap();
                        let mut x2 : i32 = 0;
                        let mut y2 : i32 = 0;
                        match x{
                            Val::Vi32(num) => x2 =num,
                            _ => panic!("didn't find Vi32"),
                        }
                        match y{
                            Val::Vi32(num) => y2 =num,
                            _ => panic!("didn't find Vi32"),
                        }
                        s.stack.push(Val::Vi32(x2 + y2));
                    },
                    Binop::Sub =>{
                        let x = s.stack.pop().unwrap();
                        let y = s.stack.pop().unwrap();
                        let mut x2 : i32 = 0;
                        let mut y2 : i32 = 0;
                        match x{
                            Val::Vi32(num) => x2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        match y{
                            Val::Vi32(num) => y2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        s.stack.push(Val::Vi32(y2 - x2));
                    },
                    Binop::Mul =>{
                        let x = s.stack.pop().unwrap();
                        let y = s.stack.pop().unwrap();
                        let mut x2 : i32 = 0;
                        let mut y2 : i32 = 0;
                        match x{
                            Val::Vi32(num) => x2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        match y{
                            Val::Vi32(num) => y2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        s.stack.push(Val::Vi32(x2 * y2));
                    },
                    Binop::Div =>{
                        let x = s.stack.pop().unwrap();
                        let y = s.stack.pop().unwrap();
                        let mut x2 : i32 = 0;
                        let mut y2 : i32 = 0;
                        match x{
                            Val::Vi32(num) => x2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        match y{
                            Val::Vi32(num) => y2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        s.stack.push(Val::Vi32(y2 / x2));
                    },
                    Binop::Lt =>{
                        let x = s.stack.pop().unwrap();
                        let y = s.stack.pop().unwrap();
                        let mut x2 : i32 = 0;
                        let mut y2 : i32 = 0;
                        match x{
                            Val::Vi32(num) => x2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        match y{
                            Val::Vi32(num) => y2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        s.stack.push(Val::Vbool(x2 > y2));
                    },
                    Binop::Eq =>{
                        let x = s.stack.pop().unwrap();
                        let y = s.stack.pop().unwrap();
                        let mut x2 : i32 = 0;
                        let mut y2 : i32 = 0;
                        match x{
                            Val::Vi32(num) => x2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        match y{
                            Val::Vi32(num) => y2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        s.stack.push(Val::Vbool(x2 == y2));
                    },
                }
                
            },
            Instr::Swap => {
                let mut v1 = s.stack.pop().unwrap();
                let mut v2 = s.stack.pop().unwrap();
                // let mut tmp = v2;
                // v2 = v1;
                // v1 = tmp;
                s.stack.push(v1);
                s.stack.push(v2);
            },
            Instr::Alloc => {
                
                let mut init = s.stack.pop().unwrap();
                let vsize = s.stack.pop().unwrap();
                let mut size = 0;
                match vsize{
                    Val::Vi32(num) => {
                        size = num;
                    },
                    _ => panic!("alloc panic"),
                }
                s.stack.push(Val::Vaddr(s.heap.len()));
                s.heap.push(Val::Vsize(size));
                if s.heap.len() + size > MAX_HEAP_SIZE{
                    panic!("went over heap size");
                }
                for i in 0..size{
                    //let init_clone = s.stack.pop().unwrap().clone();
                    s.heap.push(init);
                }

            },
            Instr::Set => {
                let add = s.stack.pop().unwrap();
                let ind = s.stack.pop().unwrap();
                let addr = s.stack.pop().unwrap();

                match ind {
                    Val::Vi32(num) =>{
                        match addr {
                            Val::Vaddr(base) => {
                                s.heap.insert((num + (base as i32)+1) as usize, add);
                            },
                            _ => panic!("ohno"),
                        }
                        
                    },
                    _ => panic!("bad"),
                }
            },
            Instr::Get => {
                let mut vid = s.stack.pop().unwrap();
                let mut ind = 0;
                let vbase =s.stack.pop().unwrap();
                let mut base = 0;
                match vbase{
                    Val::Vaddr(num) => {
                        base = num as i32;
                    },
                    _ => panic!("panic in get"),
                }
                match vid{
                    Val::Vi32(num) => {
                        ind = num;
                    },
                    _ => panic!("panic"),
                }
                if let Val::Vsize(array_size) = s.heap[base]{
                    if ind > array_size{
                        panic!("not possible");
                    }
                    let sum = ind + base + 1;
                    let get = s.heap[sum as usize].clone();
                    s.stack.push(get);
                }
                else {
                    panic!("something wrong with the heap");
                }
                if 
                
            },
            Instr::Var(va) => {
                if s.fp + va > s.stack.len() as u32{
                    panic!("out of range");
                }
                else{
                    s.stack.push(s.stack[s.fp + va as usize]);
                }
            },
            Instr::Store(st) => {
                let vnew = st;
                if s.fp + st > s.stack.len() as u32{
                    panic!("out of range");
                }
                else{
                   let v = s.stack.pop().unwrap();
                   s.stack[s.fp + st] = v;
                }
            },
            Instr::SetFrame(vloc) => {
                s.stack.push(Val::Vloc(*vloc));
                s.fp = (s.stack.len() as u32) - vloc -1;
            },
            Instr::Call => {
                let target = s.stack.pop().unwrap();
                //println!("{:?}", target);
                s.stack.push(Val::Vloc(s.pc));
                match target {
                    Val::Vloc(loc) => s.pc = loc,
                    //Val::
                    _ => panic!("oh no, no location"),
                }


            },
            Instr::Ret => {
                let cur_fp = s.fp;
                let vret = s.stack.pop().unwrap();
                //let mut prev_pc = 0;
                match s.stack.pop().unwrap(){
                    Val::Vloc(num) =>{
                        s.pc = num;
                    },
                    _ => panic!("ret failed"),
                }
                let caller_fp = s.stack.pop().unwrap();
                while s.stack.len() > s.fp {
                    s.stack.pop();
                    
                }
                s.stack.push(vret);
            },
            Instr::Branch => {
                let target = s.stack.pop().unwrap();
                let arg = s.stack.pop().unwrap();

                match arg{
                    Val::Vbool(b) =>{
                        if b{
                            match target {
                                Val::Vloc(num) => {
                                    s.pc = num;
                                },
                                _ => panic!("oh no branch"),
                            }
                        }
                    },
                    _ => panic!("ohno"),
                }

            },
            Instr::Halt => s.halt = true,
        }
       // println!("{}, next instr = {:?}", show_state(&s), prog[s.pc]) 
             
    }
    let mut output = s.stack.pop().unwrap();
    //s.stack.read_to_string(&mut output);
    //let output: String = s.stack.into_iter().map(|i| i.to_string()).collect::<String>();
     print!("{:?}", output);

    // for p in prog
    // {
    //      println!("{:?}",p );//prints list of instr
    // }



}
