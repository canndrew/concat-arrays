use {
    syn::{
        Expr,
        punctuated::Punctuated,
        parse::{Parse, ParseStream},
    },
    proc_macro::TokenStream,
    quote::{format_ident, quote},
};

struct Args {
    punctuated: Punctuated<Expr, syn::Token![,]>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::parse::Result<Args> {
        let punctuated = Punctuated::parse_terminated(input)?;
        Ok(Args { punctuated })
    }
}

/// Concatenates arrays.
///
/// # Example
///
/// ```
/// # use concat_arrays::concat_arrays;
/// let x = [0];
/// let y = [1, 2];
/// let z = [3, 4, 5];
/// let concatenated = concat_arrays!(x, y, z);
/// assert_eq!(concatenated, [0, 1, 2, 3, 4, 5]);
/// ```
///
/// # Limitations
///
/// Due to limitations in rust `concat_arrays!` can't tell the compiler what the length of the
/// returned array is. As such, the length needs to be inferrable from the surrounding context. For
/// example, in the example above the length is inferred by the call to `assert_eq!`. It is safe to
/// mis-specify the length however - you'll just get a compilation error rather than broken code.
#[proc_macro]
pub fn concat_arrays(tokens: TokenStream) -> TokenStream {
    let arrays = syn::parse_macro_input!(tokens as Args);
    let arrays: Vec<Expr> = arrays.punctuated.into_iter().collect();
    let field_names = {
        let mut field_names = Vec::with_capacity(arrays.len());
        for i in 0..arrays.len() {
            field_names.push(format_ident!("concat_arrays_arg_{}", i));
        }
        field_names
    };
    let define_concat_arrays_type = {
        let type_arg_names = {
            let mut type_arg_names = Vec::with_capacity(arrays.len());
            for i in 0..arrays.len() {
                type_arg_names.push(format_ident!("ConcatArraysArg{}", i));
            }
            type_arg_names
        };
        quote! {
            #[repr(C)]
            struct ConcatArrays<#(#type_arg_names,)*> {
                #(#field_names: #type_arg_names,)*
            }
        }
    };
    let ret = quote! {{
        #(
            let #field_names = #arrays;
        )*
        if false {
            fn constrain_concat_arrays_argument_to_be_an_array<const ARRAY_ARG_LEN: usize>(
                concat_arrays_arg: &[u8; ARRAY_ARG_LEN],
            ) {
                let _ = concat_arrays_arg;
            }
            #(
                constrain_concat_arrays_argument_to_be_an_array(&#field_names);
            )*
        }
        loop {
            #define_concat_arrays_type

            let concat_arrays = ConcatArrays {
                #(#field_names,)*
            };
            if false {
                fn infer_length_of_concatenated_array<const INFERRED_LENGTH_OF_CONCATENATED_ARRAY: usize>()
                    -> [u8; INFERRED_LENGTH_OF_CONCATENATED_ARRAY]
                {
                    [0u8; INFERRED_LENGTH_OF_CONCATENATED_ARRAY]
                }
                break infer_length_of_concatenated_array();
            }
            break unsafe {
                ::core::mem::transmute(concat_arrays)
            };
        }
    }};
    ret.into()
}
