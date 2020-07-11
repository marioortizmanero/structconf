//! Testing some cases where a StructConf-derived struct should compile or
//! not. The files in `compile_fail` and `compile_pass` won't be detected by
//! cargo, and they will be ran by the crate `trybuild`.

#[test]
fn compile_fails() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile_pass/*.rs");
    t.compile_fail("tests/compile_fail/*.rs");
}
