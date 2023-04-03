
#![feature(proc_macro_quote)]
#![warn(clippy::all, clippy::pedantic)]
use syn::parse2;
extern crate proc_macro;

use crate::generate::Generate;
use crate::keywords::Root;

#[proc_macro]
pub fn run_tests(input_proc_macro: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input_proc_macro);

    let root = match parse2::<Root>(input) {
        Ok(root) => root,
        Err(err) => panic!("got error when parse input {}", err)
    };
    let code = root.generate();
    println!("{}", code);
    code.into()
}


mod generate;
mod keywords;


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use proc_macro2::{TokenStream};
    use crate::macro_rules::run_tests;

    #[test]
    fn test() {
        assert_eq!(1, 1)
    }
    #[test]
    fn small_test() {
        let input: TokenStream = TokenStream::from_str(
            "suite T { TestFunc, TestFunc2 }").expect("ds");
        println!("{}", input);
        let res = run_tests(proc_macro::TokenStream::from(input));
        assert_eq!(res.to_string(), "".to_string());
        return;
    }
}