#![recursion_limit="128"]
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

/*
 *  This allows our marks to derive from the MarkMacro trait defined in contrast/src/lib.rs.
 *  Thus, our marks all shares the methods of MarkMacro and we don't have to implement them
 *  for each mark, which would be a lot of duplicated code.
 */
#[proc_macro_derive(MarkMacro)]
pub fn mark_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_mark_macro(&ast)
}

/*
 *  Here are the implementations of methods of the MarkMacro trait.
 *  Their behavior is the same for each mark type
 */
fn impl_mark_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {  // quote! lets us write the Rust code that we want to return
        impl MarkMacro for #name {
            fn get_mark_type(&self) -> String
            {
                String::from(stringify!(#name))
            }

            fn get_id(&self) -> usize
            {
                self.common_properties.id
            }

            fn set_size(&mut self, width : f32, height : f32) -> &mut #name
            {
                self.common_properties.size.width = width;
                self.common_properties.size.height = height;
                self
            }

            fn set_color(&mut self, r : f32, g : f32, b : f32, a : f32) -> &mut #name
            {
                self.common_properties.color.r = r;
                self.common_properties.color.g = g;
                self.common_properties.color.b = b;
                self.common_properties.color.a = a;
                self
            }
            fn set_rotation(&mut self, rotation : f32) -> &mut #name
            {
                self.common_properties.rotation = rotation;
                self
            }
        }
    };

    gen.into()  // converts "gen" into something understable by the compiler
}