extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

#[proc_macro_derive(Goose)]
pub fn vertex_attrib_pointers_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = generate_impl(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn generate_impl(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl Goose for #name {
            fn fly() {
                println!("fly {} fly!", stringify!(#name));
            }
        }
    }
}