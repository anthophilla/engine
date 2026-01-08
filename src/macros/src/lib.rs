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
fn make_op_f32(op: proc_macro2::TokenStream, fields: &syn::Fields) -> Vec<proc_macro2::TokenStream> {
    let mut fieldsv: Vec<proc_macro2::TokenStream>= vec![];
    for field in fields {
        let f = field.ident.as_ref().unwrap();
        fieldsv.push(quote! {
            #f: self.#f #op other,
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

    let fields = &data.fields;

    let add = make_op(quote!{+}, fields);
    let sub = make_op(quote!{-}, fields);
    let mul = make_op(quote!{*}, fields);
    let div = make_op(quote!{/}, fields);

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
fn impl_vectops_scalar(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;
    let data = match &input.data {
        syn::Data::Struct(x) => x,
        _ => panic!(),
    };

    let fields = &data.fields;

    let add = make_op_f32(quote! {+}, fields);
    let sub = make_op_f32(quote!{-}, fields);
    let mul = make_op_f32(quote!{*}, fields);
    let div = make_op_f32(quote!{/}, fields);

    let generated = quote! {
        impl std::ops::Add<f32> for #name {
            type Output = Self; fn add(self, other: f32) -> Self { Self { #(#add)* } }
        }
        impl std::ops::Sub<f32> for #name {
            type Output = Self; fn sub(self, other: f32) -> Self { Self { #(#sub)* } }
        }
        impl std::ops::Mul<f32> for #name {
            type Output = Self; fn mul(self, other: f32) -> Self { Self { #(#mul)* } }
        }
        impl std::ops::Div<f32> for #name {
            type Output = Self; fn div(self, other: f32) -> Self { Self { #(#div)* } }
        }
    };
    generated.into()
}

#[proc_macro_derive(VectOps)]
pub fn vectops_scalar_derive(input: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse(input).unwrap();
    let vectops: proc_macro2::TokenStream = impl_vectops(&input).into();
    let vectops_scalar: proc_macro2::TokenStream = impl_vectops_scalar(&input).into();
    quote!{
        #vectops_scalar
        #vectops
    }.into()
}

//TODO: #[proc_macro_derive(Uniform)]