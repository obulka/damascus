// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

pub mod edges;
pub mod node_graph;
pub mod scene_graph;

use edges::BidirectedEdges;

pub trait BidirectedGraph<Family, Edges, Parent, Child>
where
    Family: Clone + PartialEq,
    Edges: BidirectedEdges<Parent, Child>,
    Parent: Clone,
    Child: Clone,
{
    // fn edges(&self) -> &Edges;
    // fn edges_mut(&mut self) -> &mut Edges;

    fn family_count(&self) -> usize;

    fn clear(&mut self);

    // fn iter_family_children<'a>(&'a self, family: Family) -> impl Iterator<Item = &'a Child> + 'a
    // where
    //     Family: 'a;
    // fn iter_family_parents<'a>(&'a self, family: Family) -> impl Iterator<Item = &'a Parent> + 'a
    // where
    //     Family: 'a;

    fn iter_children<'a>(&'a self, family: &'a Family) -> impl Iterator<Item = &'a Family> + 'a
    where
        Family: 'a;
    fn iter_parents<'a>(&'a self, family: &'a Family) -> impl Iterator<Item = &'a Family> + 'a
    where
        Family: 'a;
    // {
    //     self
    //         .iter_family_parents()
    //         .flat_map(|parent| self.edges().iter_children(parent))
    //         .map(|input_id| &self[*input_id].family_id)
    // }

    fn has_child(&self, family: &Family) -> bool {
        self.iter_children(family).peekable().peek().is_some()
    }

    fn has_parent(&self, family: &Family) -> bool {
        self.iter_parents(family).peekable().peek().is_some()
    }

    /// Apply a closure to all descendants of `parent` in breadth first order
    // and collect the return values
    fn for_each_descendant<'a, B, F>(&'a self, family: &'a Family, closure: F) -> Vec<B>
    where
        F: Fn(&'a Family) -> B,
    {
        let mut result: Vec<B> = vec![];
        let mut families_to_search: Vec<&Family> = vec![family];
        while let Some(family) = families_to_search.pop() {
            result.extend(self.iter_children(family).map(|descendant| {
                families_to_search.push(descendant);
                closure(descendant)
            }));
        }
        result
    }

    /// Check if a family contains an ancestor of another
    fn is_descendant(&self, family: &Family, potential_descendant: &Family) -> bool {
        // Familys are not their own descendant
        if family == potential_descendant {
            return false;
        }

        let mut families_to_search: Vec<&Family> = vec![family];
        while let Some(search_family) = families_to_search.pop() {
            for descendant in self.iter_children(search_family) {
                if *descendant == *potential_descendant {
                    return true;
                }
                families_to_search.push(descendant);
            }
        }
        false
    }

    /// Get all descendant families of `family` in breadth first order
    fn descendants<'a>(&'a self, family: &'a Family) -> Vec<&'a Family> {
        self.for_each_descendant(family, |descendant| descendant)
    }

    /// Apply a closure to all ancestors of `family` in breadth first order
    // and collect the return values
    fn for_each_ancestor<'a, B, F>(&'a self, family: &'a Family, closure: F) -> Vec<B>
    where
        F: Fn(&'a Family) -> B,
    {
        let mut result: Vec<B> = vec![];
        let mut families_to_search: Vec<&'a Family> = vec![family];
        while let Some(search_family) = families_to_search.pop() {
            result.extend(self.iter_parents(search_family).map(|ancestor| {
                families_to_search.push(ancestor);
                closure(ancestor)
            }));
        }
        result
    }

    /// Get all ancestor families of `family` in breadth first order
    fn ancestors<'a>(&'a self, family: &'a Family) -> Vec<&'a Family> {
        self.for_each_ancestor(family, |ancestor| ancestor)
    }

    /// Check if a family is an ancestor of another
    fn is_ancestor(&self, family: &Family, potential_ancestor: &Family) -> bool {
        // Familys are not their own ancestor
        if family == potential_ancestor {
            return false;
        }

        let mut families_to_search: Vec<&Family> = vec![family];
        while let Some(search_family) = families_to_search.pop() {
            for ancestor in self.iter_parents(search_family) {
                if *ancestor == *potential_ancestor {
                    return true;
                }
                families_to_search.push(ancestor);
            }
        }
        false
    }
}
