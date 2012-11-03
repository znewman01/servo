use dom::node::{Node, NodeTree, Doctype, Comment, Element, Text};
use newcss::SelectHandler;
use util::tree;

pub struct NodeSelectHandler {
    node: Node
}

/// Placeholder names
fn unnamed_node(name: &str) -> ~str {
    fmt!("unnamed-%s", name)
}

impl NodeSelectHandler: SelectHandler<Node> {
    fn node_name(node: &Node) -> ~str {
        do node.read |data| {
            match *data.kind {
                Doctype(*) => unnamed_node("doctype"),
                Comment(*) => unnamed_node("comment"),
                Element(ref data) => copy data.tag_name,
                Text(*) => unnamed_node("text")
            }
        }
    }

    fn named_parent_node(node: &Node, name: &str) -> Option<Node> {
        let parent = tree::parent(&NodeTree, node);
        match parent {
            Some(parent) => {
                do parent.read |data| {
                    if name == data.tag_name {
                        Some(parent)
                    } else {
                        None
                    }
                }
            }
            None => None
        }
    }

    fn parent_node(node: &Node) -> Option<Node> {
        tree::parent(&NodeTree, node)
    }
}