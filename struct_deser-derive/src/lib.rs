//! Derive proc macro for `struct_deser` crate. See that one for more information.

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::{Tokens, ToTokens};

#[proc_macro_derive(StructDeser, attributes(struct_deser, be, le))]
pub fn derive_struct_deser(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let gen = impl_struct_deser(&ast);

    gen.parse().unwrap()
}

// Not to be confused  with one in byteorder crate...
#[derive(Eq, PartialEq)]
enum ByteOrder {
    LE,
    BE,
}

impl ToTokens for ByteOrder {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match *self {
            ByteOrder::LE => tokens.append("LE"),
            ByteOrder::BE => tokens.append("BE"),
        }
    }
}

// Actual implementation
fn impl_struct_deser(ast: &syn::DeriveInput) -> quote::Tokens {
    use syn::{Body, VariantData};
    use quote::Ident;

    let mut res = quote::Tokens::new();
    let body = if let Body::Struct(ref body) = ast.body {
        body
    } else {
        panic!("The type must be a struct");
    };

    impl_identifier(ast, &mut res);

    let name = &ast.ident;
    let dummy_const = Ident::new(format!("_IMPL_STRUCT_DESER_FOR_{}", name));

    let mut deser_body = quote::Tokens::new();
    let mut ser_body = quote::Tokens::new();
    let mut byte_len = quote! { 0 };
    for (field_no, field) in body.fields().iter().enumerate() {
        let ty = &field.ty;

        let byte_order = get_byte_order(&field.attrs);

        let field_accessor = match field.ident {
            Some(ref ident) => quote! { #ident },
            None => {
                // Interpolating directly would cause adding `usize` sufix
                let mut tmp = Tokens::new();
                tmp.append(format!("{}", field_no));
                tmp
            },
        };

        let byte_slice = quote! { bytes[(#byte_len)..(#byte_len + <#ty as _struct_deser::SerializedByteLen>::BYTE_LEN)] };

        let (deser_impl, ser_impl) = match byte_order {
            None => (quote! { _struct_deser::FromBytes::from_bytes(&#byte_slice) },
                     quote! { _struct_deser::IntoBytes::into_bytes(&self.#field_accessor, &mut #byte_slice); }),
            Some(bo) => (quote! { _struct_deser::FromBytesOrdered::from_bytes::<_struct_deser::byteorder::#bo>(&#byte_slice) },
                         quote! { _struct_deser::IntoBytesOrdered::into_bytes::<_struct_deser::byteorder::#bo>(&self.#field_accessor, &mut #byte_slice); }),
        };

        deser_body.append(match field.ident {
            Some(ref ident) => quote! { #ident: #deser_impl, },
            None => quote! { #deser_impl, },
        });
        ser_body.append(ser_impl);

        byte_len.append(quote! { + <#ty as _struct_deser::SerializedByteLen>::BYTE_LEN });
    }

    match *body {
        VariantData::Struct(_) => res.append(quote! {
            impl _struct_deser::FromBytes for #name {
                fn from_bytes(bytes: &[u8]) -> Self {
                    assert_eq!(bytes.len(), <Self as _struct_deser::SerializedByteLen>::BYTE_LEN);

                    #name {
                        #deser_body
                    }
                }
            }
        }),
        VariantData::Tuple(_) => res.append(quote! {
            impl _struct_deser::FromBytes for #name {
                fn from_bytes(bytes: &[u8]) -> Self {
                    assert_eq!(bytes.len(), <Self as _struct_deser::SerializedByteLen>::BYTE_LEN);

                    #name(#deser_body)
                }
            }
        }),
        VariantData::Unit => panic!("(De)serializing empty struct doesn't make sense"),
    }

    res.append(quote! {
        impl _struct_deser::IntoBytes for #name {
            fn into_bytes(&self, bytes: &mut [u8]) {
                assert_eq!(bytes.len(), <Self as _struct_deser::SerializedByteLen>::BYTE_LEN);
                #ser_body
            }
        }

        impl _struct_deser::SerializedByteLen for #name {
            const BYTE_LEN: usize = #byte_len;
        }
    });

    res = quote! {
        #[allow(non_upper_case_globals)]
        const #dummy_const: () = {
            extern crate struct_deser as _struct_deser;
            #res
        };
    };

    /*
    if name == "IntegersTuple" {
        panic!(res.to_string());
    }
    */
    res
}

// Impls identifier trait
fn impl_identifier(ast: &syn::DeriveInput, res: &mut Tokens) {
    use syn::{MetaItem, NestedMetaItem, Lit};

    let name = &ast.ident;

    for attr in &ast.attrs {
        if attr.value.name() == "struct_deser" {
            if let MetaItem::List(_, ref nested) = attr.value {
                let mut val = None;
                let mut ty = None;
                for item in nested {
                    if let NestedMetaItem::MetaItem(MetaItem::NameValue(ref name, ref value)) = *item {
                        if name == "identifier" {
                            val = Some(value);
                        }

                        if name == "identifier_type" {
                            ty = Some(value);
                        }
                    }
                }

                match (val, ty) {
                    (Some(&Lit::Str(ref val, _)), Some(&Lit::Str(ref ty, _))) => {
                        let ty = syn::parse_type(ty).expect("expected type");
                        let val = syn::parse_expr(val).expect("expected expression");

                        res.append(quote! {
                            impl _struct_deser::Identifier for #name {
                                type IdentifierType = #ty;
                                const IDENTIFIER: Self::IdentifierType = #val;
                            }
                        });
                        return;
                    },
                    (None, None) => (),
                    (Some(_), Some(_)) => panic!("Identifier and it's type must be inside string"),
                    _ => panic!("Both identifier and type must be specified or none of them"),
                }
            }
        }
    }
}

// Scans attributes for byte order
fn get_byte_order(attrs: &[syn::Attribute]) -> Option<ByteOrder> {
    use syn::MetaItem;

    let mut byte_order = None;
    for attr in attrs {
        if let MetaItem::Word(ref word) = attr.value {
            if word.as_ref() == "be" {
                byte_order = Some(ByteOrder::BE);
                if byte_order == Some(ByteOrder::LE) {
                    panic!("Conflicting byte order: you can't specify both Little and Big endian");
                }
            }

            if word.as_ref() == "le" {
                byte_order = Some(ByteOrder::LE);
                if byte_order == Some(ByteOrder::BE) {
                    panic!("Conflicting byte order: you can't specify both Little and Big endian");
                }
            }
        }
    }

    byte_order
}
