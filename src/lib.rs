extern crate proc_macro;

use proc_macro::{Delimiter, TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn return_as_is(_attr: TokenStream, item: TokenStream) -> TokenStream {
    for token in item.clone() {
        match token {
            TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => {
                println!("{:}", group)
            }
            _ => {}
        }
    }
    item
}
