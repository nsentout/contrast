#![recursion_limit="128"]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

/// This allows our marks to derive from the MarkMacro trait defined in contrast/src/lib.rs.
/// Thus, our marks all shares the methods of MarkMacro and we don't have to implement them
/// for each mark, which would be a lot of duplicated code.
#[proc_macro_derive(MarkMacro)]
pub fn mark_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_mark_macro(&ast)
}

/// Here are the implementations of the methods of the MarkMacro trait.
/// Their behavior is the same for each mark type
fn impl_mark_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {  // quote! lets us write the Rust code that we want to return
        impl MarkMacro for #name {
            fn get_id(&self) -> properties::markid::MarkId
            {
                self.common_properties.markid
            }

            fn get_size(&self) -> properties::size::Size
            {
                self.common_properties.size
            }

            fn get_color(&self) -> properties::color::Color
            {
                self.common_properties.color
            }

            fn get_rotation(&self) -> f32
            {
                self.common_properties.rotation
            }

            fn get_layer_index(&self) -> usize
            {
                self.common_properties.markid.layer_index
            }

            fn set_size<S : Into <properties::size::Size>>(&mut self, size : S) -> &mut #name
            {
                self.common_properties.size = size.into();
                self
            }

            fn set_color<C : Into <properties::color::Color>>(&mut self, color : C) -> &mut #name
            {
                self.common_properties.color = color.into();
                self
            }

            fn set_rotation(&mut self, rotation : f32) -> &mut #name
            {
                self.common_properties.rotation = rotation;
                self
            }

            fn set_layer_index(&mut self, layer_index : usize) -> &mut #name
            {
                self.common_properties.markid.layer_index = layer_index;
                self
            }
        }
    };

    gen.into()  // converts "gen" into something understable by the compiler
}