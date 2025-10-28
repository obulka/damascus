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
    fn edges(&self) -> &Edges;
    fn edges_mut(&mut self) -> &mut Edges;

    fn iter_children<'a>(&'a self, node: Node) -> impl Iterator<Item = &'a Node> + 'a
    where
        Node: 'a;
    fn iter_parents<'a>(&'a self, node: Node) -> impl Iterator<Item = &'a Node> + 'a
    where
        Node: 'a;

    fn iter(&self) -> impl Iterator<Item = Node> + '_;

    /// Apply a closure to all descendants of `parent` in breadth first order
    // and collect the return values
    fn for_each_descendant<B, F>(&self, node: Node, closure: F) -> Vec<B>
    where
        F: Fn(&Node) -> B,
    {
        let mut result: Vec<B> = vec![];
        let mut nodes_to_search: Vec<Node> = vec![node];
        while let Some(node) = nodes_to_search.pop() {
            result.extend(self.iter_children(node).map(|descendant| {
                nodes_to_search.push(descendant.clone());
                closure(descendant)
            }));
        }
        result
    }

    /// Check if a node is an ancestor of another
    fn is_descendant(&self, node: Node, potential_descendant: Node) -> bool {
        // Nodes are not their own descendant
        if node == potential_descendant {
            return false;
        }

        let mut nodes_to_search: Vec<Node> = vec![node];
        while let Some(search_node) = nodes_to_search.pop() {
            for descendant in self.iter_children(search_node) {
                if *descendant == potential_descendant {
                    return true;
                }
                nodes_to_search.push(descendant.clone());
            }
        }
        false
    }

    /// Get all descendant nodes of `node` in breadth first order
    fn descendants(&self, node: Node) -> Vec<Node> {
        self.for_each_descendant(node, |descendant| descendant.clone())
    }

    /// Apply a closure to all ancestors of `node` in breadth first order
    // and collect the return values
    fn for_each_ancestor<B, F>(&self, node: Node, closure: F) -> Vec<B>
    where
        F: Fn(&Node) -> B,
    {
        let mut result: Vec<B> = vec![];
        let mut nodes_to_search: Vec<Node> = vec![node];
        while let Some(search_node) = nodes_to_search.pop() {
            result.extend(self.iter_parents(search_node).map(|ancestor| {
                nodes_to_search.push(ancestor.clone());
                closure(ancestor)
            }));
        }
        result
    }

    /// Get all ancestor nodes of `node` in breadth first order
    fn ancestors(&self, node: Node) -> Vec<Node> {
        self.for_each_ancestor(node, |ancestor| ancestor.clone())
    }

    /// Check if a node is an ancestor of another
    fn is_ancestor(&self, node: Node, potential_ancestor: Node) -> bool {
        // Nodes are not their own ancestor
        if node == potential_ancestor {
            return false;
        }

        let mut nodes_to_search: Vec<Node> = vec![node];
        while let Some(search_node) = nodes_to_search.pop() {
            for ancestor in self.iter_parents(search_node) {
                if *ancestor == potential_ancestor {
                    return true;
                }
                nodes_to_search.push(ancestor.clone());
            }
        }
        false
    }
}
