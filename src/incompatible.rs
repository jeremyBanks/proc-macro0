//! Trait implementations that are not part of the standard proc_macro/proc_macro2 API.

use crate::{TokenStream, TokenTree};
use std::ops::{Deref, DerefMut};

#[cfg(any(doc, feature = "incompatible"))]
impl Deref for TokenStream {
    type Target = Vec<TokenTree>;

    fn deref(&self) -> &Self::Target {
        &self.inner.inner
    }
}

#[cfg(any(doc, feature = "incompatible"))]
impl DerefMut for TokenStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner.inner
    }
}

#[cfg(any(doc, feature = "incompatible"))]
impl AsRef<Vec<TokenTree>> for TokenStream {
    fn as_ref(&self) -> &Vec<TokenTree> {
        &self.inner.inner
    }
}

#[cfg(any(doc, feature = "incompatible"))]
impl AsMut<Vec<TokenTree>> for TokenStream {
    fn as_mut(&mut self) -> &mut Vec<TokenTree> {
        &mut self.inner.inner
    }
}
