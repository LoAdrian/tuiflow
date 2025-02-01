use std::any::{Any, TypeId};

use proc_macro::TokenStream;
use syn::{Field, FnArg, Generics, Ident, ImplItem, ImplItemFn, ItemImpl, ItemStruct, Pat, ReturnType, Type, Visibility};
use quote::{format_ident, quote};

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
        attrs,
        vis,
        struct_token: _,
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
    result.extend(get_builder_from_names_and_types(ident, field_names_and_types, generics, vis));
    return result.into();
}

// TODO: The generated builder for functions does not work yet. Fix it at some point.
fn get_builder_from_fn(item_impl: ItemImpl) -> TokenStream {
    let ItemImpl {
        attrs,
        defaultness,
        unsafety,
        impl_token,
        generics,
        trait_,
        self_ty,
        brace_token,
        items,
    } = item_impl.clone();
    let constructor = items.iter().find_map(|item| {
        if let ImplItem::Fn(fn_item) = item {
            if let ReturnType::Type(_ , ref ty) = fn_item.sig.output {
                if let Type::Path(ref type_path) = **ty {
                    if type_path.path.segments.first().unwrap().ident == format_ident!("Self") {
                        return Some(fn_item);
                    }
                }
                return Some(fn_item);
            }
        } 
        None
    });
    
    let ImplItemFn {
        attrs,
        vis,
        defaultness,
        sig,
        block,
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
    if let syn::Type::Path(type_path) = *self_ty { 
        let type_name = type_path.path.segments.last().expect("Failed to parse input function type path").ident.clone();
        result.extend(get_builder_from_names_and_types(type_name, names_and_types, generics, vis.clone()));
    } else {
        panic!("Failed to parse input function self type.");
    }
    return result.into();
}

fn get_builder_from_names_and_types(struct_identifier: Ident, fields: Vec<(Ident, Type)>, generics: Generics, builder_visibility: Visibility) -> proc_macro2::TokenStream {
    let mut field_holders = quote!();
    let mut field_setters = quote!();
    let mut optional_initializers = quote!();
    let mut build_field_initializers = quote!();

    for (field_identifier, field_type) in fields {
        
        field_holders.extend(quote!(
            #field_identifier: Option<#field_type>,
        ));

        // TODO: Handle cases with Null identifiers!
        let setter_identifier = format_ident!("with_{}", field_identifier);
        field_setters.extend(quote!(
            pub fn #setter_identifier(&mut self, #field_identifier: #field_type) -> &mut Self {
                self.#field_identifier = Some(#field_identifier);
                self
            }
        ));

        if let syn::Type::Path(type_path) = field_type {
            let type_ident = type_path.path.segments.first().unwrap().ident.clone();
            if type_ident == format_ident!("Option") {
                optional_initializers.extend(quote!(
                    #field_identifier: Some(None),
                ));
            }
        }

        let field_not_set_error = format!("Field {} is required to build {}.", field_identifier, struct_identifier);
        build_field_initializers.extend(quote!(
            #field_identifier: self.#field_identifier.clone().expect(#field_not_set_error),
        ));
    }

    let builder_identifier = format_ident!("{}Builder", struct_identifier);
    let builder_code = quote!(
        #[derive(Default)]
        #builder_visibility struct #builder_identifier<#generics> {
            #field_holders
        }
        impl<#generics> #builder_identifier<#generics> {
            pub fn new() -> Self {
                Self {
                    #optional_initializers
                    ..Default::default()
                }
            }

            #field_setters

            pub fn build(&self) -> #struct_identifier {
                return #struct_identifier {
                    #build_field_initializers
                };
            }
        }
    );
    
    builder_code.into()   
}

// TODO: Add tests
#[cfg(test)]
mod tests {

}