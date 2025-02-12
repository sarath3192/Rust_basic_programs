use proc_macro2::TokenStream;

#[proc_macro]
pub fn shout(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let output = input_str.to_uppercase();
    output.parse().unwrap()
}

fn main() {
    println!("{}", shout!(hello world));  // Output: HELLO WORLD
}
