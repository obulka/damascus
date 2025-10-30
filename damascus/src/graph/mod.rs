// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{cmp::Ord, hash::Hash};

pub mod edges;
pub mod node_graph;
pub mod scene_graph;

use edges::{BidirectedEdges, SingleParentBidirectedEdges};

pub trait BidirectedGraph<Node, Edges, Parent, Child>
where
    Node: PartialEq,
    Edges: BidirectedEdges<Parent, Child>,
    Parent: Clone,
    Child: Clone,
{
    fn edges(&self) -> &Edges;

    fn node_count(&self) -> usize;

    fn clear(&mut self);

    fn iter_children<'a>(&'a self, node: &'a Node) -> impl Iterator<Item = &'a Node> + 'a
    where
        Node: 'a;
    fn iter_parents<'a>(&'a self, node: &'a Node) -> impl Iterator<Item = &'a Node> + 'a
    where
        Node: 'a;

    fn has_child(&self, node: &Node) -> bool {
        self.iter_children(node).peekable().peek().is_some()
    }

    fn has_parent(&self, node: &Node) -> bool {
        self.iter_parents(node).peekable().peek().is_some()
    }

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

    /// Check if a node contains an ancestor of another
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

pub trait EvaluableGraph<Node, Input, Output>:
    BidirectedGraph<Node, SingleParentBidirectedEdges<Output, Input>, Output, Input>
where
    Node: PartialEq,
    Input: Clone + Hash + Ord,
    Output: Clone + Hash + Ord,
{
    fn node_for_input<'a>(&'a self, input: &'a Input) -> &'a Node;
    fn node_for_output<'a>(&'a self, output: &'a Output) -> &'a Node;

    fn iter_inputs<'a>(&'a self, node: &'a Node) -> impl Iterator<Item = &'a Input> + 'a
    where
        Node: 'a,
        Input: 'a;
    fn iter_outputs<'a>(&'a self, node: &'a Node) -> impl Iterator<Item = &'a Output> + 'a
    where
        Node: 'a,
        Output: 'a;

    fn iter_children<'a>(&'a self, node: &'a Node) -> impl Iterator<Item = &'a Node> + 'a
    where
        Node: 'a,
        Input: 'a,
        Output: 'a,
    {
        self.iter_outputs(node)
            .flat_map(|output| self.edges().iter_children(output))
            .map(|input| self.node_for_input(input))
    }

    fn iter_parents<'a>(&'a self, node: &'a Node) -> impl Iterator<Item = &'a Node> + 'a
    where
        Node: 'a,
        Input: 'a,
        Output: 'a,
    {
        self.iter_inputs(node)
            .flat_map(|input| self.edges().iter_parents(input))
            .map(|output| self.node_for_output(output))
    }
}
