#![cfg(test)]

/// Test that the multispace regex works.
#[test]
fn test_multispace() {
    let re = crate::formatter::MULTISPACE
        .lock()
        .expect("A mutex was poisoned!");

    assert_eq!(re.replace_all("   ", " "), " ");
    assert_eq!(re.replace_all("  ", " "), " ");
    assert_eq!(re.replace_all("\t\t\t", " "), " ");
    assert_eq!(re.replace_all("\t\t", " "), " ");
    assert_eq!(re.replace_all("\t", " "), " ");
    assert_eq!(re.replace_all(" ", " "), " ");
}

/// Test that formatting works on example code.
#[test]
fn test_example_format() {
    let code = "
    ; Write some assembly...\n\
    jmp LABEL\n\
    nop\n\
    nop\n\
    \n\
    LABEL:\n\
        add #0x01 #0x02 #0x03\n\
        lda #-0x01\n\
        ldb +0x01\n\
    ";

    let expected = "jmp LABEL\n\
                    nop\n\
                    nop\n\
                    \n\
                    LABEL:\n\
                    \tadd #0x01 #0x02 #0x03\n\
                    \tlda #-0x01\n\
                    \tldb +0x01\n";
}
