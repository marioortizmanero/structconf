#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/parse_0.rs");
    t.pass("tests/parse_1.rs");
    t.pass("tests/parse_2.rs");
    // t.pass("tests/parse_3.rs");
}
