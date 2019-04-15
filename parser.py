import ply.yacc as yacc 
import ply.lex as lex
import sys


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


def parser(lexer, current_token, rp_track, lp_track):

  if current_token.type == 'MOD':
    rp_track = rp_track + 0


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




# Build the parser
parser = yacc.yacc()


# Use this if you want to build the parser using SLR instead of LALR
# yacc.yacc(method="SLR")

while True:
  try:
      s = raw_input('calc > ')
  except EOFError:
      break
  if not s: continue
  result = parser.parse(s)
  print(result)
 
