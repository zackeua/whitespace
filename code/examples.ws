[Space] [Tab]	[LB]

Stack Manipulation: [Space]
push [Space] (Number)       Push (number) onto the stack
dupl [LF][Space]            Duplicate the top item on the stack
swap [LF][Tab]              Swap the top two items
pop [LF][LF]                Discard the top item on the stack

Arithmetic: [Tab][Space]    The first item pushed is left of the op
add [Space][Space]          Add the two top items on the stack
sub [Space][Tab]            Subtract the top items on the stack
mul [Space][LF]             Multiply the top two items
div [Tab][Space]            Divide the top two items
mod [Tab][Tab]              Take modulo of the top two items

Heap Access: [Tab][Tab]
store [Space]               Push address then value to store item on the stack
get [Tab]                   Push address to stack

Flow Control: [LF]
label [Space][Space](label) Mark a location in the program
call [Space][Tab](label)    Call a subroutine (label)
jump [Space][LF](label)     Jump unconditionally to a label
jumpz [Tab][Space](label)   Jump to a label if item on top of stack is zero
jumpl [Tab][Tab](label)     Jump to a label if item on top if stack is negative
endcall [Tab][LF]           End a subroutine and transfer control back to the caller
end [LF][LF]                End the program

I/O: [Tab][LF]
printc [Space][Space]       Output the char at the top of the stack
printi [Space][Tab]         Output the number at the top of the stack
readc [Tab][Space]          Read a char and place it in the location given by the top of the stack
readi [Tab][Tab]            Read a number and place it in the location given by the top of the stack
