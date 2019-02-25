setframe 0
push branch
call
halt
Lmain:
push 65
push 200
swap
unary neg
binary +
_Lmain:
ret
branch:
push 5
binary /
ret