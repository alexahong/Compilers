# Compilers

Partners: Alexa Hong and Jacob Mulligan

Alexa: I worked on implementing from_bin and getting the instructions into a vector.
Jacob: I worked on implementing the execution loop to get the correct value output.

our program works by these steps
1. Read in Binary File 
    we accomplished this by reading command line arguments and putting it into a vector of u8s
2. Convert Binary to Instr 
    once the file was read in we took the binary vector and used a trait and implemented for each type to convert to instructions
3. Implement Exec loop ( switch dispatch)
    with the Instructions we then ran our execution loop to match with each instruction and execute its operation
4. print last item in stack to stdout
    print the last element on the stack
