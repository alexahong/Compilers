import ply.yacc as yacc 
import sys
import lexer

tokens = lexer.tokens
tok = lexer.tok

def p_binary_operators(p):
     '''expression : expression PLUS term
                   | expression MINUS term
        term       : term TIMES factor
                   | term DIVIDE factor'''
     if p[1] == '+':
         p[0] = p[2] + p[3]
     elif p[1] == '-':
         p[0] = p[2] - p[3]
     elif p[1] == '*':
         p[0] = p[2] * p[3]
     elif p[1] == '/':
         p[0] = p[2] / p[3]

# def p_expression_plus(p):
#     'expression : expression PLUS term'
#     p[0] = p[1] + p[3]

# def p_expression_minus(p):
#     'expression : expression MINUS term'
#     p[0] = p[1] - p[3]

def p_expression_term(p):
    'expression : term'
    p[0] = p[1]

# def p_term_times(p):
#     'term : term TIMES factor'
#     p[0] = p[1] * p[3]

# def p_term_div(p):
#     'term : term DIVIDE factor'
#     p[0] = p[1] / p[3]

def p_term_factor(p):
    'term : factor'
    p[0] = p[2]

def p_factor_num(p):
    'factor : NUMBER'
    p[0] = p[2]

def p_factor_expr(p):
    'factor : LPAREN expression RPAREN'
    p[0] = p[3]

# Error rule for syntax errors
def p_error(p):
    print ("Syntax error in input!")

# Build the parser
yacc.yacc()

# Use this if you want to build the parser using SLR instead of LALR
# yacc.yacc(method="SLR")

while 1:
   try:
       s = input(tok)
   except EOFError:
       break
   if not s: continue
   result = yacc.parse(lexer = s)
   print (result)