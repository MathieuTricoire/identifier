use super::ctxt::Ctxt;
use super::symbol::*;
use proc_macro2::{Group, Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{
    parse,
    parse::Parse,
    punctuated::Punctuated,
    token::Comma,
    Data, DeriveInput, Fields,
    Meta::{List, NameValue},
    NestedMeta::{Lit, Meta},
};

struct Attr<'c, T> {
    cx: &'c Ctxt,
    name: Symbol,
    tokens: TokenStream,
    value: Option<T>,
}

impl<'c, T> Attr<'c, T> {
    fn none(cx: &'c Ctxt, name: Symbol) -> Self {
        Attr {
            cx,
            name,
            tokens: TokenStream::new(),
            value: None,
        }
    }

    fn set<A: ToTokens>(&mut self, obj: A, value: T) {
        let tokens = obj.into_token_stream();

        if self.value.is_some() {
            self.cx.error_spanned_by(
                tokens,
                format!("duplicate identifier attribute `{}`", self.name),
            );
        } else {
            self.tokens = tokens;
            self.value = Some(value);
        }
    }

    fn get(self) -> Option<T> {
        self.value
    }

    fn is_none(&self) -> bool {
        self.value.is_none()
    }
}

pub struct Attrs {
    with: Option<syn::ExprPath>,
    field_type: Option<syn::Ident>,
    params: Option<Punctuated<syn::Expr, Comma>>,
}

const ERR_EXPECT_IDENTIFIER: &str = "expected #[identifier(with = \"mod\", ...)";

impl Attrs {
    pub fn get(cx: &Ctxt, input: &syn::DeriveInput) -> Attrs {
        let mut params = Attr::none(cx, PARAMS);
        let mut field_type = Attr::none(cx, FIELD_TYPE);
        let mut with = Attr::none(cx, WITH);

        if let Ok(ident) = get_ident_field_type(&cx, input) {
            field_type.set(ident, ident.clone());
        }

        let identifier_result = input
            .attrs
            .iter()
            .find(|attr| attr.path == IDENTIFIER)
            .ok_or_else(|| {
                cx.error_spanned_by(&input, ERR_EXPECT_IDENTIFIER);
            });

        let meta_items =
            identifier_result.map_or(Vec::new(), |identifier| get_meta_items(cx, identifier));

        for meta_item in meta_items {
            match &meta_item {
                // Parse `#[identifier(with = "expr_path")]`
                Meta(NameValue(m)) if m.path == WITH => {
                    if let Ok(w) = parse_lit_into_expr_path(cx, WITH, &m.lit) {
                        with.set(&m.path, w);
                    }
                }

                // Parse `#[identifier(params = "param1, param2")]`
                Meta(NameValue(m)) if m.path == PARAMS => {
                    if let Ok(p) = parse_lit_into_params(cx, PARAMS, &m.lit) {
                        params.set(&m.path, p);
                    }
                }

                Meta(meta_item) => {
                    let path = meta_item
                        .path()
                        .into_token_stream()
                        .to_string()
                        .replace(' ', "");
                    cx.error_spanned_by(
                        meta_item.path(),
                        format!("unknown identifier attribute `{}`", path),
                    );
                }

                Lit(lit) => {
                    cx.error_spanned_by(lit, "unexpected literal in identifier attribute");
                }
            }
        }

        if identifier_result.is_ok() && with.is_none() {
            cx.error_spanned_by(
                &identifier_result.unwrap().tokens,
                "The `with` attribute is required.",
            );
        }

        Attrs {
            field_type: field_type.get(),
            with: with.get(),
            params: params.get(),
        }
    }

    pub fn with(&self) -> Option<&syn::ExprPath> {
        self.with.as_ref()
    }

    pub fn params(&self) -> Option<&Punctuated<syn::Expr, Comma>> {
        self.params.as_ref()
    }

    pub fn field_type(&self) -> Option<&syn::Ident> {
        self.field_type.as_ref()
    }
}

