main:
    nop
    add $3
    jmp main
    beq end

end:
    hlt