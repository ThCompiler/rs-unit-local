//! Contains methods to generate the manipulated `Tokenstreams` based of the parsed AST.
use proc_macro2::{Ident, TokenStream};
use quote::{quote_spanned, TokenStreamExt};
use syn::Expr::Block;
use syn::spanned::Spanned;

use crate::keywords::{Describe, Root, Setup, SetupAll, Teardown, TeardownAll, Test};

pub trait Generate {
    fn generate(&self) -> TokenStream;

    fn generate_test(
        &self,
        _setup_all: &TokenStream,
        _setup: &Option<Setup>,
        _teardown_all: &TokenStream,
        _teardown: &Option<Teardown>,
    ) -> TokenStream {
        TokenStream::new()
    }
}

// Generates the outer wrapper test wrapper.
//
// ```rust
// #[cfg(test)]
// mod tests {
//     Here are the describe blocks
// }
// ```
impl Generate for Root {
    fn generate(&self) -> TokenStream {
        let ident = &self.ident;

        let describe_blocks = self
            .describes
            .iter()
            .map(|d| d.generate())
            .collect::<Vec<_>>();

        let root_block = quote_spanned! {ident.span()=>
            #[cfg(test)]
            mod #ident {
                #[allow(unused_imports)]
                use super::*;

                #(#describe_blocks)*
            }
        };

        root_block
    }
}

// Generates a module block that groups related tests. These modules are located in the `Root` block.
//
// ```rust
// mod add_numbers {
//     Here are your tests
// }
// ```
impl Generate for Describe {
    fn generate(&self) -> TokenStream {
        let ident = &self.ident;

        let tests = &self
            .tests
            .iter()
            .map(|t| {
                t.generate_test(
                    &self.setup_all,
                    &self.setup,
                    &self.teardown_all,
                    &self.teardown,
                )
            })
            .collect::<Vec<_>>();
        if self.setup_all.is_empty() && self.teardown_all.is_empty() {
            let describe_block = quote_spanned! {ident.span()=>
            mod #ident {
                use super::*;

                #(#tests)*
            }
        };

            describe_block
        } else {
            let describe_block = quote_spanned! {ident.span()=>
            mod #ident {
                use super::*;
                use std::sync::Once;

                static INIT: Once = Once::new();

                #(#tests)*
            }
        };

            describe_block
        }
    }
}

// Generates a valid Rust test function. These function are located within the modules where they belong to.
//
// # Example
//
// ```rust
// #[test]
// fn success_add_positive_numbers() {
//   let result = add(1,1);
//   assert_eq!(result, 2);
// }
// ```
impl Generate for Test {
    fn generate(&self) -> TokenStream {
        let sanitied_name = &self
            .name
            .to_string()
            .to_lowercase()
            .replace(" ", "_")
            .replace(":", "");
        let new_ident = Ident::new(sanitied_name, self.ident.span());

        let block = &self.content;

        let test_block = quote_spanned! {new_ident.span()=>
            #[test]
            fn #new_ident() {
                #block
            }
        };

        test_block
    }

    fn generate_test(
        &self,
        setup_all: &TokenStream,
        setup: &Option<Setup>,
        teardown_all: &TokenStream,
        teardown: &Option<Teardown>,
    ) -> TokenStream {
        let sanitied_name = &self
            .name
            .to_string()
            .to_lowercase()
            .replace(" ", "_")
            .replace(":", "");
        let new_ident = Ident::new(sanitied_name, self.ident.span());

        let setup_all = setup_all;
        let setup = setup.clone();
        let mut block = self.content.clone();
        let teardown_all = teardown_all;
        let teardown = teardown.clone();

        if let Some(mut stp) = setup {
            stp.content.stmts.append(&mut block.stmts);
            block = stp.content;
        }

        if let Some(mut stp) = teardown {
            block.stmts.append(&mut stp.content.stmts);
        }

        let test_block = quote_spanned! {new_ident.span()=>
            #[test]
            fn #new_ident() {
                #setup_all

                #block

                #teardown_all
            }
        };

        println!("{}", test_block);
        test_block
    }
}

// Generates a Rust function that is run before every test. These function are located within the modules where they belong to.
//
// # Example
//
// ```rust
// #[test]
// fn success_add_positive_numbers() {
//   setup();
//
//   let result = add(1,1);
//   assert_eq!(result, 2);
// }
// ```
impl Generate for Setup {
    fn generate(&self) -> TokenStream {
        let ident = &self.ident;
        let block = &self.content;

        let setup_block = quote_spanned!(ident.span()=> #block);

        setup_block
    }
}

// Generates a Rust function that is run after every test. These function are located within the modules where they belong to.
//
// # Example
//
// ```rust
// #[test]
// fn success_add_positive_numbers() {
//   let result = add(1,1);
//   assert_eq!(result, 2);
//
//   teardown();
// }
// ```
impl Generate for Teardown {
    fn generate(&self) -> TokenStream {
        let ident = &self.ident;
        let block = &self.content;

        let teardown_block = quote_spanned!(ident.span()=> #block);

        teardown_block
    }
}

// Generates a Rust function that is once before the tests were started. These function are located within the modules where they belong to.
impl Generate for SetupAll {
    fn generate(&self) -> TokenStream {
        let ident = &self.ident;
        let block = &self.content;

        let setup_all_block = quote_spanned! {ident.span()=>
            INIT.call_once(|| {
                #block
            });
        };

        setup_all_block
    }
}

// Generates a Rust function that is once after the tests were started. These function are located within the modules where they belong to.
impl Generate for TeardownAll {
    fn generate(&self) -> TokenStream {
        let ident = &self.ident;
        let block = &self.content;

        let teardown_all_block = quote_spanned! {ident.span()=>
            INIT.call_once(|| {
                #block
            });
        };

        teardown_all_block
    }
}
