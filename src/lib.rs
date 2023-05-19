#![doc(html_root_url = "https://docs.rs/impl_sim/0.2.0")]
//! impl_sim auto implement callback functions for trait Sim
//!

use proc_macro::TokenStream;
use proc_macro2::TokenStream as PM2TS;
use proc_macro2::TokenTree;
// use proc_macro2::{TokenTree, Group, Ident, Literal, Punct};
// use proc_macro2::{Delimiter, Span, Spacing};
use quote::{quote, ToTokens}; // quote::ToTokens in proc_macro2
use syn; // syn::{parse_macro_input, ItemFn};

/// macro implement callback function
#[proc_macro]
pub fn impl_sim_fn(item: TokenStream) -> TokenStream {
  let ts: PM2TS = item.into();
  let tk = match ts.into_iter().next().unwrap() {
    TokenTree::Ident(t) => Some(t), // match only Ident
    _ => None // panic when arrive here
  };
  let f = &tk.unwrap();
  let s = &f.to_string();
  match s.as_str() {
    "draw_geom" => {
      quote! {
        /// #f
        fn #f(&self, geom: dGeomID,
          pos: Option<*const dReal>, rot: Option<*const dReal>, ws: i32) {
          ostatln!(concat!("called ", #s));
          self.super_get().#f(geom, pos, rot, ws);
        }
      }
    },
    "near_callback" => {
      quote! {
        /// #f
        fn #f(&mut self, o1: dGeomID, o2: dGeomID) {
          ostatln!(concat!("called ", #s));
          self.super_mut().#f(o1, o2);
        }
      }
    },
    "step_callback" => {
      quote! {
        /// #f
        fn #f(&mut self, pause: i32) {
          ostatln!(concat!("called ", #s));
          self.super_mut().#f(pause);
        }
      }
    },
    "command_callback" => {
      quote! {
        /// #f
        fn #f(&mut self, cmd: i32) {
          ostatln!(concat!("called ", #s));
          self.super_mut().#f(cmd);
        }
      }
    },
    _ => {
      quote! {
        /// #f
        fn #f(&mut self) {
          ostatln!(concat!("called ", #s));
          self.super_mut().#f();
        }
      }
    }
  }.into()
}

/// derive callback functions for trait Sim
#[proc_macro_attribute]
pub fn impl_sim_derive(attr: TokenStream, item: TokenStream) -> TokenStream {
  let mut ast = syn::parse_macro_input!(item as syn::ItemImpl);
  let ts: PM2TS = attr.into();
  for tt in ts {
    match tt {
      TokenTree::Ident(f) => { // match only Ident
        ast.items.push(syn::parse_quote! {
          impl_sim_fn!(#f);
        });
      },
      _ => {} // skip Punct ',' etc
    }
  }
  ast.into_token_stream().into()
}
