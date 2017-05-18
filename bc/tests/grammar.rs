 
 
mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}
 
 
#[test]
fn test_decimal() {
    assert_eq!(grammar::decimal("343.234"), Ok("343.234".to_string()));
    assert_eq!(grammar::decimal(".234"), Ok(".234".to_string()));
    assert_eq!(grammar::decimal("343"), Ok("343".to_string()));
}

