extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed, Ident, Type};

fn suffix_ident(ident: &Ident, suffix: &str) -> Ident {
    Ident::new(&format!("{}{}", ident, suffix), ident.span())
}

fn wrap_type(ty: &Type) -> proc_macro2::TokenStream {
    quote! {
        Diff<'t, #ty, <#ty as Diffable<'t>>::Detailed>
    }
}

fn collect_fields_into_diffable(input: &DeriveInput) -> Vec<proc_macro2::TokenStream> {
    let named_fields = collect_fields(input);
    let mut diffable_fields = Vec::new();
    for named_field in &named_fields.named {
        let ident = named_field.ident.as_ref().expect("a named field");
        let wrapped_type = wrap_type(&named_field.ty);
        diffable_fields.push(quote! {
            #ident : #wrapped_type
        });
    }

    diffable_fields
}

fn collect_fields<'t>(input: &'t DeriveInput) -> &'t FieldsNamed {
    use Data::*;
    if let Struct(strct) = &input.data {
        let fields = &strct.fields;
        if let Fields::Named(named_fields) = fields {
            named_fields
        } else {
            panic!("Can only handle structs with named fields");
        }
    } else {
        panic!("can only apply derivation on structs")
    }
}

fn collect_field_names<'t>(input: &'t DeriveInput) -> impl Iterator<Item = &'t Ident> {
    collect_fields(input).named.iter().flat_map(|f| &f.ident)
}

#[proc_macro_derive(Diffable)]
pub fn derive_diffable_fn(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let diff_struct_name = suffix_ident(&input.ident, "Diff");
    let diff_struct_fields = collect_fields_into_diffable(&input);
    let struct_name = &input.ident;

    let fields: Vec<_> = collect_field_names(&input).collect();
    let default_fields = fields.iter().map(|ident| quote! { #ident: Diff::Same });

    let check_fields = fields.iter().map(|ident| quote! {{
        let own_diff = self.#ident.diff(&other.#ident);
        if !own_diff.is_same() {
            all_the_same = false;
        }
        diff.#ident = own_diff;
    }});

    let output = quote! {
        #[derive(Debug)]
        struct #diff_struct_name<'t> {
            #(#diff_struct_fields),*
        }

        impl<'t> Default for #diff_struct_name<'t> {
            fn default() -> Self {
                #diff_struct_name {
                    #(#default_fields),*
                }
            }
        }

        impl<'t> Diffable<'t> for #struct_name {
            type Detailed = #diff_struct_name<'t>;
            fn diff(&'t self, other: &'t Self) -> Diff<'t, Self, Self::Detailed> {
                let mut all_the_same = true;
                let mut diff = #diff_struct_name::default();
                #(#check_fields)*

                if all_the_same {
                    Diff::Same
                } else {
                    Diff::Detailed(diff)
                }
            }
        }
    };

    output.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
