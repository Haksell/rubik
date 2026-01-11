extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

// TODO: NO

#[proc_macro]
pub fn moves(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let sequence = input_str.value();

    let moves: Vec<String> = sequence.split_whitespace().map(|m| m.to_string()).collect();

    let expanded = quote! {
        {
            use crate::puzzles::Puzzle;
            let puzzle = crate::puzzles::Cube::<3>::new(); // :( :( :( :(
            let mut moves_vec = Vec::new();
            #(
                moves_vec.push(puzzle.parse_move(#moves).expect("moves! failed"));
            )*
            moves_vec
        }
    };

    TokenStream::from(expanded)
}
