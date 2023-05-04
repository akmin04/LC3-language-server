.ORIG x3000

; unexpected token
LABEL   .stringz "asdf" R5

; #16 is out of range 
        ADD R1,R3,#16

; incorrect argument type
        NOT R1,#2     

; label that doesn't exist          
        JSR FAKE_LABEL

; incorrect number of arguments
        ADD R1

; directive that doesn't exist
.END2

.END