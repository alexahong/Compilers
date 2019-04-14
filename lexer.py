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

import ply.lex as lex
import sys
 # List of token names.   This is always required
# class CalcLexer(Lexer): 
tokens = (
'NUMBER',
'PLUS',
'MINUS',
'TIMES',
'DIVIDE',
'LPAREN',
'RPAREN',
'RCOMMENT',
'LCOMMENT',
'TRUE',
'FALSE',
'TT',
'EQUAL',
'LT',
'ASSIGN',
'APP',
'SEP',
'LAM',
'VAR',
'LET',
'SEQ',
'ALLOC',
'SET',
'GET',
'COND',
'FUNPTR',
'CALL',
'F',
'ID',
'ARROW',
'WHITESPACE',
'FUN',
'ARRAY',
'BOOL',
'I32',
'UNIT',
)
 
 # Regular expression rules for simple tokens
t_PLUS    = r'\+'
t_MINUS   = r'\-'
t_TIMES   = r'\*'
t_DIVIDE  = r'\/'
t_LPAREN  = r'\('
t_RPAREN  = r'\)'
#t_RCOMMENT = r'\*/'
#t_LCOMMENT = r'\/*'
t_TRUE = r'\true'
t_FALSE = r'\false'
# t_tt = r'tt'
t_EQUAL = r'\=='
t_LT = r'\<'
t_ASSIGN = r'\='
t_SEP = r'\%'
t_VAR = r'[a-zA-Z_][a-zA-Z0-9_]*'
t_ARROW = r'\->'

reserved = {
    'lam' : 'LAM',
    'cond' : 'COND',
    'let' : 'LET',
    'seq' : 'SEQ',
    'alloc' : 'ALLOC',
    'set' : 'SET',
    'get' : 'GET',
    'funptr' : 'FUNPTR',
    'call' : 'CALL',
    'app' : 'APP',
    'fun' : 'FUN',
    'array' : 'ARRAY',
    'bool' : 'BOOL',
    'i32' : 'I32',
    'unit' : 'UNIT',
}



def t_ID(t):
    r'[a-zA-Z_][a-zA-Z0-9_]*'
    t.type = reserved.get(t.value,'ID')
    return t        

# def t_LAM(t):
#     print 'do lamda stuff'
# def t_Comment(t): 
#     print('stuff')
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
t_ignore_COMMENT = r'\#.*'
 
 # Error handling rule
def t_error(t):
    print("Illegal character '%s'" % t.value[0])
    t.lexer.skip(1)
 
 # Build the lexer
lexer = lex.lex()
 


file = sys.argv[1]
f = open(file, "r")
file_contents = f.read()

lexer = lex.lex()
# Give the lexer some input
lexer.input(file_contents)

for token in lexer:
    print "Token:",token

# # Tokenize
# while True:
#  tok = lexer.token()
#  if not tok: 
#      break      # No more input
#  print(tok)


 
