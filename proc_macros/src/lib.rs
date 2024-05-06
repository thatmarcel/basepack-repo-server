#[proc_macro_derive(SileoDepictionView, attributes(sdv_rename))]
pub fn derive_duplicate_struct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let mut class_name_string = String::new();
    for attribute in input.attrs {
        let path = attribute.path();

        if !path.is_ident("sdv_class_name") {
            continue;
        }

        let argument = match attribute.parse_args::<proc_macro2::TokenStream>() {
            Ok(a) => a,
            Err(_) => continue
        };

        class_name_string = argument.to_string().replace("\"", "");
    }

    let name = &input.ident;
    let name_string = name.to_string();

    let data = match input.data {
        syn::Data::Struct(ds) => ds,
        _ => return proc_macro::TokenStream::new()
    };
    
    let mut serialization_code_lines = data.fields.iter().map(|field| {
        let field_name = &field.ident.clone().unwrap();

        let field_type = &field.ty;

        let mut field_serialized_name_string = field_name.clone().to_string();
        for attribute in &field.attrs {
            let path = attribute.path();

            if !path.is_ident("sdv_rename") {
                continue;
            }

            let argument = match attribute.parse_args::<proc_macro2::TokenStream>() {
                Ok(a) => a,
                Err(_) => continue
            };

            field_serialized_name_string = argument.to_string().replace("\"", "");
        }

        // Filter out Options with value None that would serialize to null
        if quote::ToTokens::to_token_stream(field_type).to_string().replace("&", "").replace("std::option::", "").starts_with("Option") {
            (quote::quote! {
                match &self.#field_name {
                    Some(_) => {
                        _ = serde::ser::SerializeStruct::serialize_field(&mut state, #field_serialized_name_string, &self.#field_name);
                    },
                    None => {}
                }
            }).to_string()
        } else {
            (quote::quote! {
                _ = serde::ser::SerializeStruct::serialize_field(&mut state, #field_serialized_name_string, &self.#field_name);
            }).to_string()
        }
    }).collect::<Vec<String>>();
    
    if name_string == "DepictionTabView" {
        serialization_code_lines.push((quote::quote! {
            _ = serde::ser::SerializeStruct::serialize_field(&mut state, "minVersion", "0.4");
        }).to_string());
    }

    let serialized_item_count: usize = serialization_code_lines.len() + 1;
    
    let combined_serialization_code_lines: proc_macro2::TokenStream = std::str::FromStr::from_str(serialization_code_lines.join("\n").as_str()).unwrap();

    let result = quote::ToTokens::to_token_stream(&quote::quote! {
        impl crate::sileo::sileo_depiction_view::SileoDepictionView for #name {}
        impl #name {
            pub fn into_box(self) -> Box<Self> {
                Box::new(self)
            }
        }

        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
                let mut state = serializer.serialize_struct(#name_string, #serialized_item_count)?;
                _ = serde::ser::SerializeStruct::serialize_field(&mut state, "class", #class_name_string);
                #combined_serialization_code_lines
                serde::ser::SerializeStruct::end(state)
            }
        }
    });

    proc_macro::TokenStream::from(result)
}

#[proc_macro_attribute]
pub fn sdv_class_name(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}