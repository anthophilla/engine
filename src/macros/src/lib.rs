extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

fn make_op(op: proc_macro2::TokenStream, fields: &syn::Fields) -> Vec<proc_macro2::TokenStream> {
    let mut fieldsv: Vec<proc_macro2::TokenStream>= vec![];
    for field in fields {
        let f = field.ident.as_ref().unwrap();
        fieldsv.push(quote! {
            #f: self.#f #op other.#f,
        }.into());
    };
    return fieldsv
}

fn impl_vectops(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;
    let data = match &input.data {
        syn::Data::Struct(x) => x,
        _ => panic!(),
    };

    let add = make_op(quote!{+}, &data.fields);
    let sub = make_op(quote!{-}, &data.fields);
    let mul = make_op(quote!{*}, &data.fields);
    let div = make_op(quote!{/}, &data.fields);

    let generated = quote! {
        impl std::ops::Add for #name {
            type Output = Self; fn add(self, other: Self) -> Self { Self { #(#add)* } }
        }
        impl std::ops::Sub for #name {
            type Output = Self; fn sub(self, other: Self) -> Self { Self { #(#sub)* } }
        }
        impl std::ops::Mul for #name {
            type Output = Self; fn mul(self, other: Self) -> Self { Self { #(#mul)* } }
        }
        impl std::ops::Div for #name {
            type Output = Self; fn div(self, other: Self) -> Self { Self { #(#div)* } }
        }
    };
    generated.into()
}

#[proc_macro_derive(VectOps)]
pub fn vectops_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse(input).unwrap();
    impl_vectops(&input)
}

//TODO: #[proc_macro_derive(Uniform)]