//Function-like Procedural Macro (#[proc_macro])
use proc_macro::TokenStream;
#[proc_macro]
pub fn shout(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let output = input_str.to_uppercase();
    output.parse().unwrap()
}

