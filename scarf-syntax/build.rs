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
      #[derive(Debug, Clone)]
      /// A reference to a data structure in a SystemVerilog CST
      pub enum Node<'a: 'b, 'b> {
        #( #node_names(&'b #node_names<'a>) ),*
      }

      impl<'a: 'b, 'b> Nodes<'a, 'b> for Node<'a, 'b> {
          fn nodes(&'b self) -> NodeIter<'a, 'b> {
              self.iter()
          }
          fn add_nodes(&'b self, dest: &mut Vec<Node<'a, 'b>>, pred: fn(Node<'a, 'b>) -> bool)
          {
            match self {
                #( Node::#node_names(inner_ref) => { inner_ref.add_nodes(dest, pred) } )*
            }
          }
      }

      impl<'a: 'b, 'b> IntoIterator for Node<'a, 'b> {
          type Item = Node<'a, 'b>;
          type IntoIter = NodeIter<'a, 'b>;
          fn into_iter(self) -> Self::IntoIter {
              self.into()
          }
      }

      impl<'a: 'b, 'b> Node<'a, 'b> {
          /// Iterate over the current [`Node`] and all child [`Node`]s
          pub fn iter(&self) -> NodeIter<'a, 'b> {
              self.clone().into()
          }
          /// The child nodes of a given [`Node`]
          fn children(&self) -> Vec<Node<'a, 'b>> {
              match self {
                  #( Node::#node_names(inner_ref) => { inner_ref.children() } )*
              }
          }
          /// The name of the data structure the [`Node`] references (as well
          /// as the name of the [`Node`] variant)
          ///
          /// ```rust
          /// # use scarf_syntax::*;
          /// let identifier = Identifier::SimpleIdentifier((
          ///     "my_signal",
          ///     Metadata::default()
          /// ));
          /// let node: Node<'_, '_> = (&identifier).into();
          /// assert_eq!(node.name(), "Identifier");
          /// ```
          pub fn name(&self) -> &str {
              match self {
                  #( Node::#node_names(_) => { stringify!(#node_names) } )*
              }
          }
      }
    };
    let iter_doc = node_names.iter().map(|ident| format!{"Iterate across the [`{}`] and its children", ident.to_string()});
    let node_defs = quote! {
        #(
            impl<'a: 'b, 'b> From<&'b #node_names<'a>> for Node<'a, 'b> {
                fn from(value: &'b #node_names<'a>) -> Self {
                    Node::#node_names(value)
                }
            }
            impl<'a: 'b, 'b> IntoIterator for &'b #node_names<'a> {
                type Item = Node<'a, 'b>;
                type IntoIter = NodeIter<'a, 'b>;
                fn into_iter(self) -> Self::IntoIter {
                    self.iter()
                }
            }

            impl<'a: 'b, 'b> #node_names<'a> {
                #[doc = #iter_doc]
                pub fn iter(&'b self) -> NodeIter<'a, 'b> {
                    Into::<Node<'a, 'b>>::into(self).iter()
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
        let variant_add_nodes = item_enum.variants.iter().map(|v| {
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
                .map(|(i, _)| format!("v{}.add_nodes(dest, pred)", i))
                .collect::<Vec<_>>()
                .join("; ");
            if expansion_string.is_empty() {
                quote!(NodeIter::default())
            } else {
                let res: TokenStream = expansion_string
                    .parse()
                    .expect("Unable to parse enum expansion");
                res
            }
        });
        let variants_clone = variants.clone();
        let variant_expansions_clone = variant_expansions.clone();
        node_impls.extend(quote! {
            impl<'a: 'b, 'b> Nodes<'a, 'b> for #ident<'a> {
                fn nodes(&'b self) -> NodeIter<'a, 'b> {
                    Into::<NodeIter<'a, 'b>>::into(Into::<Node<'a, 'b>>::into(self))
                }
                fn add_nodes(&'b self, dest: &mut Vec<Node<'a, 'b>>, pred: fn(Node<'a, 'b>) -> bool)
                {
                    if pred(Into::<Node<'a, 'b>>::into(self)) {
                        dest.push(Into::<Node<'a, 'b>>::into(self))
                    }
                    match self {
                        #( #ident::#variants(#variant_expansions) => { #variant_add_nodes; } )*
                    }
                }
            }
            impl<'a: 'b, 'b> #ident<'a> {
                fn children(&'b self) -> Vec<Node<'a, 'b>> {
                    match self {
                        #( #ident::#variants_clone(#variant_expansions_clone) => { (#variant_children).raw() } )*
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
        let indices_clone = indices.clone();
        node_impls.extend(quote! {
            impl<'a: 'b, 'b> Nodes<'a, 'b> for #ident<'a> {
                fn nodes(&'b self) -> NodeIter<'a, 'b> {
                    Into::<NodeIter<'a, 'b>>::into(Into::<Node<'a, 'b>>::into(self))
                }
                fn add_nodes(&'b self, dest: &mut Vec<Node<'a, 'b>>, pred: fn(Node<'a, 'b>) -> bool)
                {
                    if pred(Into::<Node<'a, 'b>>::into(self)) {
                        dest.push(Into::<Node<'a, 'b>>::into(self))
                    }
                    #( self.#indices.add_nodes(dest, pred); )*
                }
            }
            impl<'a: 'b, 'b> #ident<'a> {
                fn children(&'b self) -> Vec<Node<'a, 'b>> {
                    (#( self.#indices_clone.nodes() )+*).raw()
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
