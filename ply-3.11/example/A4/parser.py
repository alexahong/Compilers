import ply.yacc as yacc 
import ply.lex as lex
import sys

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
'true' : 'TRUE',
'false' : 'FALSE',

}

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

# t_tt = r'tt'
t_EQUAL = r'\=='
t_LT = r'\<'
t_ASSIGN = r'\='
t_SEP = r'\%'
t_VAR = r'[a-zA-Z_][a-zA-Z0-9_]*'
t_ARROW = r'\->'




def t_ID(t):
  r'[a-zA-Z_][a-zA-Z0-9_]*'
  t.type = reserved.get(t.value,'ID')
  return t        


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
#t_ignore_COMMENT = r'\#.*'
 
 # Error handling rule
def t_error(t):
  print("Illegal character '%s'" % t.value[0])
  t.lexer.skip(1)
 


sep = False

def parser(lexer, current_token, rp_track, lp_track):

  if current_token.type == 'SEP':
    rp_track = rp_track + 0
    sep = True

  elif current_token.type == 'PLUS':
    tok1 = lexer.token()
    tok2 = lexer.token()
    print("push "+ str(tok1.value) + "\npush " + str(tok2.value) + '\nbinary +')
    return lexer

  elif current_token.type == 'MINUS':
    tok1 = lexer.token()
    tok2 = lexer.token()
    print("push "+ str(tok1.value) + "\npush " + str(tok2.value) + '\nbinary -')
    return lexer

  elif current_token.type == 'DIVIDE':
    tok1 = lexer.token()
    tok2 = lexer.token()
    print("push "+ str(tok1.value) + "\npush " + str(tok2.value) + '\nbinary /')
    return lexer

  elif current_token.type == 'TIMES':
    tok1 = lexer.token()
    tok2 = lexer.token()
    print("push "+ str(tok1.value) + "\npush " + str(tok2.value) + '\nbinary *')
    return lexer





lexer = lex.lex()

file = sys.argv[1]
f = open(file, "r")
file_contents = f.read()



# Give the lexer some input
lexer.input(file_contents)




rp_track = 0
lp_track = 0



while True:
  tok = lexer.token()
  if not tok: 
      break 
  #print(tok)
  parser(lexer, tok, rp_track, lp_track)

print("ret")
