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
use std::collections::HashMap;



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
const MAX_HEAP_SIZE : i32 = 1024;

pub fn instr( i: &Instr, s: &mut State)
{
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
                        match y{
                            Val::Vi32(num) => y2 =num,
                            _ => panic!("didn't find Vi32"),
                        }
                        match x{
                            Val::Vi32(num) => x2 =num,
                            _ => panic!("didn't find Vi32"),
                        }
                        s.stack.push(Val::Vi32(x2 + y2));
                    },
                    Binop::Sub =>{
                        let x = s.stack.pop().unwrap();
                        let y = s.stack.pop().unwrap();
                        let mut x2 : i32 = 0;
                        let mut y2 : i32 = 0;
                        match y{
                            Val::Vi32(num) => y2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        match x{
                            Val::Vi32(num) => x2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        s.stack.push(Val::Vi32(x2 - y2));
                    },
                    Binop::Mul =>{
                        let x = s.stack.pop().unwrap();
                        let y = s.stack.pop().unwrap();
                        let mut x2 : i32 = 0;
                        let mut y2 : i32 = 0;
                        match y{
                            Val::Vi32(num) => y2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        match x{
                            Val::Vi32(num) => x2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        s.stack.push(Val::Vi32(x2 * y2));
                    },
                    Binop::Div =>{
                        let x = s.stack.pop().unwrap();
                        let y = s.stack.pop().unwrap();
                        let mut x2 : i32 = 0;
                        let mut y2 : i32 = 0;
                        match y{
                            Val::Vi32(num) => y2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        match x{
                            Val::Vi32(num) => x2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        s.stack.push(Val::Vi32(x2 / y2));
                    },
                    Binop::Lt =>{
                        let x = s.stack.pop().unwrap();
                        let y = s.stack.pop().unwrap();
                        let mut x2 : i32 = 0;
                        let mut y2 : i32 = 0;
                        match y{
                            Val::Vi32(num) => y2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        match x{
                            Val::Vi32(num) => x2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        s.stack.push(Val::Vbool(x2 < y2));
                    },
                    Binop::Eq =>{
                        let x = s.stack.pop().unwrap();
                        let y = s.stack.pop().unwrap();
                        let mut x2 : i32 = 0;
                        let mut y2 : i32 = 0;
                        match y{
                            Val::Vi32(num) => y2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        match x{
                            Val::Vi32(num) => x2 =num,
                            _ => panic!("didn't find Vi32 sub"),
                        }
                        s.stack.push(Val::Vbool(x2 == y2));
                    },
                }
                
            },
            Instr::Swap => {
                let mut v1 = s.stack.pop().unwrap();
                let mut v2 = s.stack.pop().unwrap();
              
                s.stack.push(v1);
                s.stack.push(v2);
            },

            Instr::Alloc => {
            let init = s.stack.pop().unwrap(); //vinit
            let vsize = s.stack.pop().unwrap(); //Vi32(size)
            match vsize {
                Val::Vi32(num) =>{
                    
                    if num as usize + s.heap.len() as usize > MAX_HEAP_SIZE as usize
                    {
                        garbage_collector(s);
                       
                    }
                    
                    s.stack.push(Val::Vaddr(s.heap.len() as usize));
                    s.heap.push(Val::Vsize(num));
                    for x in 0..num {
                    	let vinit = init.clone();
                        s.heap.push(vinit);
                    }
                }
                _    => panic!("Alloc Panic"),
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
                if let Val::Vsize(array_size) = s.heap[base as usize]{
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
                
                
            },
            Instr::Var(va) => {
                let n = va;
                if s.fp + n > s.stack.len() as u32{
                    panic!("out of range var");
                }
                else{
                    let n2 = s.stack[(s.fp + n) as usize].clone();
                     s.stack.push(n2);
                }
            },
            Instr::Store(st) => {
                let mut vnew = st;
                if s.fp + vnew > s.stack.len() as u32{
                    panic!("out of range store");
                }
                else{
                   let v = s.stack.pop().unwrap();
                 s.stack[(s.fp as usize) + *vnew as usize] = v;
                }
            },
            Instr::SetFrame(vloc) => {
                s.stack.push(Val::Vloc(*vloc));
                s.fp = (s.stack.len() as u32) - vloc - 1;
            },
            Instr::Call => {
                let target = s.stack.pop().unwrap();
                match target {
                    Val::Vloc(loc) =>
                    {
                    s.stack.push(Val::Vloc(s.pc));

                     s.pc = loc;
                    }
                    _ => panic!("oh no, no location"),
                }


            },
            Instr::Ret => {

              
                let vret = s.stack.pop().unwrap();
                let caller_pc = s.stack.pop().unwrap();
                let caller_fp = s.stack.pop().unwrap();
  				let cur_fp = s.fp;
                let cur_pc = s.pc;

                match caller_pc{

                    Val::Vloc(num) => s.pc = num,
                    Val::Vi32(num2) => s.pc = num2 as u32,
                 
                    _=> panic!("incorrect Vals caller_pc"),
                       
                    
                }

    			match caller_fp{

                    Val::Vloc(num) => s.fp = num,
                    Val::Vi32(num2) =>  s.fp = num2 as u32,
                     
                    _=> panic!("incorrect Vals caller_fp"),
                       
                    
                }

                while s.stack.len() > cur_fp as usize {

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

}

// Pass 1: Search the Stack
// - For each value = Vaddr(base) entry in the stack
//     + If from_heap[base] = Vsize(size)
//         * set value = Vaddr(to_heap.length)
//         * Copy the array at from_heap[base] into the to_heap
//         * next += size
//     + Else panic
 
// Pass 2: Search the To Heap
// - While scan < next
//     + For each value = Vaddr(base) entry in the to_heap
//         - Repeat the steps in the above for loop, just using the to_heap instead of the stack
//         - Every time you encounter a `Vsize`, set scan to the current index

pub fn garbage_collector(s: &mut State){
    let mut next = 0;
    let mut scan = 0;
    let mut to_heap: Vec<Val>  = Vec::with_capacity(MAX_HEAP_SIZE as usize); 
    let mut address: HashMap<usize, usize> = HashMap::new();
 
//first pass

    for num in 0..s.stack.len()
    {   // each entry in the stack
    	                           

        if let Val::Vaddr(base) = s.stack[num] 
        {  //vaddr(base) in stack  
           
            if let Some(&number) = address.get(&base)
        {
            to_heap[scan as usize] = Val::Vaddr(number);
        }else
        {
            if let Val::Vsize(mut size) = &s.heap[base]{
                for val in base..(base+(size as usize))
                {
                    to_heap.push(s.heap[val].clone());
                }
                next += size as usize;
                address.insert(base, scan); //add adress into hash that has been copied
            }
        } 
      	}
    }


 //second pass
    while scan < next{                                     
    if let Val::Vaddr(base) = to_heap[scan as usize]
    {          
        if let Some(&number) = address.get(&base)
        {
            to_heap[scan as usize] = Val::Vaddr(number);
        }else
        {
            if let Val::Vsize(mut size) = &s.heap[base]{
                for val in base..(base+(size as usize))
                {
                    to_heap.push(s.heap[val].clone());
                }
                next += size as usize;
                address.insert(base, scan);
            }
        } 
    }
    scan += 1;
    }
    s.heap = to_heap; // switch
}

fn main() {

    
    let args: Vec<_> = env::args().collect();
    let mut file = String::new();
    let file = args[1].to_string();


    let mut binvec: Vec<u8> = fs::read(file).unwrap(); // values from .o file 
    let mut prog: Vec<Instr> = Vec::new();// new vec to store instructions
    let mut byte_iterator = binvec.iter();
    let prog_len = <u32 as FromBin>::from_bin(byte_iterator.by_ref()); // first 4 bytes


    
        let mut s = State{halt: false, pc:0, fp:0, stack: Vec::new(), heap: Vec::new(), program: prog};

    for i in 0..prog_len
    {

     s.program.push(Instr::from_bin(byte_iterator.by_ref()));//pushes list of instr

    }

    

    
    'mainloop:loop { 
        if s.halt{break 'mainloop}
        let mut pc = s.pc;
        s.pc = pc + 1;

        
        if (pc as usize) >= s.program.len(){
            panic!("pc is out to bounds");
        }
        let i = &s.program[pc as usize].clone();
        instr(i, &mut s);
       } //end of mainloop
		           

    let mut output = s.stack.pop().unwrap();
     print!("{:?}", output);


}
