; Andy Min (akmin2), 2023-01-29
;
; Evaluates reverse polish notation calculations inputed from the user. Handles addition,
; subtraction, multiplication, division, and exponents with single-digit positive integers.

            .ORIG   x3000

; Main program loop.
MAIN_LOOP
            GETC                        ; read user input
            OUT                         ; echo input back to console
            JSR     EVAL                ; evaluate input
            BRnzp   MAIN_LOOP           ; loop back to start


; Evaluates an input and runs calculations.
;
; IN:  R0 (user input)
;
; Register table:
; R0: The user input (sometimes negated for ease of checking values). Also used for printing the
;     invalid message.
; R1: Various ASCII variables used when checking R0. Also the STACK_START pointer when calculating
;     stack size.
; R2: STACK_TOP pointer when calculating stack size.
; R5: Return values from PUSH and POP (0-success)
;
EVAL
            ST      R7,EVAL_R7          ; save R7

            NOT     R0,R0               ; negate R0
            ADD     R0,R0,#1

            ; if input is an equal sign, jump to EVAL_STOP

            LD      R1,EQUAL_CHAR       ; load '=' into R1
            ADD     R1,R1,R0            ; add R0 to R1
            BRz     EVAL_STOP

            ; if input is a space, jump to EVAL_RET

            LD      R1,SPACE_CHAR       ; load ' ' into R1
            ADD     R1,R1,R0            ; add R0 to R1
            BRz     EVAL_RET

            ; if input is a number, push value to stack

            LD      R1,ZERO_CHAR        ; load '0' into R1
            ADD     R1,R1,R0            ; add R0 to R1
            BRp     EVAL_NOT_NUMBER     ; if input is < '0', input is not a number
            LD      R1,NINE_CHAR        ; load '9' into R1
            ADD     R1,R1,R0            ; add R0 to R1
            BRn     EVAL_NOT_NUMBER     ; if input is > '9', input is not a number

            LD      R1,ZERO_CHAR        ; load '0' into R1
            ADD     R0,R0,R1            ; add R1 to R0
            NOT     R0,R0               ; negate R0 (back to positive)
            ADD     R0,R0,#1

            JSR     PUSH                ; push value onto stack

            ADD     R5,R5,#0            ; PUSH failed - invalid input
            BRp     EVAL_INVALID

            BRnzp   EVAL_RET

EVAL_NOT_NUMBER

            ; addition

            LD      R1,PLUS_CHAR        ; load '+' into R1
            ADD     R1,R1,R0            ; add R0 to R1
            BRz     EVAL_PLUS

            ; subtraction

            LD      R1,MIN_CHAR         ; load '-' into R1
            ADD     R1,R1,R0            ; add R0 to R1
            BRz     EVAL_MIN

            ; multiplication

            LD      R1,MUL_CHAR         ; load '*' into R1
            ADD     R1,R1,R0            ; add R0 to R1
            BRz     EVAL_MUL

            ; division

            LD      R1,DIV_CHAR         ; load '/' into R1
            ADD     R1,R1,R0            ; add R0 to R1
            BRz     EVAL_DIV

            ; exponent

            LD      R1,EXP_CHAR         ; load '^' into R1
            ADD     R1,R1,R0            ; add R0 to R1
            BRz     EVAL_EXP

            ; invalid input character

            BRnzp   EVAL_INVALID

EVAL_PLUS
            JSR     GET_OPS             ; get operands
            ADD     R5,R5,#0            ; check POP status
            BRp     EVAL_INVALID
            JSR     PLUS                ; compute sum
            JSR     PUSH                ; push result
            BRnzp   EVAL_RET

EVAL_MIN
            JSR     GET_OPS             ; get operands
            ADD     R5,R5,#0            ; check POP status
            BRp     EVAL_INVALID
            JSR     MIN                 ; compute difference
            JSR     PUSH                ; push result
            BRnzp   EVAL_RET

EVAL_MUL
            JSR     GET_OPS             ; get operands
            ADD     R5,R5,#0            ; check POP status
            BRp     EVAL_INVALID
            JSR     MUL                 ; compute product
            JSR     PUSH                ; push result
            BRnzp   EVAL_RET

