use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use syn::{Field, FnArg, Generics, Ident, ImplItem, ImplItemFn, ItemImpl, ItemStruct, Pat, ReturnType, Type, Visibility};

pub fn buildable_impl(item: TokenStream) -> TokenStream {
    let parsed_item = syn::parse::<ItemStruct>(item.clone()); // TODO: Mitigate this clone
    if let Ok(input_struct) = parsed_item {
        return get_builder_from_struct(input_struct);
    } else {
        let parsed_item = syn::parse::<ItemImpl>(item);
        if let Ok(item_impl) = parsed_item {
            return get_builder_from_fn(item_impl);
        }
    }
    panic!("Failed to generate builder from input struct or function.");
    //compile_error!("Failed to parse input struct."); //TODO: Find out how to use compile_error instead of panic
}

fn get_builder_from_struct(input_struct: ItemStruct) -> TokenStream {
    let ItemStruct {
        attrs: _attrs,
        vis,
        struct_token: _struct_token,
        ident,
        generics,
        fields,
        semi_token: _,
    } = input_struct.clone(); // TODO: Mitigate clone
    
    let field_names_and_types = fields.iter().map(|field| {
        let Field {
            attrs: _,
            vis: _,
            mutability: _,
            ident,
            colon_token: _,
            ty,
        } = field;
        (ident.clone().unwrap(), ty.clone())
    }).collect::<Vec<(Ident, Type)>>();
    let mut result = quote!(#input_struct);
    result.extend(get_builder_from_names_and_types(ident.clone(), None, quote!{#ident}, field_names_and_types, generics, vis));
    result.into()
}

// TODO: Make the builder visibility the same as the target struct, not instead of the visiblitity of the builders methods
fn get_builder_from_fn(item_impl: ItemImpl) -> TokenStream {
    let ItemImpl {
        attrs: _attrs,
        defaultness: _defaultness,
        unsafety: _unsafety,
        impl_token: _impl_token,
        generics,
        trait_: _trait_,
        self_ty,
        brace_token: _brace_token,
        items,
    } = item_impl.clone();
    let constructor = items.iter().find_map(|item| {
        if let ImplItem::Fn(fn_item) = item {
            // TODO clean up this double if
            if fn_item.sig.ident == format_ident!("new") {
                return Some(fn_item);
            }

            if let ReturnType::Type(_ , ref ty) = fn_item.sig.output {
                if let Type::Path(ref type_path) = **ty {
                    if type_path.path.segments.first().unwrap().ident == format_ident!("Self") {
                        return Some(fn_item);
                    }
                }
            }
        } 
        None
    });
    
    let ImplItemFn {
        attrs: _attrs,
        vis,
        defaultness: _defaultness,
        sig,
        block: _block,
    } = constructor.expect("Failed to find constructor function in input impl block.");
    
    let names_and_types = sig.inputs.iter().map(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(ref ident) = *pat_type.pat {
                let ty = *pat_type.ty.clone();
                return (ident.ident.clone(), ty);
            }
        }
        panic!("Failed to parse input function arguments.");
    }).collect::<Vec<(Ident, Type)>>();
    let mut result = quote!(#item_impl);
    if let Type::Path(ref type_path) = *self_ty { 
        let type_name= type_path.path.segments.last().expect("Failed to parse input function type path").ident.clone();
        let return_type = if let ReturnType::Type(_, ref ty) = sig.output { 
            
            let return_type = ty.as_ref().into_token_stream().to_string();
            let return_type_explicit_self = return_type.replace("Self", &type_name.to_string());
            let return_type_token_explicit_self = proc_macro2::TokenStream::from_str(&return_type_explicit_self).unwrap(); //TODO: error handling
            

            quote!(#return_type_token_explicit_self)
        } else {
            quote!(())
        };

        result.extend(get_builder_from_names_and_types(type_name, Some(sig.ident.clone()), return_type, names_and_types, generics, vis.clone()));
    } else {
        panic!("Failed to parse input function self type.");
    }
    result.into()
}

fn get_builder_from_names_and_types(struct_identifier: Ident, constructor_identifier: Option<Ident>, builder_return_type: proc_macro2::TokenStream, fields: Vec<(Ident, Type)>, generics: Generics, builder_visibility: Visibility) -> proc_macro2::TokenStream {
    let mut field_holders = quote!();
    let mut field_setters = quote!();
    let mut optional_initializers = quote!();
    let mut build_field_initializers = quote!();

    let mut current_lifetime_parameter = ('a'..='z').into_iter();
    let mut lifetimes = Vec::new();
    for (field_identifier, field_type) in fields {
        let mut field_type_with_lifetime = field_type.clone();
        let mut is_reference_field = false;
        if let Type::Reference(type_reference) = field_type {
            is_reference_field = true;
            let lifetime = syn::Lifetime::new(&format!("'{}", current_lifetime_parameter.next().unwrap()), Span::call_site()); //TODO: Check for better ways to generate this span
            lifetimes.push(lifetime.clone());
            field_type_with_lifetime = Type::Reference(syn::TypeReference {
                and_token: type_reference.and_token,
                lifetime: Some(lifetime), 
                mutability: type_reference.mutability,
                elem: type_reference.elem,
            });
        }
        field_holders.extend(quote!(
            #field_identifier: Option<#field_type_with_lifetime>,
        ));

        // TODO: Handle cases with Null identifiers!
        extend_field_setter_fn(&mut field_setters, &field_identifier, &field_type_with_lifetime);
        extend_optional_initializer(&mut optional_initializers, &field_identifier, &field_type_with_lifetime);
        if let Some(ctor_ident) = constructor_identifier.clone() {
            extend_build_argument(&mut build_field_initializers, &field_identifier, &struct_identifier, &ctor_ident, is_reference_field);
        } else {
            extend_build_field_initializer(&mut build_field_initializers, &field_identifier, &struct_identifier, is_reference_field);
        }
    }

    let builder_identifier = format_ident!("{}Builder", struct_identifier);
    
    let ctor_call = if let Some(ctor_name) = constructor_identifier {
        quote! {
            #struct_identifier::#ctor_name (
                #build_field_initializers
            )
        }
    } else {
        quote! {
            #struct_identifier {
                #build_field_initializers 
            }
        }
    };
    
    let builder_declaration = if !lifetimes.is_empty() || generics.params.len() > 0 {
        quote!(#builder_visibility struct #builder_identifier<#(#lifetimes)*, #generics>)
    } else {
        quote!(#builder_visibility struct #builder_identifier)
    };

    let impl_block_declaration = if !lifetimes.is_empty() || generics.params.len() > 0 {
        quote!(impl<#(#lifetimes)*, #generics> #builder_identifier<#(#lifetimes)*, #generics>)
    } else {
        quote!(impl #builder_identifier)
    };
    let builder_code = quote!(
        #[derive(Default)]
        #builder_declaration {
            #field_holders
        }
        #impl_block_declaration {
            pub fn new() -> Self {
                Self {
                    #optional_initializers
                    ..Default::default()
                }
            }

            #field_setters

            pub fn build(&mut self) -> #builder_return_type {
                #ctor_call
            }
        }
    );
    
    builder_code.into()   
}

fn extend_field_setter_fn(field_setters: &mut proc_macro2::TokenStream, field_identifier: &Ident, field_type: &Type) {
    let setter_identifier = format_ident!("with_{}", field_identifier);
    field_setters.extend(quote!(
        pub fn #setter_identifier(&mut self, #field_identifier: #field_type) -> &mut Self {
            self.#field_identifier = Some(#field_identifier);
            self
        }
    ));
}

fn extend_optional_initializer(optional_initializers: &mut proc_macro2::TokenStream, field_identifier: &Ident, field_type: &Type) {
    if let Type::Path(type_path) = field_type {
        let type_ident = type_path.path.segments.first().unwrap().ident.clone();
        if type_ident == format_ident!("Option") {
            optional_initializers.extend(quote!(
                #field_identifier: Some(None),
            ));
        }
    }
}

fn extend_build_field_initializer(build_field_initializers: &mut proc_macro2::TokenStream, field_identifier: &Ident, struct_identifier: &Ident, is_reference_field: bool) {
    let field_not_set_error = format!("Field {} is required to build {}.", field_identifier, struct_identifier);
    build_field_initializers.extend(get_builder_field_option(field_identifier, is_reference_field, field_not_set_error));
}

fn extend_build_argument(build_arguments: &mut proc_macro2::TokenStream, field_identifier: &Ident, struct_identifier: &Ident, ctor_identifier: &Ident, is_reference_field: bool) {
    let argument_not_set_error = format!("Argument {} is required to build {} via constructor {}.", field_identifier, struct_identifier, ctor_identifier);
    build_arguments.extend(get_builder_field_option(field_identifier, is_reference_field, argument_not_set_error));
}

fn get_builder_field_option(field_identifier: &Ident, is_reference_field: bool, not_set_error: String) -> proc_macro2::TokenStream {
    if is_reference_field {
        quote!(
            self.#field_identifier.as_mut().expect(#not_set_error), //TODO: also allow for immutable fields
        )
    } else {
        quote!(
            self.#field_identifier.clone().expect(#not_set_error),
        )
    }

}

// TODO: Add tests
#[cfg(test)]
mod tests {

}