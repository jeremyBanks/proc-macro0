//! Conversions between proc-macro0 and proc-macro2 types.
//!
//! This module is only available when the `proc-macro2` feature is enabled.
//!
//! # What's Preserved vs Lost
//!
//! ## Converting proc-macro2 → proc-macro0 via `From` (re-parsing)
//!
//! **Preserved:**
//! - All token content (identifiers, literals, punctuation)
//! - Token structure (groups, spacing)
//! - With `span-locations`: valid line/column positions in proc-macro0's source map
//!
//! **Lost:**
//! - `Delimiter::None` groups (invisible delimiters have no string representation)
//! - Original span locations from proc-macro2
//! - Negative literals become two tokens (`-` and positive literal)
//!
//! ## Converting proc-macro0 → proc-macro2 via `From`
//!
//! **Preserved:**
//! - All token content and structure (including `Delimiter::None`)
//!
//! **Lost:**
//! - All span information (becomes `Span::call_site()`)
//!
//! # Choosing a Conversion Method
//!
//! - Use `From<proc_macro2::TokenStream>` when you need valid span positions
//!   in proc-macro0 and don't care about `Delimiter::None` groups.
//! - Use [`TokenStream::from_proc_macro2_structural`] when you need exact
//!   structural fidelity (including `Delimiter::None`) but don't need spans.

use crate::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

// ============================================================================
// Span conversions (lossy - we can only use call_site)
// ============================================================================

impl From<proc_macro2::Span> for Span {
    /// Converts a proc-macro2 Span to proc-macro0.
    ///
    /// **Warning:** The actual source location is lost. Returns `Span::call_site()`.
    fn from(_: proc_macro2::Span) -> Self {
        Span::call_site()
    }
}

impl From<Span> for proc_macro2::Span {
    /// Converts a proc-macro0 Span to proc-macro2.
    ///
    /// **Warning:** The actual source location is lost. Returns `Span::call_site()`.
    fn from(_: Span) -> Self {
        proc_macro2::Span::call_site()
    }
}

// ============================================================================
// Delimiter conversions (lossless)
// ============================================================================

impl From<proc_macro2::Delimiter> for Delimiter {
    fn from(d: proc_macro2::Delimiter) -> Self {
        match d {
            proc_macro2::Delimiter::Parenthesis => Delimiter::Parenthesis,
            proc_macro2::Delimiter::Brace => Delimiter::Brace,
            proc_macro2::Delimiter::Bracket => Delimiter::Bracket,
            proc_macro2::Delimiter::None => Delimiter::None,
        }
    }
}

impl From<Delimiter> for proc_macro2::Delimiter {
    fn from(d: Delimiter) -> Self {
        match d {
            Delimiter::Parenthesis => proc_macro2::Delimiter::Parenthesis,
            Delimiter::Brace => proc_macro2::Delimiter::Brace,
            Delimiter::Bracket => proc_macro2::Delimiter::Bracket,
            Delimiter::None => proc_macro2::Delimiter::None,
        }
    }
}

// ============================================================================
// Spacing conversions (lossless)
// ============================================================================

impl From<proc_macro2::Spacing> for Spacing {
    fn from(s: proc_macro2::Spacing) -> Self {
        match s {
            proc_macro2::Spacing::Alone => Spacing::Alone,
            proc_macro2::Spacing::Joint => Spacing::Joint,
        }
    }
}

impl From<Spacing> for proc_macro2::Spacing {
    fn from(s: Spacing) -> Self {
        match s {
            Spacing::Alone => proc_macro2::Spacing::Alone,
            Spacing::Joint => proc_macro2::Spacing::Joint,
        }
    }
}

// ============================================================================
// Ident conversions (span is lost)
// ============================================================================

impl From<proc_macro2::Ident> for Ident {
    fn from(ident: proc_macro2::Ident) -> Self {
        Ident::new(&ident.to_string(), Span::call_site())
    }
}

impl From<Ident> for proc_macro2::Ident {
    fn from(ident: Ident) -> Self {
        proc_macro2::Ident::new(&ident.to_string(), proc_macro2::Span::call_site())
    }
}

// ============================================================================
// Punct conversions (span is lost)
// ============================================================================

