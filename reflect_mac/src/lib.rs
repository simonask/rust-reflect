#![feature(plugin_registrar, phase, quote, macro_rules)]

extern crate rustc;
extern crate syntax;
extern crate phf;
extern crate phf_mac;

use syntax::ext::base::{ExtCtxt};
use syntax::codemap::{Span};
use syntax::ast;
use syntax::ast::{ItemStruct, StructField};
use syntax::ptr::P;
use syntax::parse::token;

fn generate_attributes_map(
  c: &mut ExtCtxt,
  s: Span,
  _: &ast::MetaItem,
  _: &ast::Item,
  fields: &Vec<StructField>
) -> P<ast::Item> {
  use syntax::ext::build::AstBuilder;

  let entries = fields.iter().map(|ref field| {
    let ident = field.node.ident().unwrap();
    let key_contents = phf_mac::util::Key::Str(token::get_ident(ident.clone()));
    let fn_name = c.ident_of(format!("attr_{}", ident.as_str()).as_slice());
    phf_mac::util::Entry {
      key: c.expr_lit(s, ast::LitStr(token::get_ident(ident.clone()), ast::StrStyle::CookedStr)),
      key_contents: key_contents,
      value: quote_expr!(c, $fn_name as fn()-> &'static ::reflect::AnyAttribute)
    }
  }).collect::<Vec<_>>();

  let state = phf_mac::util::generate_hash(c, s, entries.as_slice());
  let created_map = phf_mac::util::create_map(c, s, entries, state);

  // Now we have the equivalent of what phf_map! does, but we don't
  // want to force the user to add "extern crate phf" to their build,
  // so replace ::phf::Map with ::reflect::AttributeMap (which is the same).
  let map_expr = created_map.make_expr().unwrap();
  let new_map_expr = match map_expr.node {
    ast::ExprStruct(_, ref fields, _) => {
      let new_path = c.path_global(s, vec!(c.ident_of("reflect"), c.ident_of("AttributeMap")));
      c.expr_struct(s, new_path, fields.clone())
    },
    _ => c.span_bug(s, "phf_mac::util::create_map did not return an expression to initialize a map.")
  };

  let attributes_initializer = P(new_map_expr);

  quote_item!(c, static ATTRIBUTES: ::reflect::AttributeMap = $attributes_initializer;).unwrap()
}

fn generate_attribute_info_getter(
  c: &mut ExtCtxt,
  s: Span,
  _: &ast::MetaItem,
  struct_item: &ast::Item,
  field: &StructField
) -> P<ast::Item> {
  use syntax::ext::build::AstBuilder;

  let ident = match field.node.ident() {
    Some(i) => i,
    None => c.span_bug(s, format!("unnamed field in normal struct").as_slice())
  };

  let field_ty = field.node.ty.clone();
  let self_ty = struct_item.ident;
  let fn_name = c.ident_of(format!("attr_{}", ident.as_str()).as_slice());

  quote_item!(c,
    fn $fn_name() -> &'static ::reflect::AnyAttribute {
      struct Attr;
      static ATTR: Attr = Attr;

      impl ::reflect::Attribute<$self_ty, $field_ty> for Attr {
        fn get(&self, instance: &$self_ty) -> ::reflect::AttrResult<$field_ty> {
          Ok(instance.$ident.clone())
        }
        fn set(&self, instance: &mut $self_ty, new_value: $field_ty) -> ::reflect::AttrResult<()> {
          instance.$ident = new_value;
          Ok(())
        }
      }

      &ATTR as &::reflect::AnyAttribute
    }
  ).unwrap()
}

fn generate_attributes_info_getters(
  c: &mut ExtCtxt,
  s: Span,
  meta_item: &ast::MetaItem,
  struct_item: &ast::Item,
  fields: &Vec<StructField>
) -> Vec<P<ast::Stmt>> {
  use syntax::ext::build::AstBuilder;

  fields.iter().map(|field| {
    let fn_item = generate_attribute_info_getter(c, s, meta_item, struct_item, field);
    c.stmt_item(s, fn_item)
  }).collect::<Vec<_>>()
}

fn generate_get_type_info_fn_impl(
  c: &mut ExtCtxt,
  s: Span,
  meta_item: &ast::MetaItem,
  struct_item: &ast::Item,
) -> P<ast::Block> {
  use syntax::ext::build::AstBuilder;

  // TODO: Get fully qualified name with all modules etc.
  let self_name = token::get_ident(struct_item.ident);

  let fields = match struct_item.node {
    ItemStruct(ref struct_def, _) => &struct_def.fields,
    _ => c.span_bug(s, format!("Expected struct, got {}", struct_item).as_slice())
  };

  let mut stmts = generate_attributes_info_getters(c, s, meta_item, struct_item, fields);
  let generated_attr_map = generate_attributes_map(c, s, meta_item, struct_item, fields);
  stmts.push(c.stmt_item(s, generated_attr_map));

  let name_expr = c.expr_str(s, self_name);
  let type_info_initializer = quote_expr!(c, ::reflect::TypeInfo {
    name: $name_expr,
    attributes: &ATTRIBUTES,
  });

  let type_info_decl = quote_item!(c, static TYPE_INFO: ::reflect::TypeInfo = $type_info_initializer;).unwrap();
  stmts.push(c.stmt_item(s, type_info_decl));
  let retval = quote_expr!(c, ::reflect::TypeInfoFor(&TYPE_INFO));

  c.block(s, stmts, Some(retval))
}

fn generate_get_type_info_impl_for_struct(
  c: &mut ExtCtxt,
  s: Span,
  meta_item: &ast::MetaItem,
  struct_item: &ast::Item,
) -> P<ast::Item> {
  let ty = struct_item.ident;
  let type_info_for_impl = generate_get_type_info_fn_impl(c, s, meta_item, struct_item);

  quote_item!(c,
    impl ::reflect::GetTypeInfo for $ty {
      fn get_type_info(_: Option<$ty>) -> ::reflect::TypeInfoFor<$ty>
        $type_info_for_impl
    }
  ).unwrap()
}

pub fn reflect_on_struct(context: &mut ExtCtxt, span: Span, meta_item: &ast::MetaItem, item: &ast::Item, push: |P<ast::Item>|) {
  use syntax::ast::{Item_};
  match &item.node {
    &Item_::ItemStruct(_, _) => {
      push(generate_get_type_info_impl_for_struct(context, span, meta_item, item))
    },
    _ => {
      context.span_bug(span, "reflect_on_struct called with non-struct argument.");
    }
  };
}

fn expand_reflect(context: &mut ExtCtxt, span: Span, meta_item: &ast::MetaItem, item: &ast::Item, push: |P<ast::Item>| ) {
  use syntax::ast::{Item_};
  match &item.node {
    &Item_::ItemStruct(_, _) => {
      reflect_on_struct(context, span, meta_item, item, push);
    },
    _ => {
      context.span_err(span, "#[reflect] attribute can only be used on structs and enums.")
    }
  };
}

#[plugin_registrar]
pub fn registrar(reg: &mut rustc::plugin::Registry) {
  use syntax::ext::base;
  use syntax::parse::token::intern;

  let interned_name = intern("reflect");
  let decorator = base::SyntaxExtension::Decorator(box expand_reflect);
  reg.register_syntax_extension(interned_name, decorator);
}
