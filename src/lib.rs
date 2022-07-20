#![allow(unused)]

use {
    once_cell::sync::OnceCell,
    proc_macro2::{TokenStream, TokenTree},
    std::{
        collections::BTreeMap,
        error::Error,
        ops::{Index, Range},
        str::FromStr,
        sync::{Arc, Weak as WeakArc},
    },
};

#[derive(Debug)]
pub struct File {
    source: Arc<String>,
    root: OnceCell<Arc<Node>>,
    line_offsets: Vec<usize>,
    nodes_by_span: BTreeMap<(usize, usize), Arc<Node>>,
}

impl File {
    pub fn parse(source: &str) -> Result<Self, Box<dyn Error>> {
        let source = Arc::new(source.to_string());
        let token_stream = TokenStream::from_str(source.as_ref())?;
        let mut children = Vec::new();

        let line_offsets = vec![];
        for (offset, byte) in source.bytes().enumerate() {
            if byte == b'\n' {
                line_offsets.push(offset);
            }
        }

        let line_offsets = vec![];
        for (offset, byte) in source.bytes().enumerate() {
            if byte == b'\n' {
                line_offsets.push(offset);
            }
        }

        let nodes_by_span = Default::default();

        let file = Ok(Arc::new_cyclic(|root| {
            let mut previous = WeakArc::new();

            for token in token_stream.into_iter() {
                let node = Node::parse_token(token, source.clone(), root.clone(), previous.clone());
                if let Some(ref previous) = previous.upgrade() {
                    previous
                        .next_sibling
                        .set(Arc::downgrade(&previous))
                        .unwrap();
                }
                children.push(node);
            }

            if let Some(last) = children.last() {
                last.next_sibling.set(Default::default());
            };

            Node {
                node_type: NodeType::File,
                span: (0, source.len()),
                source,
                line_offsets,
                children,
                parent: WeakArc::new(),
                next_sibling: OnceCell::with_value(WeakArc::new()),
                previous_sibling: WeakArc::new(),
            }
        }));

        // needs more new_cylic
        Ok(Arc::new(File {
            nodes_by_span,
            line_offsets,
            source,
            root: OnceCell::new(),
        }));
    }
}

#[derive(Debug)]
pub struct Node {
    source: Arc<String>,
    node_type: NodeType,
    span: (usize, usize),
    children: Vec<Arc<Node>>,
    file: WeakArc<File>,
    parent: WeakArc<Node>,
    previous_sibling: WeakArc<Node>,
    next_sibling: OnceCell<WeakArc<Node>>,
}

impl Node {
    fn parse_file(source: &str) -> Result<Arc<Node>, Box<dyn Error>> {
        let source = Arc::new(source.to_string());
        let token_stream = TokenStream::from_str(source.as_ref())?;
        let mut children = Vec::new();
    }

    fn parse_token(
        token: TokenTree,
        file: Arc<String>,
        parent: WeakArc<Node>,
        previous_sibling: WeakArc<Node>,
    ) -> Arc<Node> {
        let mut children = Vec::new();
        let node_type = match token {
            TokenTree::Group(group) => {
                let mut children = Vec::new();
                for token in group.stream().into_iter() {
                    children.push(Node::parse_token(
                        token,
                        file.clone(),
                        parent.clone(),
                        previous_sibling.clone(),
                    ));
                }
                NodeType::Group
            }
            TokenTree::Punct(punct) => NodeType::Punct,
            TokenTree::Ident(ident) => NodeType::Ident,
            TokenTree::Literal(literal) => NodeType::Literal,
        };
        let span = (token.span().start(), token.span().end());
        let node = Arc::new(Node {
            node_type,
            span,
            file,
            parent,
            previous_sibling,
            next_sibling: OnceCell::with_value(WeakArc::new()),
            children: Vec::new(),
            root: parent.clone(),
        });
        if let Some(ref previous_sibling) = previous_sibling.upgrade() {
            previous_sibling
                .next_sibling
                .set(Arc::downgrade(&previous_sibling))
                .unwrap();
        }
        node
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NodeType {
    File,
    Group,
    Punct,
    Ident,
    Literal,
}

impl Node {
    pub fn node_type(&self) -> &NodeType {
        &self.node_type
    }
}

impl AsRef<str> for Node {
    fn as_ref(&self) -> &str {
        &self.file[self.span.0..self.span.1]
    }
}
