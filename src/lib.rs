#![allow(unused)]

use {
    once_cell::sync::OnceCell,
    proc_macro2::{TokenStream, TokenTree},
    serde::{Deserialize, Serialize},
    std::{
        collections::BTreeMap,
        error::Error,
        ops::{Index, Range},
        str::FromStr,
        sync::{Arc, Weak as WeakArc},
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    root_node: Arc<Node>,
    source: String,
    nodes_by_span: BTreeMap<(usize, usize), Arc<Node>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    node_type: NodeType,
    span: (usize, usize),
    children: Vec<Arc<Node>>,
    #[serde(skip)]
    file: WeakArc<File>,
    #[serde(skip)]
    parent: WeakArc<Node>,
    #[serde(skip)]
    previous_sibling: WeakArc<Node>,
    #[serde(skip)]
    next_sibling: OnceCell<WeakArc<Node>>,
}

pub fn parse(s: &str) -> Result<Arc<Node>, Box<dyn Error>> {
    Node::parse_file(s)
}

impl Node {
    fn parse_file(s: &str) -> Result<Arc<Node>, Box<dyn Error>> {
        let file = Arc::new(s.to_string());
        let token_stream = TokenStream::from_str(s)?;
        let mut children = Vec::new();

        Ok(Arc::new_cyclic(|root| {
            let mut previous = WeakArc::new();

            for token in token_stream.into_iter() {
                let node = Node::parse_token(token, file.clone(), root.clone(), previous.clone());
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
                root: root.clone(),
                span: (0, file.len()),
                file,
                children,
                parent: WeakArc::new(),
                next_sibling: OnceCell::with_value(WeakArc::new()),
                previous_sibling: WeakArc::new(),
            }
        }))
    }

    fn parse_token(
        token: TokenTree,
        file: Arc<String>,
        parent: WeakArc<Node>,
        previous_sibling: WeakArc<Node>,
    ) -> Arc<Node> {
        todo!()
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
