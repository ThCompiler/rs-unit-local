use convert_case::{Case, Casing};
use syn::{braced, parse::{Parse, ParseStream}, Ident, Result, Token};
use syn::punctuated::Punctuated;

pub const LIB_NAME: &str = "run_tests";
pub const SINGLETON_MOD: &str = "suite";
pub const SINGLETON_NAME: &str = "SuiteSingleton";

// Parsing entrypoint of the whole application.
#[derive(Debug)]
pub struct Root {
    pub ident: Ident,
    pub describes: Vec<Suite>,
}

// Parses all describe blocks within the `rs_unit!` run_tests_macro.
impl Parse for Root {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut describes = Vec::<Suite>::new();

        while !input.is_empty() {
            describes.push(input.parse()?);
        }

        let ident = Ident::new("tests", proc_macro2::Span::call_site());

        Ok(Self { ident, describes })
    }
}


// Suite block that contains the actual tests suite
#[derive(Debug)]
pub struct Suite {
    pub ident: Ident,
    pub suite_struct: Ident,
    pub suite_struct_name: String,
    pub functions: Vec<Func>,
}

// Parses the Suite block. The pre- and postprocessing blocks are optional.
impl Parse for Suite {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;
        let suite_struct = input.parse::<Ident>()?;
        let suite_struct_name = suite_struct.to_string().to_case(Case::Snake);

        let contents;
        let _braces = braced!(contents in input);
        let funcs: Punctuated<Ident, Token![,]> = Punctuated::parse_terminated(&contents)?;
        let mut functions: Vec<Func> = Vec::new();

        funcs.into_iter().for_each(|ident| {
            let func = ident;
            let name_func = func.to_string().to_case(Case::Snake);
            functions.push(Func { function: func, function_name: name_func, struct_ident: suite_struct.clone() });
        });

        Ok(Self {
            ident,
            suite_struct,
            suite_struct_name,
            functions,
        })
    }
}

// Describe block that contains the actual tests and any pre- and postprocessing blocks.
#[derive(Debug)]
pub struct Func {
    pub struct_ident: Ident,
    pub function: Ident,
    pub function_name: String,
}