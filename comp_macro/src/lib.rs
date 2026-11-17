use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

/// Generates a lazy iterator comprehension.
/// Example: `comp![x * 2 for x in 0..10 if x % 2 == 0]`
#[proc_macro]
pub fn comp(input: TokenStream) -> TokenStream {
    let comprehension = parse_macro_input!(input as Comp);
    quote! { #comprehension }.into()
}

/// Generates a `Vec` comprehension by collecting the iterator.
/// Example: `comp_vec![(x, y) for x in 0..3 for y in 0..x]`
#[proc_macro]
pub fn comp_vec(input: TokenStream) -> TokenStream {
    let comprehension = parse_macro_input!(input as Comp);
    quote! { (#comprehension).collect::<Vec<_>>() }.into()
}

pub(crate) struct Comp {
    mapping: Mapping,
    for_if_clauses: Vec<ForIfClause>,
}

impl Parse for Comp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mapping = input.parse()?;
        let mut for_if_clauses = Vec::new();

        // Require at least one `for` clause
        for_if_clauses.push(input.parse()?);

        // Parse any additional nested `for` clauses
        while input.peek(Token![for]) {
            for_if_clauses.push(input.parse()?);
        }

        Ok(Self {
            mapping,
            for_if_clauses,
        })
    }
}

impl ToTokens for Comp {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let mapping = &self.mapping;
        let mut clauses = self.for_if_clauses.iter().rev();

        let innermost = clauses
            .next()
            .expect("Comp requires at least one ForIfClause");
        let ForIfClause {
            pattern,
            expression,
            conditions,
        } = innermost;

        let mut output = quote! {
            core::iter::IntoIterator::into_iter(#expression).filter_map(move |#pattern| {
                (true #(&& (#conditions))*).then(|| #mapping)
            })
        };

        for clause in clauses {
            let ForIfClause {
                pattern,
                expression,
                conditions,
            } = clause;

            output = quote! {
                core::iter::IntoIterator::into_iter(#expression).flat_map(move |#pattern| {
                    (true #(&& (#conditions))*)
                        .then(|| #output)
                        .into_iter()
                        .flatten()
                })
            };
        }

        tokens.extend(output);
    }
}

pub(crate) struct Mapping(syn::Expr);

impl Parse for Mapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl ToTokens for Mapping {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

pub(crate) struct ForIfClause {
    pattern: Pattern,
    expression: syn::Expr,
    conditions: Vec<Condition>,
}

impl Parse for ForIfClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Token![for] = input.parse()?;
        let pattern = input.parse()?;
        let _: Token![in] = input.parse()?;
        let expression = input.parse()?;

        let mut conditions = Vec::new();
        while input.peek(Token![if]) {
            conditions.push(input.parse()?);
        }

        Ok(Self {
            pattern,
            expression,
            conditions,
        })
    }
}

pub(crate) struct Pattern(syn::Pat);

impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        syn::Pat::parse_single(input).map(Self)
    }
}

impl ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

pub(crate) struct Condition(syn::Expr);

impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Token![if] = input.parse()?;
        input.parse().map(Self)
    }
}

impl ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

