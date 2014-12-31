#![feature(plugin_registrar, phase, quote, macro_rules)]

extern crate reflect;
extern crate rustc;
extern crate syntax;
extern crate phf;
extern crate phf_mac;

use syntax::ext::base::{ExtCtxt};
use syntax::codemap::{Span};
use syntax::ast;
use syntax::ast::{ItemStruct, StructField};
use syntax::ptr::P;
use syntax::ext::deriving::generic::{Substructure};
use syntax::parse::token;

fn generate_attributes_map(
  c: &mut ExtCtxt,
  s: Span,
  _: &ast::MetaItem,
  _: &ast::Item,
  _: &Substructure,
  fields: &Vec<StructField>
) -> P<ast::Item> {
  use syntax::ext::build::AstBuilder;

  let entries = fields.iter().map(|ref field| {
    let ident = field.node.ident().unwrap();
    let key = phf_mac::util::Key::Str(token::get_ident(ident.clone()));
    let key_contents = key.clone();
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
  _: &ast::Item,
  substructure: &Substructure,
  field: &StructField
) -> P<ast::Item> {
  use syntax::ext::build::AstBuilder;

  let ident = match field.node.ident() {
    Some(i) => i,
    None => c.span_bug(s, format!("unnamed field in normal struct").as_slice())
  };

  let attr_struct_def = quote_item!(c, struct Attr;).unwrap();
  let attr_const_decl = quote_item!(c, static ATTR: Attr = Attr;).unwrap();

  let field_ty = field.node.ty.clone();
  let self_ty = substructure.type_ident;

  let attr_impl = quote_item!(c,
    impl ::reflect::Attribute<$self_ty, $field_ty> for Attr {
      fn get_(&self, instance: &$self_ty) -> ::reflect::AttrResult<$field_ty> {
        Ok(instance.$ident.clone())
      }
      fn set_(&self, instance: &mut $self_ty, new_value: $field_ty) -> ::reflect::AttrResult<()> {
        instance.$ident = new_value;
        Ok(())
      }
    }
  ).unwrap();

  let stmts = vec!(
    c.stmt_item(s, attr_struct_def),
    c.stmt_item(s, attr_const_decl),
    c.stmt_item(s, attr_impl),
  );

  let return_type = quote_ty!(c, &'static ::reflect::AnyAttribute);
  let return_value = quote_expr!(c, &ATTR as &::reflect::AnyAttribute);
  let fn_body = c.block(s, stmts, Some(return_value));
  let fn_name = c.ident_of(format!("attr_{}", ident.as_str()).as_slice());
  c.item_fn(s, fn_name, Vec::new(), return_type, fn_body)
}

fn generate_attributes_info_getters(
  c: &mut ExtCtxt,
  s: Span,
  meta_item: &ast::MetaItem,
  struct_item: &ast::Item,
  substructure: &Substructure,
  fields: &Vec<StructField>
) -> Vec<P<ast::Stmt>> {
  use syntax::ext::build::AstBuilder;

  fields.iter().map(|field| {
    let fn_item = generate_attribute_info_getter(c, s, meta_item, struct_item, substructure, field);
    c.stmt_item(s, fn_item)
  }).collect::<Vec<_>>()
}

fn generate_static_type_info_impl(
  c: &mut ExtCtxt,
  s: Span,
  meta_item: &ast::MetaItem,
  struct_item: &ast::Item,
  substructure: &Substructure
) -> P<ast::Expr> {
  use syntax::ext::build::AstBuilder;

  let self_name = token::get_ident(substructure.type_ident);

  let fields = match struct_item.node {
    ItemStruct(ref struct_def, _) => &struct_def.fields,
    _ => c.span_bug(s, format!("Expected struct, got {}", struct_item).as_slice())
  };

  let attribute_getters = generate_attributes_info_getters(c, s, meta_item, struct_item, substructure, fields);

  let attributes_decl = generate_attributes_map(c, s, meta_item, struct_item, substructure, fields);

  let name_expr = c.expr_str(s, self_name);

  let type_info_initializer = quote_expr!(c, ::reflect::TypeInfo {
    name: $name_expr,
    attributes: &ATTRIBUTES,
  });

  let type_info_decl = quote_item!(c, static TYPE_INFO: ::reflect::TypeInfo = $type_info_initializer;).unwrap();

  let mut stmts = vec!(
    c.stmt_item(s, attributes_decl),
    c.stmt_item(s, type_info_decl)
  );
  stmts.push_all(attribute_getters.as_slice());

  c.expr_block(c.block(s, stmts, Some(quote_expr!(c, ::reflect::StaticTypeInfo(&TYPE_INFO)))))
}

fn generate_reflect_static_impl_for_struct<F>(
  ctx: &mut ExtCtxt,
  span: Span,
  meta_item: &ast::MetaItem,
  struct_item: &ast::Item,
  push: F
) where F: FnOnce(P<ast::Item>) {
  use syntax::ext::deriving::generic::{TraitDef, MethodDef, combine_substructure};
  use syntax::ext::deriving::generic::ty::{Ty, Path, LifetimeBounds};

  let path = Path::new_local(struct_item.ident.as_str());

  let trait_def = TraitDef {
    span: span,
    attributes: Vec::new(),
    path: Path::new(vec!("reflect", "ReflectStatic")),
    additional_bounds: Vec::new(),
    generics: LifetimeBounds::empty(),
    methods: vec!(
      MethodDef {
        name: "static_type_info",
        generics: LifetimeBounds::empty(),
        explicit_self: None,
        args: vec!(Ty::Literal(Path::new_(vec!("std", "option", "Option"), None, vec!(box Ty::Literal(path.clone())), false))),
        ret_ty: Ty::Literal(Path::new_(vec!("reflect", "StaticTypeInfo"), None, vec!(box Ty::Literal(path.clone())), false)),
        attributes: Vec::new(),
        combine_substructure: combine_substructure(|c, s, sub| {
          generate_static_type_info_impl(c, s, meta_item, struct_item, sub)
        }),
      }
    )
  };

  trait_def.expand(ctx, meta_item, struct_item, push);
}

fn reflect_expand(context: &mut ExtCtxt, span: Span, meta_item: &ast::MetaItem, item: &ast::Item, push: |P<ast::Item>| ) {
  use syntax::ast::{Item_};

  match &item.node {
    &Item_::ItemStruct(_, _) => {
      generate_reflect_static_impl_for_struct(context, span, meta_item, item, |i| push(i));
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
  let decorator = base::SyntaxExtension::Decorator(box reflect_expand);
  reg.register_syntax_extension(interned_name, decorator);
}
