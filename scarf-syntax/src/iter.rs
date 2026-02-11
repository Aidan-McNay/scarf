// =======================================================================
// iter.rs
// =======================================================================
// Iterating over a source tree

use crate::*;
use std::ops::Add;

pub struct NodeIter<'a> {
    nodes: Vec<Node<'a>>,
}

impl<'a> Default for NodeIter<'a> {
    fn default() -> Self {
        NodeIter { nodes: Vec::new() }
    }
}

impl<'a> From<Vec<Node<'a>>> for NodeIter<'a> {
    fn from(value: Vec<Node<'a>>) -> Self {
        NodeIter { nodes: value }
    }
}

impl<'a> Into<Vec<Node<'a>>> for NodeIter<'a> {
    fn into(self) -> Vec<Node<'a>> {
        self.nodes
    }
}

impl<'a> From<Node<'a>> for NodeIter<'a> {
    fn from(value: Node<'a>) -> Self {
        NodeIter { nodes: vec![value] }
    }
}

impl<'a> Add for NodeIter<'a> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.nodes.extend(rhs.nodes);
        self
    }
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = Node<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.nodes.pop();
        if let Some(node) = next.clone() {
            let mut new_nodes: Vec<Node<'a>> = node.nodes().into();
            new_nodes.reverse();
            self.nodes.append(&mut new_nodes);
        };
        next
    }
}