fn get_ident_field_type<'a>(cx: &'a Ctxt, input: &'a DeriveInput) -> Result<&'a syn::Ident, ()> {
    const ERROR: &str =
        "Only TupleStruct with a single `u32`, `u64` or `u128` is supported, i.e. `struct Id(u128);`";
    match &input.data {
        Data::Struct(data_struct) => {
            if let Fields::Unnamed(fields) = &data_struct.fields {
                if fields.unnamed.len() == 1 {
                    let field = fields.unnamed.first().unwrap();
                    if let syn::Type::Path(ty_path) = &field.ty {
                        match ty_path.path.get_ident() {
                            Some(ident) if ident == "u128" || ident == "u64" || ident == "u32" => {
                                return Ok(ident);
                            }
                            _ => {
                                cx.error_spanned_by(
                                    &ty_path.path,
                                    "Only `u32`, `u64` or `u128` primitive type is supported.",
                                );
                                return Err(());
                            }
                        }
                    }
                }
            }
            cx.error_spanned_by(&data_struct.fields, ERROR);
        }
        Data::Enum(data_enum) => {
            cx.error_spanned_by(&data_enum.enum_token, ERROR);
        }
        Data::Union(data_union) => {
            cx.error_spanned_by(&data_union.union_token, ERROR);
        }
    };
    return Err(());
}

pub fn get_meta_items(cx: &Ctxt, attr: &syn::Attribute) -> Vec<syn::NestedMeta> {
    match attr.parse_meta() {
        Ok(List(meta)) => meta.nested.into_iter().collect(),
        Ok(other) => {
            cx.error_spanned_by(other, ERR_EXPECT_IDENTIFIER);
            Vec::new()
        }
        Err(err) => {
            cx.syn_error(err);
            Vec::new()
        }
    }
}

fn get_lit_str<'a>(cx: &Ctxt, attr_name: Symbol, lit: &'a syn::Lit) -> Result<&'a syn::LitStr, ()> {
    get_lit_str2(cx, attr_name, attr_name, lit)
}

fn get_lit_str2<'a>(
    cx: &Ctxt,
    attr_name: Symbol,
    meta_item_name: Symbol,
    lit: &'a syn::Lit,
) -> Result<&'a syn::LitStr, ()> {
    if let syn::Lit::Str(lit) = lit {
        Ok(lit)
    } else {
        cx.error_spanned_by(
            lit,
            format!(
                "expected identifier {} attribute to be a string: `{} = \"...\"`",
                attr_name, meta_item_name
            ),
        );
        Err(())
    }
}

fn parse_lit_into_expr_path(
    cx: &Ctxt,
    attr_name: Symbol,
    lit: &syn::Lit,
) -> Result<syn::ExprPath, ()> {
    let string = get_lit_str(cx, attr_name, lit)?;
    parse_lit_str(string).map_err(|_| {
        cx.error_spanned_by(lit, format!("failed to parse path: {:?}", string.value()))
    })
}

fn parse_lit_into_params(
    cx: &Ctxt,
    attr_name: Symbol,
    lit: &syn::Lit,
) -> Result<Punctuated<syn::Expr, Comma>, ()> {
    let string = get_lit_str(cx, attr_name, lit)?;
    return string
        .parse_with(Punctuated::<syn::Expr, Comma>::parse_terminated)
        .map_err(|_| {
            cx.error_spanned_by(lit, format!("failed to parse params: {:?}", string.value()))
        });
}

fn parse_lit_str<T>(s: &syn::LitStr) -> parse::Result<T>
where
    T: Parse,
{
    let tokens = spanned_tokens(s)?;
    syn::parse2(tokens)
}

fn spanned_tokens(s: &syn::LitStr) -> parse::Result<TokenStream> {
    let stream = syn::parse_str(&s.value())?;
    Ok(respan_token_stream(stream, s.span()))
}

fn respan_token_stream(stream: TokenStream, span: Span) -> TokenStream {
    stream
        .into_iter()
        .map(|token| respan_token_tree(token, span))
        .collect()
}

fn respan_token_tree(mut token: TokenTree, span: Span) -> TokenTree {
    if let TokenTree::Group(g) = &mut token {
        *g = Group::new(g.delimiter(), respan_token_stream(g.stream(), span));
    }
    token.set_span(span);
    token
}
