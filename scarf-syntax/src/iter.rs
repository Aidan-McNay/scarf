// =======================================================================
// iter.rs
// =======================================================================
// Iterating over a source tree

use crate::*;
use std::ops::Add;

pub struct NodeIter<'a, 'b> {
    nodes: Vec<Node<'a, 'b>>,
}

impl<'a: 'b, 'b> Default for NodeIter<'a, 'b> {
    fn default() -> Self {
        NodeIter { nodes: Vec::new() }
    }
}

impl<'a: 'b, 'b> From<Vec<Node<'a, 'b>>> for NodeIter<'a, 'b> {
    fn from(value: Vec<Node<'a, 'b>>) -> Self {
        NodeIter { nodes: value }
    }
}

impl<'a: 'b, 'b> Into<Vec<Node<'a, 'b>>> for NodeIter<'a, 'b> {
    fn into(self) -> Vec<Node<'a, 'b>> {
        self.nodes
    }
}

impl<'a: 'b, 'b> From<Node<'a, 'b>> for NodeIter<'a, 'b> {
    fn from(value: Node<'a, 'b>) -> Self {
        NodeIter { nodes: vec![value] }
    }
}

impl<'a: 'b, 'b> Add for NodeIter<'a, 'b> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.nodes.extend(rhs.nodes);
        self
    }
}

impl<'a: 'b, 'b> Iterator for NodeIter<'a, 'b> {
    type Item = Node<'a, 'b>;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.nodes.pop();
        if let Some(node) = next.clone() {
            let mut new_nodes: Vec<Node<'a, 'b>> = node.children();
            new_nodes.reverse();
            self.nodes.append(&mut new_nodes);
        };
        next
    }
}

/// An object that can be represented as a collection of AST nodes
pub trait Nodes<'a: 'b, 'b> {
    /// The nodes of the object
    ///
    /// This includes the object itself, as well as any children nodes
    fn nodes(&'b self) -> NodeIter<'a, 'b>;

    /// Iterate over the nodes for an object
    fn iter(&'b self) -> NodeIter<'a, 'b> {
        self.nodes()
    }
}

impl<'a: 'b, 'b, T> Nodes<'a, 'b> for Box<T>
where
    T: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.as_ref().nodes()
    }
}

impl<'a: 'b, 'b, T> Nodes<'a, 'b> for Option<T>
where
    T: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        match self {
            Some(data) => data.nodes(),
            None => NodeIter::default(),
        }
    }
}

impl<'a: 'b, 'b, T> Nodes<'a, 'b> for Vec<T>
where
    T: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        let mut ret: NodeIter<'a, 'b> = NodeIter::default();
        for member in self {
            ret = ret + member.nodes().into();
        }
        ret
    }
}

impl<'a: 'b, 'b, T0, T1> Nodes<'a, 'b> for (T0, T1)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes() + self.1.nodes()
    }
}

impl<'a: 'b, 'b, T0, T1, T2> Nodes<'a, 'b> for (T0, T1, T2)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes() + self.1.nodes() + self.2.nodes()
    }
}

impl<'a: 'b, 'b, T0, T1, T2, T3: 'a> Nodes<'a, 'b> for (T0, T1, T2, T3)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
    T3: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes() + self.1.nodes() + self.2.nodes() + self.3.nodes()
    }
}

impl<'a: 'b, 'b, T0, T1, T2, T3, T4: 'a> Nodes<'a, 'b> for (T0, T1, T2, T3, T4)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
    T3: Nodes<'a, 'b>,
    T4: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes()
            + self.1.nodes()
            + self.2.nodes()
            + self.3.nodes()
            + self.4.nodes()
    }
}

impl<'a: 'b, 'b, T0, T1, T2, T3, T4, T5: 'a> Nodes<'a, 'b>
    for (T0, T1, T2, T3, T4, T5)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
    T3: Nodes<'a, 'b>,
    T4: Nodes<'a, 'b>,
    T5: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes()
            + self.1.nodes()
            + self.2.nodes()
            + self.3.nodes()
            + self.4.nodes()
            + self.5.nodes()
    }
}

impl<'a: 'b, 'b, T0, T1, T2, T3, T4, T5, T6: 'a> Nodes<'a, 'b>
    for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
    T3: Nodes<'a, 'b>,
    T4: Nodes<'a, 'b>,
    T5: Nodes<'a, 'b>,
    T6: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes()
            + self.1.nodes()
            + self.2.nodes()
            + self.3.nodes()
            + self.4.nodes()
            + self.5.nodes()
            + self.6.nodes()
    }
}

