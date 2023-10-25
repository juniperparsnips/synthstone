main:
    ldai 1
    ldbi 0

loop:
    add $1
    ldb $1

    add $2
    lda $2

    add $3
    ldb $3

    add $4
    lda $4

    add $5
    ldb $5

    add $6
    lda $6

    jmp loop