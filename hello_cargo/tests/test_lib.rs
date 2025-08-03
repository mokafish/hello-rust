use hello_cargo::inc;

#[test]
fn inc5() {
    
    assert_eq! (6, inc(5));
}