impl From<proc_macro2::Punct> for Punct {
    fn from(punct: proc_macro2::Punct) -> Self {
        let mut p = Punct::new(punct.as_char(), punct.spacing().into());
        p.set_span(Span::call_site());
        p
    }
}

impl From<Punct> for proc_macro2::Punct {
    fn from(punct: Punct) -> Self {
        let mut p = proc_macro2::Punct::new(punct.as_char(), punct.spacing().into());
        p.set_span(proc_macro2::Span::call_site());
        p
    }
}

// ============================================================================
// Literal conversions (span is lost)
// ============================================================================

impl From<proc_macro2::Literal> for Literal {
    fn from(lit: proc_macro2::Literal) -> Self {
        // Parse the literal's string representation
        lit.to_string()
            .parse::<Literal>()
            .expect("valid proc-macro2 literal should parse as proc-macro0 literal")
    }
}

impl From<Literal> for proc_macro2::Literal {
    fn from(lit: Literal) -> Self {
        // Parse the literal's string representation
        lit.to_string()
            .parse::<proc_macro2::Literal>()
            .expect("valid proc-macro0 literal should parse as proc-macro2 literal")
    }
}

// ============================================================================
// Group conversions (span is lost)
// ============================================================================

impl From<proc_macro2::Group> for Group {
    fn from(group: proc_macro2::Group) -> Self {
        let mut g = Group::new(group.delimiter().into(), group.stream().into());
        g.set_span(Span::call_site());
        g
    }
}

impl From<Group> for proc_macro2::Group {
    fn from(group: Group) -> Self {
        let mut g = proc_macro2::Group::new(group.delimiter().into(), group.stream().into());
        g.set_span(proc_macro2::Span::call_site());
        g
    }
}

// ============================================================================
// TokenTree conversions (span is lost)
// ============================================================================

impl From<proc_macro2::TokenTree> for TokenTree {
    fn from(tt: proc_macro2::TokenTree) -> Self {
        match tt {
            proc_macro2::TokenTree::Group(g) => TokenTree::Group(g.into()),
            proc_macro2::TokenTree::Ident(i) => TokenTree::Ident(i.into()),
            proc_macro2::TokenTree::Punct(p) => TokenTree::Punct(p.into()),
            proc_macro2::TokenTree::Literal(l) => TokenTree::Literal(l.into()),
        }
    }
}

impl From<TokenTree> for proc_macro2::TokenTree {
    fn from(tt: TokenTree) -> Self {
        match tt {
            TokenTree::Group(g) => proc_macro2::TokenTree::Group(g.into()),
            TokenTree::Ident(i) => proc_macro2::TokenTree::Ident(i.into()),
            TokenTree::Punct(p) => proc_macro2::TokenTree::Punct(p.into()),
            TokenTree::Literal(l) => proc_macro2::TokenTree::Literal(l.into()),
        }
    }
}

// ============================================================================
// TokenStream conversions
// ============================================================================

impl From<proc_macro2::TokenStream> for TokenStream {
    /// Converts a proc-macro2 TokenStream to proc-macro0 by re-parsing.
    ///
    /// The resulting spans point to valid positions in proc-macro0's source map
    /// (registered from the string representation). However:
    /// - These are *new* positions, not the original proc-macro2 locations
    /// - `Delimiter::None` groups may not survive the string roundtrip
    ///
    /// Use [`TokenStream::from_proc_macro2_structural`] if you need to preserve
    /// `Delimiter::None` groups exactly (at the cost of losing all span info).
    fn from(stream: proc_macro2::TokenStream) -> Self {
        // Re-parse through our lexer to register source and create valid spans.
        // This gives meaningful span positions in our source map.
        stream
            .to_string()
            .parse()
            .expect("valid proc-macro2 TokenStream should parse as proc-macro0")
    }
}

impl TokenStream {
    /// Converts a proc-macro2 TokenStream structurally, preserving `Delimiter::None`.
    ///
    /// Unlike the `From` implementation which re-parses (and may lose
    /// `Delimiter::None` groups), this method preserves the exact token tree
    /// structure. However, all spans become `Span::call_site()`.
    ///
    /// Use this when exact structural fidelity matters more than span information.
    #[cfg(feature = "proc-macro2")]
    pub fn from_proc_macro2_structural(stream: proc_macro2::TokenStream) -> Self {
        stream.into_iter().map(TokenTree::from).collect()
    }
}

