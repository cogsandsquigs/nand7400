// uniffi::include_scaffolding!("nand7400_asm");
// uniffi::setup_scaffolding!();

#[uniffi::export]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[uniffi::export]
fn flip(a: bool) -> bool {
    !a
}
