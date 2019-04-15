import ply.yacc as yacc 
import sys
import lexer


tokens = lexer.tokens
tok = lexer.tok
def parser(lexer, current_token, rp_track, lp_track):

    if tok.type == 'MOD':
        rp_track = rp_track + 0


    elif tok.type == 'PLUS':
        tok1 = lexer.token()
        tok2 = lexer.token()
        print("push "+ str(tok1.value) + "\npush " + str(tok2.value) + '\nbinary +')
        return lexer

    elif tok.type == 'MINUS':
        tok1 = lexer.token()
        tok2 = lexer.token()
        print("push "+ str(tok1.value) + "\npush " + str(tok2.value) + '\nbinary -')
        return lexer

    elif tok.type == 'DIVIDE':
        tok1 = lexer.token()
        tok2 = lexer.token()
        print("push "+ str(tok1.value) + "\npush " + str(tok2.value) + '\nbinary /')
        return lexer

    elif tok.type == 'TIMES':
        tok1 = lexer.token()
        tok2 = lexer.token()
        print("push "+ str(tok1.value) + "\npush " + str(tok2.value) + '\nbinary *')
        return lexer


def p_empty(p):
    '''
    empty :
    '''
    p[0] = None

# Build the parser
parser = yacc.yacc()


# Use this if you want to build the parser using SLR instead of LALR
# yacc.yacc(method="SLR")


while True:
   try:
       s = input(tok)
   except EOFError:
       break
   if not s: continue
   result = parser.parse(s)
   print (result)
