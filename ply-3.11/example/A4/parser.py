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
#t_ignore_COMMENT = r'\#.*'
 
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

def parser(lexer, current_token, lp_track, rp_track):

  if current_token.type == 'SEP':
    lp_track = lp_track + 0
    sep = True

  elif current_token.type == 'PLUS':
    tok1 = lexer.token()
    tok2 = lexer.token()
    print("push "+ str(tok1.value) + "\npush " + str(tok2.value) + '\nbinary +')
    return lexer

  elif current_token.type == 'MINUS':
    tok1 = lexer.token()
    tok2 = lexer.token()
    print("push "+ str(tok2.value) + "\npush " + str(tok1.value) + '\nbinary -')
    return lexer

  elif current_token.type == 'DIVIDE':
    tok1 = lexer.token()
    tok2 = lexer.token()
    print("push "+ str(tok2.value) + "\npush " + str(tok1.value) + '\nbinary /')
    return lexer

  elif current_token.type == 'TIMES':
    tok1 = lexer.token()
    tok2 = lexer.token()
    print("push "+ str(tok1.value) + "\npush " + str(tok2.value) + '\nbinary *')
    return lexer

  elif current_token.type == 'LPAREN':
    lp_track = lp_track + 1 
    return lexer

  elif current_token.type == 'RPAREN':
    rp_track = rp_track + 1
    return lexer

  elif current_token.type == 'NUMBER':
    print("push "+ str(current_token.value))
    return lexer

  elif current_token.type == 'NEG':
    parser(lexer, lexer.token(), lp_track, rp_track)
    print("unary neg")
    return lexer

  elif current_token.type == 'SEQ':
    parser(lexer, lexer.token(), lp_track, rp_track)
    parser(lexer, lexer.token(), lp_track, rp_track)
    return lexer

  elif current_token.type == 'TRUE':
    print("push true")
    return lexer

  elif current_token.type == 'FALSE':
    print("push false")
    return lexer

  elif current_token.type == 'LT':
    tok1 = lexer.token()
    tok2 = lexer.token()

    if tok1.value < tok2.value:
      print("push true" + '\nbinary <')
    else:
      print("push false")
    return lexer

  elif current_token.type == 'EQUAL':
    tok1 = lexer.token()
    tok2 = lexer.token()

    if tok1.value == tok2.value:
      print( "push true" + '\nbinary ==' )
    else:
      print("push false")
      return lexer

  elif current_token.type == 'LCOMMENT':
    comment = 0
    while(True):
      tok = lexer.token()
      if tok.type == 'LCOMMENT':
          comment = comment+ 1
      elif tok.type == 'RCOMMENT' and comment == 1:
          return lexer
      elif tok.type == 'RCOMMENT' and comment != 1:
          comment = comment - 1

  elif current_token.type == 'ASSIGN':
    tok1 = lexer.token()
    tok2 = lexer.token()
    tok1 = tok2
    print("push "+ str(tok1.value))
    return lexer
        
  elif current_token.type == 'LAM':
    function = lambda fun: current_token.type
    print( str(function.value))
    return lexer

  elif current_token.type == 'LET':
    #parses next value to check that its a variable
    parser(lexer, lexer.token(), lp_track, rp_track)
    print("push undef")
    return lexer
    #parses next value to get the number
    parser(lexer, lexer.token(), lp_track, rp_track)
    parser(lexer, lexer.token(), lp_track, rp_track)
    print("var " + str(val.value))
    #return lexer

    



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
  rp_track = 0
  lp_track = 0
  parser(lexer, tok, rp_track, lp_track)
  #checking to make sure that parenthesis match, shoud be zero
  if lp_track != rp_track:
    raise Exception("parenthesis don't match")
print("ret")



