//! Pipeline statements with a placeholder
//!
//! see [`pipevine`]

use proc_macro::TokenStream;
use proc_macro2::{Group, Ident, Span, TokenTree};
use quote::quote;

/// Pipelining with placeholder
///
/// `<>` is a placeholder for result of previous statement.
///
/// ```
/// use pipevine::pipevine;
///
/// fn add(a: i32, b: i32) -> i32 {a + b}
/// fn mul(a: i32, b: i32) -> i32 {a * b}
/// fn double(x: i32) -> i32 {x * 2}
/// fn negate(x: i32) -> i32 {-x}
///
/// let result = pipevine!(5,
///                        add(1, <>),
///                        mul(<>, 3),
///                        double,
///                        add(10, mul(<>, 2)),
///                        |x| x + 4,
///                        <> + <> - 2,
///                        if <> % 2 == 0 { <> / 2 } else { <> * 3 + 1 },
///                        negate,
/// );
/// assert_eq!(-85, result);
/// ```
#[proc_macro]
pub fn pipevine(input: TokenStream) -> TokenStream {
    match expand(input.into()) {
        Ok(output) => output.into(),
        Err(message) => {
            let message = syn::LitStr::new(&message, Span::call_site());
            quote!(compile_error!(#message);).into()
        }
    }
}

fn expand(input: proc_macro2::TokenStream) -> Result<proc_macro2::TokenStream, String> {
    let parts = split_top_level_commas(input);

    if parts.is_empty() || parts[0].is_empty() {
        return Err("Pipevine! needs an initial value".into());
    }

    let initial: proc_macro2::TokenStream = parts[0].clone().into_iter().collect();

    let mut steps = Vec::new();
    for part in parts.iter().skip(1) {
        if part.is_empty() {
            continue;
        }

        let (step, has_placeholder) = replace_placeholders(part);
        steps.push((step, has_placeholder));
    }

    let mut output = quote! {
        let _____pipevine_____ = #initial;
    };

    for (step, has_placeholder) in steps {
        let step: proc_macro2::TokenStream = step.into_iter().collect();

        if has_placeholder {
            output.extend(quote! {
                let _____pipevine_____ = { #step };
            });
        } else {
            output.extend(quote! {
                let _____pipevine_____ = (#step)(_____pipevine_____);
            });
        }
    }

    Ok(quote!({
        #output
        _____pipevine_____
    }))
}

fn split_top_level_commas(
    input: proc_macro2::TokenStream,
) -> Vec<Vec<TokenTree>> {
    let mut parts = vec![Vec::new()];

    for token in input {
        if matches!(&token, TokenTree::Punct(p) if p.as_char() == ',') {
            parts.push(Vec::new());
        } else {
            parts.last_mut().unwrap().push(token);
        }
    }

    parts
}

fn replace_placeholders(tokens: &[TokenTree]) -> (Vec<TokenTree>, bool) {
    let mut output = Vec::new();
    let mut found = false;
    let mut i = 0;

    while i < tokens.len() {
        // Recognize <> as two adjacent punctuation tokens.
        if i + 1 < tokens.len()
            && matches!(&tokens[i], TokenTree::Punct(p) if p.as_char() == '<')
            && matches!(&tokens[i + 1], TokenTree::Punct(p) if p.as_char() == '>')
        {
            output.push(TokenTree::Ident(Ident::new(
                "_____pipevine_____",
                Span::call_site(),
            )));
            found = true;
            i += 2;
            continue;
        }

        match &tokens[i] {
            TokenTree::Group(group) => {
                let (inner, inner_found) =
                    replace_placeholders(&group.stream().into_iter().collect::<Vec<_>>());

                let mut new_group = Group::new(
                    group.delimiter(),
                    inner.into_iter().collect(),
                );
                new_group.set_span(group.span());
                output.push(TokenTree::Group(new_group));
                found |= inner_found;
            }
            other => output.push(other.clone()),
        }

        i += 1;
    }

    (output, found)
}
