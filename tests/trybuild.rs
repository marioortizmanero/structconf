#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/parse/*.rs");
    t.compile_fail("tests/error/*.rs");
}
