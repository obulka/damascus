// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

pub mod edges;
pub mod node_graph;
pub mod scene_graph;

use edges::BidirectedEdges;

pub trait BidirectedGraph<Node, Edges, Parent, Child>
where
    Node: Clone + PartialEq,
    Edges: BidirectedEdges<Parent, Child>,
    Parent: Clone,
    Child: Clone,
{
    // fn edges(&self) -> &Edges;
    // fn edges_mut(&mut self) -> &mut Edges;

    fn node_count(&self) -> usize;

    fn clear(&mut self);

    // fn iter_node_children<'a>(&'a self, node: Node) -> impl Iterator<Item = &'a Child> + 'a
    // where
    //     Node: 'a;
    // fn iter_node_parents<'a>(&'a self, node: Node) -> impl Iterator<Item = &'a Parent> + 'a
    // where
    //     Node: 'a;

    // fn iter(&self) -> impl Iterator<Item = Node> + '_;

    fn iter_children<'a>(&'a self, node: &'a Node) -> impl Iterator<Item = &'a Node> + 'a
    where
        Node: 'a;
    fn iter_parents<'a>(&'a self, node: &'a Node) -> impl Iterator<Item = &'a Node> + 'a
    where
        Node: 'a;
    // {
    //     self
    //         .iter_node_parents()
    //         .flat_map(|parent| self.edges().iter_children(parent))
    //         .map(|input_id| &self[*input_id].node_id)
    // }

    /// Apply a closure to all descendants of `parent` in breadth first order
    // and collect the return values
    fn for_each_descendant<'a, B, F>(&'a self, node: &'a Node, closure: F) -> Vec<B>
    where
        F: Fn(&'a Node) -> B,
    {
        let mut result: Vec<B> = vec![];
        let mut nodes_to_search: Vec<&Node> = vec![node];
        while let Some(node) = nodes_to_search.pop() {
            result.extend(self.iter_children(node).map(|descendant| {
                nodes_to_search.push(descendant);
                closure(descendant)
            }));
        }
        result
    }

    /// Check if a node is an ancestor of another
    fn is_descendant(&self, node: &Node, potential_descendant: &Node) -> bool {
        // Nodes are not their own descendant
        if node == potential_descendant {
            return false;
        }

        let mut nodes_to_search: Vec<&Node> = vec![node];
        while let Some(search_node) = nodes_to_search.pop() {
            for descendant in self.iter_children(search_node) {
                if *descendant == *potential_descendant {
                    return true;
                }
                nodes_to_search.push(descendant);
            }
        }
        false
    }

    /// Get all descendant nodes of `node` in breadth first order
    fn descendants<'a>(&'a self, node: &'a Node) -> Vec<&'a Node> {
        self.for_each_descendant(node, |descendant| descendant)
    }

    /// Apply a closure to all ancestors of `node` in breadth first order
    // and collect the return values
    fn for_each_ancestor<'a, B, F>(&'a self, node: &'a Node, closure: F) -> Vec<B>
    where
        F: Fn(&'a Node) -> B,
    {
        let mut result: Vec<B> = vec![];
        let mut nodes_to_search: Vec<&'a Node> = vec![node];
        while let Some(search_node) = nodes_to_search.pop() {
            result.extend(self.iter_parents(search_node).map(|ancestor| {
                nodes_to_search.push(ancestor);
                closure(ancestor)
            }));
        }
        result
    }

    /// Get all ancestor nodes of `node` in breadth first order
    fn ancestors<'a>(&'a self, node: &'a Node) -> Vec<&'a Node> {
        self.for_each_ancestor(node, |ancestor| ancestor)
    }

    /// Check if a node is an ancestor of another
    fn is_ancestor(&self, node: &Node, potential_ancestor: &Node) -> bool {
        // Nodes are not their own ancestor
        if node == potential_ancestor {
            return false;
        }

        let mut nodes_to_search: Vec<&Node> = vec![node];
        while let Some(search_node) = nodes_to_search.pop() {
            for ancestor in self.iter_parents(search_node) {
                if *ancestor == *potential_ancestor {
                    return true;
                }
                nodes_to_search.push(ancestor);
            }
        }
        false
    }
}