EVAL_DIV
            JSR     GET_OPS             ; get operands
            ADD     R5,R5,#0            ; check POP status
            BRp     EVAL_INVALID
            JSR     DIV                 ; compute quotient
            JSR     PUSH                ; push result
            BRnzp   EVAL_RET

EVAL_EXP
            JSR     GET_OPS             ; get operands
            ADD     R5,R5,#0            ; check POP status
            BRp     EVAL_INVALID
            JSR     EXP                 ; computer quotient
            JSR     PUSH                ; push result
            BRnzp   EVAL_RET

EVAL_RET
            LD      R7,EVAL_R7          ; restore EVAL_R7
            RET

EVAL_STOP
            LD      R1,STACK_START      ; set R1 to STACK_START
            LD      R2,STACK_TOP        ; set R2 to STACK_TOP

            NOT     R1,R1               ; negate R1
            ADD     R1,R1,#1

            ADD     R1,R1,R2            ; add R2 to R1

            ADD     R1,R1,#1            ; if R1 is not 1, invalid
            BRnp    EVAL_INVALID

            JSR     POP                 ; pop result
            ADD     R5,R0,#0            ; set R5 to result
            JSR     PRINT_HEX           ; print result

            HALT

EVAL_INVALID
            LEA     R0,INVALID_MSG      ; print invalid message
            PUTS

            HALT

EVAL_R7     .BLKW   #1                  ; R7 saved value


; Push a value into the stack.
;
; IN:  R0 (value to push)
; OUT: R5 (0-success, 1-fail/overflow)
;
; Register table:
; R3: stack end pointer
; R4: stack top pointer
;
PUSH
            ST      R3,PUSH_R3          ; save R3
            ST      R4,PUSH_R4          ; save R4
            LD      R3,STACK_END        ; set R3 to stack end pointer
            LD      R4,STACK_TOP        ; set R4 to stack top pointer

            AND     R5,R5,#0            ; clear R5

            ADD     R3,R3,#-1           ; subtract 1 from stack end pointer
            NOT     R3,R3               ; negate R3 (stack end pointer)
            ADD     R3,R3,#1
            ADD     R3,R3,R4            ; add R3 to R4
            BRz     PUSH_OVERFLOW       ; stack is full - overflow

            STR     R0,R4,#0            ; no overflow, store value in the stack
            ADD     R4,R4,#-1           ; move stack top pointer
            ST      R4,STACK_TOP        ; store stack top pointer
            BRnzp   PUSH_RET
PUSH_OVERFLOW
            ADD     R5,R5,#1
PUSH_RET
            LD      R3,PUSH_R3          ; restore R3
            LD      R4,PUSH_R4          ; restore R4
            RET

PUSH_R3     .BLKW   #1                  ; R3 saved value
PUSH_R4     .BLKW   #1                  ; R4 saved value


; Pop a value from the stack.
;
; OUT: R0 (value from stack)
; OUT: R5 (0-success, 1-fail/underflow)
;
; Register table:
; R3: stack start pointer
; R4: stack top pointer
;
POP
            ST      R3,POP_R3           ; save R3
            ST      R4,POP_R4           ; save R3
            LD      R3,STACK_START      ; set R3 to stack start pointer
            LD      R4,STACK_TOP        ; set R4 to stack top pointer

            AND     R5,R5,#0            ; clear R5

            NOT     R3,R3               ; negate R3 (stack start pointer)
            ADD     R3,R3,#1
            ADD     R3,R3,R4            ; add R3 to R4
            BRz     POP_UNDERFLOW       ; stack is empty - underflow

            ADD     R4,R4,#1            ; move stack top pointer
            LDR     R0,R4,#0            ; load value into stack
            ST      R4,STACK_TOP        ; store stack top pointer
            BRnzp   POP_RET
POP_UNDERFLOW
            ADD     R5,R5,#1
POP_RET
            LD      R3,POP_R3           ; restore R3
            LD      R4,POP_R4           ; restore R4
            RET

POP_R3      .BLKW   #1                  ; R3 saved value
POP_R4      .BLKW   #1                  ; R4 saved value


