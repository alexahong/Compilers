 # ------------------------------------------------------------
 # calclex.py
 #
 # tokenizer for a simple expression evaluator for
 # numbers and +,-,*,/
 # ------------------------------------------------------------

# Values 
# v ::= n                   //32-bit signed integers
#       true | false        //Boolean values
#       tt                  //The unit value

# Expressions 
# e ::= v                     //Values
#       x                     //Variables
#       (u e)                 //Unary operation u applied to expression e
#       (b e1 e2)             //Binary operation b applied to expressions e1 and e2
#       (let x e1 e2)         //Let x equal the result of e1 in e2 (in which x may appear free)
#       (seq e1 e2)           //Sequential composition (do e1 then e2)
#       (alloc esize einit)   //Allocate an array of size esize, initialized at each index to einit
#       (set earr eidx e)     //Update array earr at index eidx to the value of e
#       (get earr eidx)       //Get the value at index eidx of array earr
#       (cond econd e1 e2)    //If econd evaluates to true then e1, else e2
#       (funptr f)            //A pointer to function f
#       (call e e1 e2 ... eN) //Call function pointer e  
#       (f e1 e2 ... eN)      //Call function f
      
# //Extended expression types:
#       (print e)             //Evaluate e to an i32 then print as ASCII by casting to u8
#       (spawn eclos)         //Spawn a new thread initialized to run eclos (a heap-allocated closure

# Unary Operators
# u ::= neg

# Binary Operators
# b ::= + | * | - | / | < | ==

# Functions 
# param ::= (x ty)                                   //Function parameters, annotated with types
#    fn ::= (fun f param1 param2 ... paramN -> ty e) //Function definitions

# Types 
# ty ::= i32                //32-bit integers
#        bool               //Booleans
#        unit               //The unit type
#        (array ty)         //Arrays of values of type ty

# Programs
# prog ::= fn1 fn2 ... fnM % e

import lex as lex
import yacc as yacc 
 # List of token names.   This is always required
tokens = (
'NUMBER',
'PLUS',
'MINUS',
'TIMES',
'DIVIDE',
'LPAREN',
'RPAREN',
)
 
 # Regular expression rules for simple tokens
t_PLUS    = r'\+'
t_MINUS   = r'-'
t_TIMES   = r'\*'
t_DIVIDE  = r'/'
t_LPAREN  = r'\('
t_RPAREN  = r'\)'
 
 # A regular expression rule with some action code
def t_NUMBER(t):
    r'\d+'
    t.value = int(t.value)    
    return t
 
 # Define a rule so we can track line numbers
def t_newline(t):
    r'\n+'
    t.lexer.lineno += len(t.value)
 
 # A string containing ignored characters (spaces and tabs)
t_ignore  = ' \t'
 
 # Error handling rule
def t_error(t):
    print("Illegal character '%s'" % t.value[0])
    t.lexer.skip(1)
 
 # Build the lexer
lexer = lex.lex()
 

 # Test it out
data = '''
3 + 4 * 10
+ -20 *2
'''

# Give the lexer some input
lexer.input(data)

# Tokenize
while True:
 tok = lexer.token()
 if not tok: 
     break      # No more input
 print(tok)
