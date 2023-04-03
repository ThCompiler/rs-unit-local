use proc_macro2::{Ident, TokenStream};
use quote::{quote_spanned};

use crate::keywords::{Func, LIB_NAME, Root, SINGLETON_MOD, SINGLETON_NAME, Suite};

pub trait Generate {
    fn generate(&self) -> TokenStream;
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

        let suite_blocks = self
            .describes
            .iter()
            .map(|d| d.generate())
            .collect::<Vec<_>>();

        let root_block = quote_spanned! {ident.span()=>
            #[cfg(test)]
            mod #ident {
                #[allow(unused_imports)]
                use super::*;

                #(#suite_blocks)*
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
impl Generate for Suite {
    fn generate(&self) -> TokenStream {
        let ident = &self.ident;

        let functions = &self.functions
            .iter()
            .map(|f| {
                f.generate()
            })
            .collect::<Vec<_>>();

        let lib_name = Ident::new(LIB_NAME, proc_macro2::Span::call_site());
        let singleton_mod = Ident::new(SINGLETON_MOD, proc_macro2::Span::call_site());
        let singleton_name = Ident::new(SINGLETON_NAME, proc_macro2::Span::call_site());

        let describe_block = quote_spanned! {ident.span()=>
        mod #ident {
            use super::*;
            use #lib_name::#singleton_mod::#singleton_name;

            static mut suite: #singleton_name<#ident> = #singleton_name::new();

            #(#functions)*
        }
        };

        describe_block
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
impl Generate for Func {
    fn generate(&self) -> TokenStream {
        let new_ident = Ident::new(self.function_name.as_str(), self.function.span());

        let func = &self.function;

        let test_block = quote_spanned! {new_ident.span()=>
            #[test]
            fn #new_ident() {
                suite.get().defer_mut().before_test();
                suite.get().defer().#func;
                suite.get().defer_mut().after_test();
            }
        };

        test_block
    }
}