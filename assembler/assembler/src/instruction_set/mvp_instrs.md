R Types: opcode, rs, rt
I Types: opcode, imm
J Types: opcode, label
B Types: opcode, 0000_0000

Label can be a text label i.e. "main" or a hex PC address 

Instr   Opcode  R/I Behavior    

NOP     0000 0  B   nothing
LDA     0001 1  R   load *A* from *$rs*
LDB     0010 2  R   load *B* from *$rs*
LDAI    0011 3  I   load *A* from *imm*
LDBI    0100 4  I   load *B* from *imm*
ADD     0101 5  R   store *A + B* to *$rs*
SUB     0110 6  R   store *A - B* to *$rs*
INV     0111 7  R   store *~A* to *$rs*
????    1000 8  ?   ?
????    1001 9  ?   ?
????    1010 A  ?   ?
????    1011 B  ?   ?
JMP     1100 C  J   set the PC to *imm*
BEQ     1101 D  J   if *A == B* set PC to *imm*   
BNE     1110 E  J   if *A != B* set PC to *imm*
HLT     1111 F  B   stop the clock


Simple parsing rules:
labels are:
"<label>:"
and must be at the beginning of the line and followed by a ":"

lines must be
label:
instruction reg/imm


TODO:
label parsing is currently just line# - 1
fails with a empty line