/// An object that can be represented as a collection of AST nodes
pub trait Nodes<'a> {
    /// The nodes of the object
    ///
    /// This includes the object itself, as well as any children nodes
    fn nodes(&'a self) -> NodeIter<'a>;

    /// Iterate over the nodes for an object
    fn iter(&'a self) -> NodeIter<'a> {
        self.nodes()
    }
}

impl<'a, T> Nodes<'a> for Box<T>
where
    T: Nodes<'a>,
{
    fn nodes(&'a self) -> NodeIter<'a> {
        self.as_ref().nodes()
    }
}

impl<'a, T> Nodes<'a> for Option<T>
where
    T: Nodes<'a>,
{
    fn nodes(&'a self) -> NodeIter<'a> {
        match self {
            Some(data) => data.nodes(),
            None => NodeIter::default(),
        }
    }
}

impl<'a, T> Nodes<'a> for Vec<T>
where
    T: Nodes<'a>,
{
    fn nodes(&'a self) -> NodeIter<'a> {
        let mut ret: NodeIter<'a> = NodeIter::default();
        for member in self {
            ret = ret + member.nodes().into();
        }
        ret
    }
}

impl<'a, T0: 'a, T1: 'a> Nodes<'a> for (T0, T1)
where
    T0: Nodes<'a>,
    T1: Nodes<'a>,
{
    fn nodes(&'a self) -> NodeIter<'a> {
        self.0.nodes() + self.1.nodes()
    }
}

impl<'a, T0: 'a, T1: 'a, T2: 'a> Nodes<'a> for (T0, T1, T2)
where
    T0: Nodes<'a>,
    T1: Nodes<'a>,
    T2: Nodes<'a>,
{
    fn nodes(&'a self) -> NodeIter<'a> {
        self.0.nodes() + self.1.nodes() + self.2.nodes()
    }
}

impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a> Nodes<'a> for (T0, T1, T2, T3)
where
    T0: Nodes<'a>,
    T1: Nodes<'a>,
    T2: Nodes<'a>,
    T3: Nodes<'a>,
{
    fn nodes(&'a self) -> NodeIter<'a> {
        self.0.nodes() + self.1.nodes() + self.2.nodes() + self.3.nodes()
    }
}

impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a> Nodes<'a>
    for (T0, T1, T2, T3, T4)
where
    T0: Nodes<'a>,
    T1: Nodes<'a>,
    T2: Nodes<'a>,
    T3: Nodes<'a>,
    T4: Nodes<'a>,
{
    fn nodes(&'a self) -> NodeIter<'a> {
        self.0.nodes()
            + self.1.nodes()
            + self.2.nodes()
            + self.3.nodes()
            + self.4.nodes()
    }
}

impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a> Nodes<'a>
    for (T0, T1, T2, T3, T4, T5)
where
    T0: Nodes<'a>,
    T1: Nodes<'a>,
    T2: Nodes<'a>,
    T3: Nodes<'a>,
    T4: Nodes<'a>,
    T5: Nodes<'a>,
{
    fn nodes(&'a self) -> NodeIter<'a> {
        self.0.nodes()
            + self.1.nodes()
            + self.2.nodes()
            + self.3.nodes()
            + self.4.nodes()
            + self.5.nodes()
    }
}

impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a> Nodes<'a>
    for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: Nodes<'a>,
    T1: Nodes<'a>,
    T2: Nodes<'a>,
    T3: Nodes<'a>,
    T4: Nodes<'a>,
    T5: Nodes<'a>,
    T6: Nodes<'a>,
{
    fn nodes(&'a self) -> NodeIter<'a> {
        self.0.nodes()
            + self.1.nodes()
            + self.2.nodes()
            + self.3.nodes()
            + self.4.nodes()
            + self.5.nodes()
            + self.6.nodes()
    }
}

impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a>
    Nodes<'a> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: Nodes<'a>,
    T1: Nodes<'a>,
    T2: Nodes<'a>,
    T3: Nodes<'a>,
    T4: Nodes<'a>,
    T5: Nodes<'a>,
    T6: Nodes<'a>,
    T7: Nodes<'a>,
{
    fn nodes(&'a self) -> NodeIter<'a> {
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

impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a>
    Nodes<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    T0: Nodes<'a>,
    T1: Nodes<'a>,
    T2: Nodes<'a>,
    T3: Nodes<'a>,
    T4: Nodes<'a>,
    T5: Nodes<'a>,
    T6: Nodes<'a>,
    T7: Nodes<'a>,
    T8: Nodes<'a>,
{
    fn nodes(&'a self) -> NodeIter<'a> {
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

impl<
    'a,
    T0: 'a,
    T1: 'a,
    T2: 'a,
    T3: 'a,
    T4: 'a,
    T5: 'a,
    T6: 'a,
    T7: 'a,
    T8: 'a,
    T9: 'a,
> Nodes<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T0: Nodes<'a>,
    T1: Nodes<'a>,
    T2: Nodes<'a>,
    T3: Nodes<'a>,
    T4: Nodes<'a>,
    T5: Nodes<'a>,
    T6: Nodes<'a>,
    T7: Nodes<'a>,
    T8: Nodes<'a>,
    T9: Nodes<'a>,
{
    fn nodes(&'a self) -> NodeIter<'a> {
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

impl<
    'a,
    T0: 'a,
    T1: 'a,
    T2: 'a,
    T3: 'a,
    T4: 'a,
    T5: 'a,
    T6: 'a,
    T7: 'a,
    T8: 'a,
    T9: 'a,
    T10: 'a,
> Nodes<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T0: Nodes<'a>,
    T1: Nodes<'a>,
    T2: Nodes<'a>,
    T3: Nodes<'a>,
    T4: Nodes<'a>,
    T5: Nodes<'a>,
    T6: Nodes<'a>,
    T7: Nodes<'a>,
    T8: Nodes<'a>,
    T9: Nodes<'a>,
    T10: Nodes<'a>,
{
    fn nodes(&'a self) -> NodeIter<'a> {
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

impl<
    'a,
    T0: 'a,
    T1: 'a,
    T2: 'a,
    T3: 'a,
    T4: 'a,
    T5: 'a,
    T6: 'a,
    T7: 'a,
    T8: 'a,
    T9: 'a,
    T10: 'a,
    T11: 'a,
> Nodes<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    T0: Nodes<'a>,
    T1: Nodes<'a>,
    T2: Nodes<'a>,
    T3: Nodes<'a>,
    T4: Nodes<'a>,
    T5: Nodes<'a>,
    T6: Nodes<'a>,
    T7: Nodes<'a>,
    T8: Nodes<'a>,
    T9: Nodes<'a>,
    T10: Nodes<'a>,
    T11: Nodes<'a>,
{
    fn nodes(&'a self) -> NodeIter<'a> {
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

impl<'a> Nodes<'a> for Metadata<'a> {
    fn nodes(&self) -> NodeIter<'a> {
        NodeIter::default()
    }
}

impl<'a> Nodes<'a> for ExtraNode<'a> {
    fn nodes(&self) -> NodeIter<'a> {
        NodeIter::default()
    }
}

// impl<'a, T: 'a> Nodes<'a> for &'a T
// where
//     T: Nodes<'a>,
// {
//     fn nodes(&'a self) -> NodeIter<'a> {
//         (*self).nodes()
//     }
// }

impl<'a> Nodes<'a> for &'a str {
    fn nodes(&self) -> NodeIter<'a> {
        NodeIter::default()
    }
}
