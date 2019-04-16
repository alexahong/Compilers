import ply.yacc as yacc 
import ply.lex as lex
import sys

reserved = {
'lam' : 'LAM', #i dont think we need this
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
'true' : 'TRUE',#
'false' : 'FALSE',#
'neg' : 'NEG',#
}

tokens = (
'NEG',#
'NUMBER',#
'PLUS',#
'MINUS',#
'TIMES',#
'DIVIDE',#
'LPAREN',#
'RPAREN',#
'LCOMMENT',#
'RCOMMENT',#
'TRUE',#
'FALSE',#
'TT',
'EQUAL',#
'LT',#
'ASSIGN',#
'APP',
'SEP',#
'LAM',# i dont think we need this
'VAR',
'LET',
'SEQ',#
'ALLOC',
'SET',
'GET',
'COND',
'FUNPTR',
'CALL',
'F',
'ID',# not sure if we need this since its alraeady used a rule
'ARROW',
'SPACE',
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
t_LCOMMENT = r'\/\*'
t_RCOMMENT   = r'\*\/'
# t_TT = r'\tt'
t_EQUAL = r'\=='
t_LT = r'\<'
t_ASSIGN = r'\='
t_SEP = r'\%'
t_VAR = r'[a-zA-Z_][a-zA-Z0-9_]*'
t_ARROW = r'\->'
t_SPACE = r'\ '




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
t_ignore_COMMENT = r'\#.*'
 
 # Error handling rule
def t_error(t):
  print("Illegal character '%s'" % t.value[0])
  t.lexer.skip(1)
 

#%
sep = False
#given
print("setframe 0")
print("push Lmain")
print("call")
print("halt")
print("Lmain:")

def parser(lexer, cur_tok, left_paren, right_paren):

  if cur_tok.type == 'SEP':
    left_paren = left_paren + 0
    sep = True

  elif cur_tok.type == 'PLUS':
    token_1 = lexer.token()
    token_2 = lexer.token()
    print("push "+ str(token_1.value) + "\npush " + str(token_2.value) + '\nbinary +')
    return lexer

  elif cur_tok.type == 'MINUS':
    token_1 = lexer.token()
    token_2 = lexer.token()
    print("push "+ str(token_2.value) + "\npush " + str(token_1.value) + '\nbinary -')
    return lexer

  elif cur_tok.type == 'DIVIDE':
    token_1 = lexer.token()
    token_2 = lexer.token()
    print("push "+ str(token_2.value) + "\npush " + str(token_1.value) + '\nbinary /')
    return lexer

  elif cur_tok.type == 'TIMES':
    token_1 = lexer.token()
    token_2 = lexer.token()
    print("push "+ str(token_1.value) + "\npush " + str(token_2.value) + '\nbinary *')
    return lexer

  elif cur_tok.type == 'LPAREN':
    left_paren = left_paren + 1 
    return lexer

  elif cur_tok.type == 'RPAREN':
    right_paren = right_paren + 1
    return lexer

  elif cur_tok.type == 'NUMBER':
    print("push "+ str(cur_tok.value))
    return lexer

  elif cur_tok.type == 'NEG':
    parser(lexer, lexer.token(), left_paren, right_paren)
    print("unary neg")
    return lexer

  elif cur_tok.type == 'SEQ':
    parser(lexer, lexer.token(), left_paren, right_paren)
    parser(lexer, lexer.token(), left_paren, right_paren)
    return lexer

  elif cur_tok.type == 'TRUE':
    print("push true")
    return lexer

  elif cur_tok.type == 'FALSE':
    print("push false")
    return lexer

  elif cur_tok.type == 'LT':
    token_1 = lexer.token()
    token_2 = lexer.token()

    if token_1.value < token_2.value:
      print("push true" + '\nbinary <')
    else:
      print("push false")
    return lexer

  elif cur_tok.type == 'EQUAL':
    token_1 = lexer.token()
    token_2 = lexer.token()

    if token_1.value == token_2.value:
      print( "push true" + '\nbinary ==' )
    else:
      print("push false")
      return lexer

  elif cur_tok.type == 'LCOMMENT':
    comment = 0
    while(True):
      tok = lexer.token()
      if tok.type == 'LCOMMENT':
          comment = comment+ 1
      elif tok.type == 'RCOMMENT' and comment == 1:
          return lexer
      elif tok.type == 'RCOMMENT' and comment != 1:
          comment = comment - 1

  elif cur_tok.type == 'ASSIGN':
    token_1 = lexer.token()
    token_2 = lexer.token()
    token_1 = token_2
    print("push "+ str(token_1.value))
    return lexer
        
  elif cur_tok.type == 'LAM':
    function = lambda fun: cur_tok.type
    print( str(function.value))
    return lexer

  elif cur_tok.type == 'LET':
    #parses next value to check that its a variable
    parser(lexer, lexer.token(), left_paren, right_paren)
    print("push undef")
    return lexer
    #parses next value to get the number
    parser(lexer, lexer.token(), left_paren, right_paren)
    parser(lexer, lexer.token(), left_paren, right_paren)
    print("var " + str(val.value))
    return lexer

  elif cur_tok.type == 'VAR':
    while(True):
      return cur_tok.type
    return lexer
    



#build Lexer
lexer = lex.lex()

#read file
file = sys.argv[1]
f = open(file, "r")
file_contents = f.read()
# Give the lexer some input
lexer.input(file_contents)


while True:
  #reads in first token
  tok = lexer.token()
  if not tok: 
      break 
  #print(tok)
  #tracking the left and right paranthesis, should be zero if they match
  left_paren = 0
  right_paren = 0
  parser(lexer, tok, left_paren, right_paren)

print("ret")



