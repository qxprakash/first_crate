/// Returns a greeting message.
pub fn greet() -> &'static str {
    "Hello from the first crate!"
}


#[docify::export(new_example_embed)]
pub fn example_to_embed() {
    assert_eq!(2 + 2, 4);
    assert_eq!(2 + 3, 5);
    println!("Example running from first_crate! right now. dummy change for checking caching , caching test , update");
}
