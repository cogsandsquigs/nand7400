




// There's whitespace at the beginning and end to test the parsing of extraneous newlines/whitespace!
// Here's one comment
// Here's another comment

/*
	This is a multi-line comment! anything can go here!
	Yay!
*/



// Now for some *real* code!
nop
lda 0x09
jmp LABEL

LABEL:
	nop
	add 0x01 0x02 0x03
	hlt