impl<'a: 'b, 'b, T0, T1, T2, T3, T4, T5, T6, T7: 'a> Nodes<'a, 'b>
    for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
    T3: Nodes<'a, 'b>,
    T4: Nodes<'a, 'b>,
    T5: Nodes<'a, 'b>,
    T6: Nodes<'a, 'b>,
    T7: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes()
            + self.1.nodes()
            + self.2.nodes()
            + self.3.nodes()
            + self.4.nodes()
            + self.5.nodes()
            + self.6.nodes()
            + self.7.nodes()
    }
}

impl<'a: 'b, 'b, T0, T1, T2, T3, T4, T5, T6, T7, T8> Nodes<'a, 'b>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
    T3: Nodes<'a, 'b>,
    T4: Nodes<'a, 'b>,
    T5: Nodes<'a, 'b>,
    T6: Nodes<'a, 'b>,
    T7: Nodes<'a, 'b>,
    T8: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes()
            + self.1.nodes()
            + self.2.nodes()
            + self.3.nodes()
            + self.4.nodes()
            + self.5.nodes()
            + self.6.nodes()
            + self.7.nodes()
            + self.8.nodes()
    }
}

impl<'a: 'b, 'b, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Nodes<'a, 'b>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
    T3: Nodes<'a, 'b>,
    T4: Nodes<'a, 'b>,
    T5: Nodes<'a, 'b>,
    T6: Nodes<'a, 'b>,
    T7: Nodes<'a, 'b>,
    T8: Nodes<'a, 'b>,
    T9: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes()
            + self.1.nodes()
            + self.2.nodes()
            + self.3.nodes()
            + self.4.nodes()
            + self.5.nodes()
            + self.6.nodes()
            + self.7.nodes()
            + self.8.nodes()
            + self.9.nodes()
    }
}

impl<'a: 'b, 'b, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Nodes<'a, 'b>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
    T3: Nodes<'a, 'b>,
    T4: Nodes<'a, 'b>,
    T5: Nodes<'a, 'b>,
    T6: Nodes<'a, 'b>,
    T7: Nodes<'a, 'b>,
    T8: Nodes<'a, 'b>,
    T9: Nodes<'a, 'b>,
    T10: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes()
            + self.1.nodes()
            + self.2.nodes()
            + self.3.nodes()
            + self.4.nodes()
            + self.5.nodes()
            + self.6.nodes()
            + self.7.nodes()
            + self.8.nodes()
            + self.9.nodes()
            + self.10.nodes()
    }
}

impl<'a: 'b, 'b, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Nodes<'a, 'b>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    T0: Nodes<'a, 'b>,
    T1: Nodes<'a, 'b>,
    T2: Nodes<'a, 'b>,
    T3: Nodes<'a, 'b>,
    T4: Nodes<'a, 'b>,
    T5: Nodes<'a, 'b>,
    T6: Nodes<'a, 'b>,
    T7: Nodes<'a, 'b>,
    T8: Nodes<'a, 'b>,
    T9: Nodes<'a, 'b>,
    T10: Nodes<'a, 'b>,
    T11: Nodes<'a, 'b>,
{
    fn nodes(&'b self) -> NodeIter<'a, 'b> {
        self.0.nodes()
            + self.1.nodes()
            + self.2.nodes()
            + self.3.nodes()
            + self.4.nodes()
            + self.5.nodes()
            + self.6.nodes()
            + self.7.nodes()
            + self.8.nodes()
            + self.9.nodes()
            + self.10.nodes()
            + self.11.nodes()
    }
}

impl<'a: 'b, 'b> Nodes<'a, 'b> for Metadata<'a> {
    fn nodes(&self) -> NodeIter<'a, 'b> {
        NodeIter::default()
    }
}

impl<'a: 'b, 'b> Nodes<'a, 'b> for ExtraNode<'a> {
    fn nodes(&self) -> NodeIter<'a, 'b> {
        NodeIter::default()
    }
}

impl<'a: 'b, 'b> Nodes<'a, 'b> for &'a str {
    fn nodes(&self) -> NodeIter<'a, 'b> {
        NodeIter::default()
    }
}
