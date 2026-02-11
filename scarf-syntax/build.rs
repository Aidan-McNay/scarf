// =======================================================================
// build.rs
// =======================================================================
// Create an enum with all possible nodes, for iterating over a tree

use proc_macro2::TokenStream;
use quote::quote;
use std::path::Path;
use std::{fs, io::Read};
use syn;
use walkdir::WalkDir;

const SKIPPED_ENTRIES: &[&str] =
    &["metadata.rs", "iter.rs", "compiler_directives"];

fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build.rs");
    let mut node_names = Vec::new();
    let mut node_structs = Vec::new();
    let mut node_enums = Vec::new();
    for entry in WalkDir::new("src")
        .into_iter()
        .filter_entry(|e| {
            !SKIPPED_ENTRIES.contains(&e.file_name().to_string_lossy().as_ref())
        })
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_type().is_file()
                && e.path().extension().and_then(|s| s.to_str()) == Some("rs")
        })
    {
        let mut file =
            fs::File::open(entry.path()).expect("Unable to open source file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read source file");

        // Iterate over syntax tree
        let syntax_tree =
            syn::parse_file(&contents).expect("Unable to parse source file");
        for item in syntax_tree.items {
            match item {
                syn::Item::Struct(item_struct) => {
                    node_names.push(item_struct.ident.clone());
                    node_structs.push(item_struct);
                }
                syn::Item::Enum(item_enum) => {
                    node_names.push(item_enum.ident.clone());
                    node_enums.push(item_enum);
                }
                _ => (),
            }
        }
    }

    // Emit new definitions for nodes
    let output_path =
        Path::new(&std::env::var("OUT_DIR").unwrap()).join("nodes.rs");
    let node_enum_def = quote! {
      #[derive(Clone)]
      pub enum Node<'a> {
        #( #node_names(&'a #node_names<'a>) ),*
      }

      impl<'a> Nodes<'a> for Node<'a> {
        fn nodes(&'a self) -> NodeIter<'a> {
            match self {
                #( Node::#node_names(inner_ref) => { inner_ref.nodes() } )*
            }
        }
      }
    };
    let node_defs = quote! {
        #(
            impl<'a> From<&'a #node_names<'a>> for Node<'a> {
                fn from(value: &'a #node_names) -> Self {
                    Node::#node_names(value)
                }
            }
        )*
    };
    let mut node_impls = TokenStream::default();
    for item_enum in node_enums {
        let ident = item_enum.ident;
        let variants = item_enum.variants.iter().map(|v| {
            let name = &v.ident;
            quote! {#name}
        });
        let variant_expansions = item_enum.variants.iter().map(|v| {
            let syn::Fields::Unnamed(ref unnamed_fields) = v.fields else {
                panic!(
                    "Syntax tree enum with named fields: {}",
                    ident.to_string()
                );
            };
            let expansion_string = unnamed_fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, _)| format!("v{}", i))
                .collect::<Vec<_>>()
                .join(", ");
            let res: TokenStream = expansion_string
                .parse()
                .expect("Unable to parse enum expansion");
            res
        });
        let variant_children = item_enum.variants.iter().map(|v| {
            let syn::Fields::Unnamed(ref unnamed_fields) = v.fields else {
                panic!(
                    "Syntax tree enum with named fields: {}",
                    ident.to_string()
                );
            };
            let expansion_string = unnamed_fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, _)| format!("v{}.nodes()", i))
                .collect::<Vec<_>>()
                .join(" + ");
            if expansion_string.is_empty() {
                quote!(NodeIter::default())
            } else {
                let res: TokenStream = expansion_string
                    .parse()
                    .expect("Unable to parse enum expansion");
                res
            }
        });
        node_impls.extend(quote! {
            impl<'a> Nodes<'a> for #ident<'a> {
                fn nodes(&'a self) -> NodeIter<'a> {
                    match self {
                        #( #ident::#variants(#variant_expansions) => { Into::<NodeIter<'a>>::into(Into::<Node<'a>>::into(self)) + #variant_children } )*
                    }
                }
            }
        })
    }
    for item_struct in node_structs {
        let ident = item_struct.ident;
        let syn::Fields::Unnamed(unnamed_fields) = item_struct.fields else {
            panic!(
                "Syntax tree struct with named fields: {}",
                ident.to_string()
            );
        };
        let indices =
            unnamed_fields.unnamed.iter().enumerate().map(|(i, _)| {
                let index = syn::Index::from(i);
                quote! {#index}
            });
        node_impls.extend(quote! {
            impl<'a> Nodes<'a> for #ident<'a> {
                fn nodes(&'a self) -> NodeIter<'a> {
                    Into::<NodeIter<'a>>::into(Into::<Node<'a>>::into(self)) +
                    #( self.#indices.nodes() )+*
                }
            }
        })
    }
    fs::write(
        &output_path,
        node_enum_def.to_string()
            + &node_defs.to_string()
            + &node_impls.to_string(),
    )
    .expect("Unable to write generated file");
}
