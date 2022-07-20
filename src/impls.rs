use crate::{fallback::TokenStream as TokenStreamInner, TokenStream, TokenTree};
use std::ops::{Deref, DerefMut};

impl Deref for TokenStream {
    type Target = Vec<TokenTree>;

    fn deref(&self) -> &Self::Target {
        &self.inner.inner
    }
}

// XXX: you're missing the important part: .group().stream() clones when you deref it!
impl DerefMut for TokenStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner.inner
    }
}

impl AsRef<Vec<TokenTree>> for TokenStream {
    fn as_ref(&self) -> &Vec<TokenTree> {
        &self.inner.inner
    }
}

impl AsMut<Vec<TokenTree>> for TokenStream {
    fn as_mut(&mut self) -> &mut Vec<TokenTree> {
        &mut self.inner.inner
    }
}

impl AsRef<Vec<TokenTree>> for TokenStreamInner {
    fn as_ref(&self) -> &Vec<TokenTree> {
        &self.inner
    }
}

impl AsMut<Vec<TokenTree>> for TokenStreamInner {
    fn as_mut(&mut self) -> &mut Vec<TokenTree> {
        &mut self.inner
    }
}