impl From<TokenStream> for proc_macro2::TokenStream {
    /// Converts a proc-macro0 TokenStream to proc-macro2.
    ///
    /// All spans become `Span::call_site()` since proc-macro0 cannot inject
    /// spans into proc-macro2's source map.
    fn from(stream: TokenStream) -> Self {
        stream.into_iter().map(proc_macro2::TokenTree::from).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_stream_roundtrip() {
        let source = "fn foo(x: i32) -> bool { x > 0 }";

        // Parse as proc-macro0
        let pm0: TokenStream = source.parse().unwrap();

        // Convert to proc-macro2
        let pm2: proc_macro2::TokenStream = pm0.clone().into();

        // Convert back to proc-macro0
        let pm0_back: TokenStream = pm2.into();

        // The string representations should match (ignoring spans)
        assert_eq!(pm0.to_string(), pm0_back.to_string());
    }

    #[test]
    fn test_structural_conversion() {
        let source = "fn foo() {}";
        let pm2: proc_macro2::TokenStream = source.parse().unwrap();

        // Both conversions should produce the same string output
        let from_impl: TokenStream = pm2.clone().into();
        let structural = TokenStream::from_proc_macro2_structural(pm2);

        assert_eq!(from_impl.to_string(), structural.to_string());
    }

    #[test]
    #[cfg(feature = "span-locations")]
    fn test_span_locations_preserved_from_reparse() {
        let source = "fn foo() {}";
        let pm2: proc_macro2::TokenStream = source.parse().unwrap();

        // Convert using From (which re-parses)
        let pm0: TokenStream = pm2.into();

        // Get the first token (should be "fn")
        let first_token = pm0.into_iter().next().unwrap();
        let span = first_token.span();

        // With span-locations, we should have valid line/column info
        // from the re-parsed source (not just zeros)
        let start = span.start();

        // The re-parsed span should point to line 1 (1-indexed)
        assert_eq!(start.line, 1);
        assert_eq!(start.column, 0);
    }

    #[test]
    fn test_delimiter_roundtrip() {
        for d in [
            Delimiter::Parenthesis,
            Delimiter::Brace,
            Delimiter::Bracket,
            Delimiter::None,
        ] {
            let pm2: proc_macro2::Delimiter = d.into();
            let back: Delimiter = pm2.into();
            assert_eq!(d, back);
        }
    }

    #[test]
    fn test_spacing_roundtrip() {
        for s in [Spacing::Alone, Spacing::Joint] {
            let pm2: proc_macro2::Spacing = s.into();
            let back: Spacing = pm2.into();
            assert_eq!(s, back);
        }
    }

    #[test]
    fn test_ident_conversion() {
        let pm0 = Ident::new("my_ident", Span::call_site());
        let pm2: proc_macro2::Ident = pm0.clone().into();
        let back: Ident = pm2.into();
        assert_eq!(pm0.to_string(), back.to_string());
    }

    #[test]
    fn test_punct_conversion() {
        let pm0 = Punct::new('+', Spacing::Alone);
        let pm2: proc_macro2::Punct = pm0.clone().into();
        let back: Punct = pm2.into();
        assert_eq!(pm0.as_char(), back.as_char());
        assert_eq!(pm0.spacing(), back.spacing());
    }

    #[test]
    fn test_literal_conversion() {
        // Test various literal types
        let literals = [
            Literal::u32_suffixed(42),
            Literal::string("hello"),
            Literal::character('x'),
            Literal::f64_unsuffixed(3.14),
        ];

        for lit in literals {
            let pm2: proc_macro2::Literal = lit.clone().into();
            let back: Literal = pm2.into();
            assert_eq!(lit.to_string(), back.to_string());
        }
    }

    #[test]
    fn test_group_conversion() {
        let inner: TokenStream = "a + b".parse().unwrap();
        let group = Group::new(Delimiter::Parenthesis, inner);

        let pm2: proc_macro2::Group = group.clone().into();
        let back: Group = pm2.into();

        assert_eq!(group.delimiter(), back.delimiter());
        assert_eq!(group.stream().to_string(), back.stream().to_string());
    }
}