; Pop the two operands from the stack.
;
; OUT: R3 (operand #1)
; OUT: R4 (operand #2)
; OUT: R5 (0-success, 1-fail/underflow)
GET_OPS
            ST      R7,GET_OPS_R7       ; save R7

            JSR     POP
            ADD     R4,R0,#0            ; copy value to R4
            ADD     R5,R5,#0            ; POP failed - invalid input
            BRp     GET_OPS_RET

            JSR     POP
            ADD     R3,R0,#0            ; copy value to R3

GET_OPS_RET
            LD      R7,GET_OPS_R7       ; restore GET_OPS_R7
            RET

GET_OPS_R7  .BLKW   #1                  ; R7 saved value


; Print the hexadecimal representation of a register.
;
; IN:  R5 (register value to print)
;
; Register table:
; R1: outer loop counter
; R2: inner loop counter
; R3: current group of 4-bits
PRINT_HEX
            ST      R0,P_HEX_R0         ; save R0
            ST      R1,P_HEX_R1         ; save R1
            ST      R2,P_HEX_R2         ; save R2
            ST      R3,P_HEX_R3         ; save R3
            ST      R5,P_HEX_R5         ; save R5
            ST      R7,P_HEX_R7         ; save R7

            ; set up and outer loop (loops through groups of 4 within the 16-bit data in R5) and
            ; and inner loop (loops through each of the 4 bits within the group)

            AND     R1,R1,#0            ; set outer loop counter (R1) to 4
            ADD     R1,R1,#4

P_HEX_LOOP1
            AND     R2,R2,#0            ; set inner loop counter (R2) to 4
            ADD     R2,R2,#4

            AND     R3,R3,#0            ; set R3 (the current group of 4-bits) to 0

P_HEX_LOOP2
            ADD     R3,R3,R3            ; left shift R3 by doubling it

            ADD     R5,R5,#0            ; set nzp flags with the register value (R5)

            BRzp    P_HEX_SHIFT         ; if MSB is 0, skip to shifting R5
            ADD     R3,R3,#1            ; otherwise, add 1 to R3

P_HEX_SHIFT
            ADD     R5,R5,R5            ; left shift R5

            ADD     R2,R2,#-1           ; decrement inner loop counter (R2)
            BRp     P_HEX_LOOP2         ; loop back to the start of the inner loop

            ; print the current group of 4-bits (R3) by checking if its less than 10 (print 0-9) or
            ; greater than or equal to 10 (print A-F)

            ADD     R2,R3,#-10          ; set R2 to R3 - 10

            BRzp    P_HEX_ALPHA         ; if R2 is >= 0, then that means R3 is A-F (alpha character)

            LD      R0,ZERO_CHAR        ; otherwise, R3 is 0-9 (numerical character)
            ADD     R0,R0,R3            ; set R0 to R3 + the offset for '0' in ASCII
            BRnzp   P_HEX_PRINT         ; jump to printing

P_HEX_ALPHA
            LD      R0,A_CHAR           ; R3 is A-F (alpha character)
            ADD     R0,R0,R2            ; set R0 to R3 - 10 + the offset for 'A' in ASCII

P_HEX_PRINT
            OUT                         ; print

            ADD     R1,R1,#-1           ; decrement outer loop counter (R1)
            BRp     P_HEX_LOOP1         ; loop back to the start of the outer loop

            LD      R0,P_HEX_R0         ; restore P_HEX_R0
            LD      R1,P_HEX_R1         ; restore P_HEX_R1
            LD      R2,P_HEX_R2         ; restore P_HEX_R2
            LD      R3,P_HEX_R3         ; restore P_HEX_R3
            LD      R5,P_HEX_R5         ; restore P_HEX_R5
            LD      R7,P_HEX_R7         ; restore P_HEX_R7

            RET

P_HEX_R0    .BLKW   #1                  ; R0 saved value
P_HEX_R1    .BLKW   #1                  ; R1 saved value
P_HEX_R2    .BLKW   #1                  ; R2 saved value
P_HEX_R3    .BLKW   #1                  ; R3 saved value
P_HEX_R5    .BLKW   #1                  ; R5 saved value
P_HEX_R7    .BLKW   #1                  ; R7 saved value


; Add two values.
;
; IN:  R3, R4 (values)
; OUT: R0 (sum)
PLUS
            ADD     R0,R3,R4
            RET


; Subtract two values.
;
; IN:  R3, R4 (values)
; OUT: R0 (difference)
MIN
            NOT     R0,R4               ; negate R4 and store in R0
            ADD     R0,R0,#1
            ADD     R0,R3,R0            ; add R3
            RET


; Multiply two values.
;
; IN:  R3, R4 (values)
; OUT: R0 (product)
MUL
            ST      R3,MUL_R3           ; save R3
            ST      R4,MUL_R4           ; save R4

            AND     R0,R0,#0            ; reset R0

            ADD     R3,R3,#0
            BRzp    MUL_LOOP
            NOT     R3,R3
            ADD     R3,R3,#1
            NOT     R4,R4
            ADD     R4,R4,#1
MUL_LOOP
            ADD     R0,R0,R4            ; add R4 to product
            ADD     R3,R3,#-1           ; decrement R3
            BRp     MUL_LOOP

            LD      R3,MUL_R3           ; restore R3
            LD      R4,MUL_R4           ; restore R4

            RET

MUL_R3      .BLKW   #1                  ; R3 saved value
MUL_R4      .BLKW   #1                  ; R4 saved value


; Divide two values (ignores remainder).
;
; IN:  R3, R4 (values)
; OUT: R0 (quotient)
DIV
            ST      R3,DIV_R3           ; save R3
            ST      R4,DIV_R4           ; save R4

            AND     R0,R0,#0            ; reset R0
            NOT     R4,R4               ; negate R4
            ADD     R4,R4,#1

DIV_LOOP
            ADD     R0,R0,#1            ; increment R0
            ADD     R3,R3,R4            ; subtract R4 from R3
            BRp     DIV_LOOP

            ADD     R3,R3,#0
            BRzp    DIV_DONE
            ADD     R0,R0,#-1

DIV_DONE
            LD      R3,DIV_R3           ; restore R3
            LD      R4,DIV_R4           ; restore R4

            RET

DIV_R3      .BLKW   #1                  ; R3 saved value
DIV_R4      .BLKW   #1                  ; R4 saved value


; Performs power operation on two values.
;
; IN:  R3, R4 (values)
; OUT: R0 (result)
EXP
            ST      R1,EXP_R1           ; save R1
            ST      R4,EXP_R4           ; save R4
            ST      R7,EXP_R7           ; save R7

            AND     R0,R0,#0            ; reset R0
            ADD     R0,R0,#1            ; set R0 to 1
            ADD     R1,R4,#0            ; copy R4 to R1 to use as a counter

EXP_LOOP
            ADD     R4,R0,#0            ; copy R0 to R4
            JSR     MUL                 ; multiply R3 and R4
            ADD     R1,R1,#-1           ; decrement R1
            BRp     EXP_LOOP

            LD      R1,EXP_R1           ; restore R1
            LD      R4,EXP_R4           ; restore R4
            LD      R7,EXP_R7           ; restore R7

            RET

EXP_R1      .BLKW   #1                  ; R1 saved value
EXP_R4      .BLKW   #1                  ; R4 saved value
EXP_R7      .BLKW   #1                  ; R7 saved value


STACK_END   .FILL   x3FF0               ; pointer to end of the stack
STACK_START .FILL   x4000               ; pointer to start of the stack
STACK_TOP   .FILL   x4000               ; pointer to current top of the stack

SPACE_CHAR  .FILL   x20                 ; ' ' character
EQUAL_CHAR  .FILL   x3D                 ; '=' character
PLUS_CHAR   .FILL   x2B                 ; '+' character
MIN_CHAR    .FILL   x2D                 ; '-' character
MUL_CHAR    .FILL   x2A                 ; '*' character
DIV_CHAR    .FILL   x2F                 ; '/' character
EXP_CHAR    .FILL   x5E                 ; '^' character
ZERO_CHAR   .FILL   x30                 ; '0' character
NINE_CHAR   .FILL   x39                 ; '9' character
A_CHAR      .FILL   x41                 ; 'A' character

INVALID_MSG .STRINGZ "Invalid Expression"

            .END