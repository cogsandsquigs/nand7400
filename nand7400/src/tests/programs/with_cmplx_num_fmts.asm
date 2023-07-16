// Test positive and negative numbers.
add +0x01 -0x02 0x03

// Test values that overflow i8s but not u8s. These should not overflow, as they should be
// parsed as u8s.
add 0x7f 0xCA 0xFE

// Test values that are in different bases.
lda 0b01011010
ldb 0o361
jmp 0x10 91