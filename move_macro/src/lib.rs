extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn moves(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let sequence = input_str.value();

    let moves: Vec<String> = sequence.split_whitespace().map(|m| m.to_string()).collect();

    let expanded = quote! {
        {
            let mut moves_vec = Vec::new();
            #(
                moves_vec.push(Move::try_from(#moves).expect("moves! failed"));
            )*
            moves_vec
        }
    };

    TokenStream::from(expanded)
}
