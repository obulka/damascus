// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{
    cmp::Ord,
    collections::{BTreeSet, HashMap},
    hash::Hash,
};

pub trait BidirectedEdges<Parent, Child>
where
    Parent: Clone,
    Child: Clone,
{
    fn num_parents(&self) -> usize;
    fn num_children(&self) -> usize;

    fn clear(&mut self);

    fn iter_parents<'a>(&'a self, child: &'a Child) -> impl Iterator<Item = &'a Parent> + 'a
    where
        Parent: 'a,
        Child: 'a;
    fn iter_children<'a>(&'a self, parent: &'a Parent) -> impl Iterator<Item = &'a Child> + 'a
    where
        Parent: 'a,
        Child: 'a;

    fn has_parent(&self, child: &Child) -> bool {
        self.iter_parents(child).peekable().peek().is_some()
    }

    fn has_child(&self, parent: &Parent) -> bool {
        self.iter_children(parent).peekable().peek().is_some()
    }

    fn disconnect(&mut self, parent: &Parent, child: &Child) -> bool;
    fn connect(&mut self, parent: Parent, child: Child) -> bool;

    fn disconnect_parents_of_child<'a>(
        &'a mut self,
        child: &'a Child,
    ) -> impl Iterator<Item = Parent> + 'a
    where
        Parent: 'a,
        Child: 'a,
    {
        self.iter_parents(child)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .filter_map(|parent| {
                if self.disconnect(&parent, child) {
                    Some(parent)
                } else {
                    None
                }
            })
    }

    fn disconnect_children_of_parent<'a>(
        &'a mut self,
        parent: &'a Parent,
    ) -> impl Iterator<Item = Child> + 'a
    where
        Parent: 'a,
        Child: 'a,
    {
        self.iter_children(parent)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .filter_map(|child| {
                if self.disconnect(parent, &child) {
                    Some(child)
                } else {
                    None
                }
            })
    }

    fn disconnect_children_of_parents<'a>(
        &'a mut self,
        parents: impl Iterator<Item = &'a Parent> + 'a,
    ) -> impl Iterator<Item = (&'a Parent, Child)> + 'a
    where
        Parent: 'a,
        Child: 'a,
    {
        parents.flat_map(|parent| {
            self.disconnect_children_of_parent(parent)
                .map(|child| (parent, child))
                .collect::<Vec<_>>()
        })
    }

    fn disconnect_parents_of_children<'a>(
        &'a mut self,
        children: impl Iterator<Item = &'a Child> + 'a,
    ) -> impl Iterator<Item = (Parent, &'a Child)> + 'a
    where
        Parent: 'a,
        Child: 'a,
    {
        children.flat_map(|child| {
            self.disconnect_parents_of_child(child)
                .map(|parent| (parent, child))
                .collect::<Vec<_>>()
        })
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SingleParentBidirectedEdges<Parent, Child>
where
    Parent: Clone + Hash + Ord,
    Child: Clone + Hash + Ord,
{
    parents: HashMap<Child, Parent>,
    children: HashMap<Parent, BTreeSet<Child>>,
}

impl<Parent: Clone + Hash + Ord, Child: Clone + Hash + Ord> BidirectedEdges<Parent, Child>
    for SingleParentBidirectedEdges<Parent, Child>
{
    fn num_parents(&self) -> usize {
        self.parents.len()
    }

    fn num_children(&self) -> usize {
        self.children.len()
    }

    fn clear(&mut self) {
        self.parents.clear();
        self.children.clear();
    }

    fn iter_parents<'a>(&'a self, child: &'a Child) -> impl Iterator<Item = &'a Parent> + 'a
    where
        Parent: 'a,
        Child: 'a,
    {
        self.parents.get(child).into_iter()
    }

    fn iter_children<'a>(&'a self, parent: &'a Parent) -> impl Iterator<Item = &'a Child> + 'a
    where
        Parent: 'a,
        Child: 'a,
    {
        self.children.get(parent).into_iter().flatten().into_iter()
    }

    fn disconnect(&mut self, parent: &Parent, child: &Child) -> bool {
        let mut disconnected = false;
        if let Some(current_parent) = self.parents.get(child)
            && *parent == *current_parent
        {
            disconnected = self.parents.remove(child).is_some();

            let mut all_children_removed = false;
            if let Some(children) = self.children.get_mut(parent) {
                disconnected |= children.remove(child);
                all_children_removed = children.is_empty();
            }
            if all_children_removed {
                self.children.remove(parent);
            }
        }
        disconnected
    }

    fn connect(&mut self, parent: Parent, child: Child) -> bool {
        if let Some(current_parent) = self.parent(&child) {
            // The child already has a parent
            if *current_parent == parent {
                // The current parent is the new parent, so we do not
                // need to do anything
                return false;
            }

            // Disconnect the child from its current parent
            self.disconnect(&current_parent.clone(), &child);
        }

        // Create a new connection
        if let Some(children) = self.children.get_mut(&parent) {
            children.insert(child.clone());
        } else {
            let mut children = BTreeSet::<Child>::new();
            children.insert(child.clone());
            self.children.insert(parent.clone(), children);
        }

        self.parents.insert(child, parent);

        true
    }
}

impl<Parent: Clone + Hash + Ord, Child: Clone + Hash + Ord>
    SingleParentBidirectedEdges<Parent, Child>
{
    pub fn new() -> Self {
        Self {
            parents: HashMap::<Child, Parent>::new(),
            children: HashMap::<Parent, BTreeSet<Child>>::new(),
        }
    }

    pub fn parent(&self, child: &Child) -> Option<&Parent> {
        self.parents.get(child)
    }

    pub fn disconnect_parent_of_child(&mut self, child: &Child) -> Option<Parent> {
        if let Some(parent) = self.parents.get(child).cloned()
            && self.disconnect(&parent, child)
        {
            Some(parent)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MultiParentBidirectedEdges<Parent, Child>
where
    Parent: Clone + Hash + Ord,
    Child: Clone + Hash + Ord,
{
    parents: HashMap<Child, BTreeSet<Parent>>,
    children: HashMap<Parent, BTreeSet<Child>>,
}

impl<Parent: Clone + Hash + Ord, Child: Clone + Hash + Ord> BidirectedEdges<Parent, Child>
    for MultiParentBidirectedEdges<Parent, Child>
{
    fn num_parents(&self) -> usize {
        self.parents.len()
    }

    fn num_children(&self) -> usize {
        self.children.len()
    }

    fn clear(&mut self) {
        self.children.clear();
        self.parents.clear();
    }

    fn iter_parents<'a>(&'a self, child: &'a Child) -> impl Iterator<Item = &'a Parent> + 'a
    where
        Parent: 'a,
        Child: 'a,
    {
        self.parents.get(child).into_iter().flatten().into_iter()
    }

    fn iter_children<'a>(&'a self, parent: &'a Parent) -> impl Iterator<Item = &'a Child> + 'a
    where
        Parent: 'a,
        Child: 'a,
    {
        self.children.get(parent).into_iter().flatten().into_iter()
    }

    fn disconnect(&mut self, parent: &Parent, child: &Child) -> bool {
        let mut disconnected = false;
        let mut all_children_removed = false;
        if let Some(children) = self.children.get_mut(parent) {
            disconnected |= children.remove(child);
            all_children_removed = children.is_empty();
        }

        if all_children_removed {
            self.children.remove(parent);
        }

        let mut all_parents_removed = false;
        if let Some(parents) = self.parents.get_mut(child) {
            disconnected |= parents.remove(parent);
            all_parents_removed = parents.is_empty();
        }

        if all_parents_removed {
            self.parents.remove(child);
        }

        disconnected
    }

    fn connect(&mut self, parent: Parent, child: Child) -> bool {
        let mut connected = false;
        if let Some(children) = self.children.get_mut(&parent) {
            connected |= children.insert(child.clone());
        } else {
            let mut children = BTreeSet::<Child>::new();
            connected |= children.insert(child.clone());
            self.children.insert(parent.clone(), children);
        }

        if let Some(parents) = self.parents.get_mut(&child) {
            connected |= parents.insert(parent);
        } else {
            let mut parents = BTreeSet::<Parent>::new();
            connected |= parents.insert(parent);
            self.parents.insert(child, parents);
        }

        connected
    }
}

impl<Parent: Clone + Hash + Ord, Child: Clone + Hash + Ord>
    MultiParentBidirectedEdges<Parent, Child>
{
    pub fn new() -> Self {
        Self {
            parents: HashMap::<Child, BTreeSet<Parent>>::new(),
            children: HashMap::<Parent, BTreeSet<Child>>::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_parent_bidirected_edges() {
        let mut edges = SingleParentBidirectedEdges::<u32, u32>::new();

        assert_eq!(edges.num_parents(), 0);
        assert_eq!(edges.num_children(), 0);

        assert!(!edges.disconnect(&1, &5));

        assert!(edges.connect(1, 5));

        assert!(!edges.connect(1, 5));

        assert_eq!(edges.num_parents(), 1);
        assert_eq!(edges.num_children(), 1);

        assert!(edges.disconnect(&1, &5));

        assert_eq!(edges.num_parents(), 0);
        assert_eq!(edges.num_children(), 0);

        // /0/1/2/3
        // | | | /4
        // | | /5/6
        // | /7/8/9
        // | | /10
        // /11/12/13
        // |  /14

        assert!(edges.connect(0, 1));
        assert!(edges.connect(1, 2));
        assert!(edges.connect(2, 3));
        assert!(edges.connect(2, 4));
        assert!(edges.connect(1, 5));
        assert!(edges.connect(5, 6));
        assert!(edges.connect(0, 7));
        assert!(edges.connect(7, 8));
        assert!(edges.connect(7, 10));
        assert!(edges.connect(8, 9));
        assert!(edges.connect(11, 12));
        assert!(edges.connect(11, 14));
        assert!(edges.connect(12, 13));

        assert_eq!(
            vec![1, 7],
            edges.iter_children(&0).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![2, 5],
            edges.iter_children(&1).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![3, 4],
            edges.iter_children(&2).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&3).next().is_none());
        assert!(!edges.has_child(&3));
        assert!(edges.iter_children(&4).next().is_none());
        assert!(!edges.has_child(&4));
        assert_eq!(
            vec![6],
            edges.iter_children(&5).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&6).next().is_none());
        assert!(!edges.has_child(&6));
        assert_eq!(
            vec![8, 10],
            edges.iter_children(&7).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![9],
            edges.iter_children(&8).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&9).next().is_none());
        assert!(!edges.has_child(&9));
        assert!(edges.iter_children(&10).next().is_none());
        assert!(!edges.has_child(&10));
        assert_eq!(
            vec![12, 14],
            edges.iter_children(&11).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![13],
            edges.iter_children(&12).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&13).next().is_none());
        assert!(!edges.has_child(&13));
        assert!(edges.iter_children(&14).next().is_none());
        assert!(!edges.has_child(&14));

        assert!(edges.iter_parents(&0).next().is_none());
        assert!(!edges.has_parent(&0));
        assert_eq!(0, *edges.iter_parents(&1).next().unwrap());
        assert_eq!(1, *edges.iter_parents(&2).next().unwrap());
        assert_eq!(2, *edges.iter_parents(&3).next().unwrap());
        assert_eq!(2, *edges.iter_parents(&4).next().unwrap());
        assert_eq!(1, *edges.iter_parents(&5).next().unwrap());
        assert_eq!(5, *edges.iter_parents(&6).next().unwrap());
        assert_eq!(0, *edges.iter_parents(&7).next().unwrap());
        assert_eq!(7, *edges.iter_parents(&8).next().unwrap());
        assert_eq!(8, *edges.iter_parents(&9).next().unwrap());
        assert_eq!(7, *edges.iter_parents(&10).next().unwrap());
        assert!(edges.iter_parents(&11).next().is_none());
        assert!(!edges.has_parent(&11));
        assert_eq!(11, *edges.iter_parents(&12).next().unwrap());
        assert_eq!(12, *edges.iter_parents(&13).next().unwrap());
        assert_eq!(11, *edges.iter_parents(&14).next().unwrap());

        assert!(edges.parent(&0).is_none());
        assert_eq!(0, *edges.parent(&1).unwrap());
        assert_eq!(1, *edges.parent(&2).unwrap());
        assert_eq!(2, *edges.parent(&3).unwrap());
        assert_eq!(2, *edges.parent(&4).unwrap());
        assert_eq!(1, *edges.parent(&5).unwrap());
        assert_eq!(5, *edges.parent(&6).unwrap());
        assert_eq!(0, *edges.parent(&7).unwrap());
        assert_eq!(7, *edges.parent(&8).unwrap());
        assert_eq!(8, *edges.parent(&9).unwrap());
        assert_eq!(7, *edges.parent(&10).unwrap());
        assert!(edges.parent(&11).is_none());
        assert_eq!(11, *edges.parent(&12).unwrap());
        assert_eq!(12, *edges.parent(&13).unwrap());
        assert_eq!(11, *edges.parent(&14).unwrap());

        // /0/1/5/6
        // | /7/8/9
        // | | /10
        // /11/2/3
        // |  /12/13
        // |  | /4
        // |  /14

        assert!(edges.connect(11, 2));

        assert_eq!(
            vec![1, 7],
            edges.iter_children(&0).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![5],
            edges.iter_children(&1).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![3, 4],
            edges.iter_children(&2).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&3).next().is_none());
        assert!(!edges.has_child(&3));
        assert!(edges.iter_children(&4).next().is_none());
        assert!(!edges.has_child(&4));
        assert_eq!(
            vec![6],
            edges.iter_children(&5).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&6).next().is_none());
        assert!(!edges.has_child(&6));
        assert_eq!(
            vec![8, 10],
            edges.iter_children(&7).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![9],
            edges.iter_children(&8).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&9).next().is_none());
        assert!(!edges.has_child(&9));
        assert!(edges.iter_children(&10).next().is_none());
        assert!(!edges.has_child(&10));
        assert_eq!(
            vec![2, 12, 14],
            edges.iter_children(&11).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![13],
            edges.iter_children(&12).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&13).next().is_none());
        assert!(!edges.has_child(&13));
        assert!(edges.iter_children(&14).next().is_none());
        assert!(!edges.has_child(&14));

        assert!(edges.parent(&0).is_none());
        assert_eq!(0, *edges.parent(&1).unwrap());
        assert_eq!(11, *edges.parent(&2).unwrap());
        assert_eq!(11, *edges.iter_parents(&2).next().unwrap());
        assert_eq!(2, *edges.parent(&3).unwrap());
        assert_eq!(2, *edges.parent(&4).unwrap());
        assert_eq!(1, *edges.parent(&5).unwrap());
        assert_eq!(5, *edges.parent(&6).unwrap());
        assert_eq!(0, *edges.parent(&7).unwrap());
        assert_eq!(7, *edges.parent(&8).unwrap());
        assert_eq!(8, *edges.parent(&9).unwrap());
        assert_eq!(7, *edges.parent(&10).unwrap());
        assert!(edges.parent(&11).is_none());
        assert_eq!(11, *edges.parent(&12).unwrap());
        assert_eq!(12, *edges.parent(&13).unwrap());
        assert_eq!(11, *edges.parent(&14).unwrap());

        // /0/1/5/6
        // | /7/8/9
        // | | /10
        // /2/3
        // | /4
        // /11/12/13
        // |  /14

        assert!(edges.disconnect(&11, &2));

        assert_eq!(
            vec![1, 7],
            edges.iter_children(&0).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![5],
            edges.iter_children(&1).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![3, 4],
            edges.iter_children(&2).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&3).next().is_none());
        assert!(!edges.has_child(&3));
        assert!(edges.iter_children(&4).next().is_none());
        assert!(!edges.has_child(&4));
        assert_eq!(
            vec![6],
            edges.iter_children(&5).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&6).next().is_none());
        assert!(!edges.has_child(&6));
        assert_eq!(
            vec![8, 10],
            edges.iter_children(&7).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![9],
            edges.iter_children(&8).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&9).next().is_none());
        assert!(!edges.has_child(&9));
        assert!(edges.iter_children(&10).next().is_none());
        assert!(!edges.has_child(&10));
        assert_eq!(
            vec![12, 14],
            edges.iter_children(&11).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![13],
            edges.iter_children(&12).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&13).next().is_none());
        assert!(!edges.has_child(&13));
        assert!(edges.iter_children(&14).next().is_none());
        assert!(!edges.has_child(&14));

        assert!(edges.parent(&0).is_none());
        assert_eq!(0, *edges.parent(&1).unwrap());
        assert!(edges.parent(&2).is_none());
        assert!(edges.iter_parents(&2).next().is_none());
        assert!(!edges.has_parent(&2));
        assert_eq!(2, *edges.parent(&3).unwrap());
        assert_eq!(2, *edges.parent(&4).unwrap());
        assert_eq!(1, *edges.parent(&5).unwrap());
        assert_eq!(5, *edges.parent(&6).unwrap());
        assert_eq!(0, *edges.parent(&7).unwrap());
        assert_eq!(7, *edges.parent(&8).unwrap());
        assert_eq!(8, *edges.parent(&9).unwrap());
        assert_eq!(7, *edges.parent(&10).unwrap());
        assert!(edges.parent(&11).is_none());
        assert_eq!(11, *edges.parent(&12).unwrap());
        assert_eq!(12, *edges.parent(&13).unwrap());
        assert_eq!(11, *edges.parent(&14).unwrap());

        // /0/1/5/6
        // | /7/8/9
        // | | /10
        // /2/3
        // | /4
        // /11
        // /12/13
        // /14

        assert_eq!(
            vec![12, 14],
            edges.disconnect_children_of_parent(&11).collect::<Vec<_>>()
        );

        assert!(edges.iter_children(&11).next().is_none());
        assert!(!edges.has_child(&11));
        assert!(edges.parent(&11).is_none());

        assert_eq!(
            vec![1, 7],
            edges.iter_children(&0).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![5],
            edges.iter_children(&1).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![3, 4],
            edges.iter_children(&2).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&3).next().is_none());
        assert!(!edges.has_child(&3));
        assert!(edges.iter_children(&4).next().is_none());
        assert!(!edges.has_child(&4));
        assert_eq!(
            vec![6],
            edges.iter_children(&5).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&6).next().is_none());
        assert!(!edges.has_child(&6));
        assert_eq!(
            vec![8, 10],
            edges.iter_children(&7).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![9],
            edges.iter_children(&8).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&9).next().is_none());
        assert!(!edges.has_child(&9));
        assert!(edges.iter_children(&10).next().is_none());
        assert!(!edges.has_child(&10));
        assert_eq!(
            vec![13],
            edges.iter_children(&12).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&13).next().is_none());
        assert!(!edges.has_child(&13));
        assert!(edges.iter_children(&14).next().is_none());
        assert!(!edges.has_child(&14));

        assert!(edges.parent(&0).is_none());
        assert_eq!(0, *edges.parent(&1).unwrap());
        assert!(edges.parent(&2).is_none());
        assert!(edges.iter_parents(&2).next().is_none());
        assert!(!edges.has_parent(&2));
        assert_eq!(2, *edges.parent(&3).unwrap());
        assert_eq!(2, *edges.parent(&4).unwrap());
        assert_eq!(1, *edges.parent(&5).unwrap());
        assert_eq!(5, *edges.parent(&6).unwrap());
        assert_eq!(0, *edges.parent(&7).unwrap());
        assert_eq!(7, *edges.parent(&8).unwrap());
        assert_eq!(8, *edges.parent(&9).unwrap());
        assert_eq!(7, *edges.parent(&10).unwrap());
        assert!(edges.parent(&11).is_none());
        assert!(edges.parent(&12).is_none());
        assert_eq!(12, *edges.parent(&13).unwrap());
        assert!(edges.parent(&14).is_none());

        // /0/1/5/6
        // | /7/10
        // /2/3
        // | /4
        // /8/9
        // /11
        // /12/13
        // /14

        assert_eq!(7, edges.disconnect_parent_of_child(&8).unwrap());

        assert!(edges.iter_children(&11).next().is_none());
        assert!(!edges.has_child(&11));
        assert!(edges.parent(&11).is_none());

        assert_eq!(
            vec![1, 7],
            edges.iter_children(&0).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![5],
            edges.iter_children(&1).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![3, 4],
            edges.iter_children(&2).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&3).next().is_none());
        assert!(!edges.has_child(&3));
        assert!(edges.iter_children(&4).next().is_none());
        assert!(!edges.has_child(&4));
        assert_eq!(
            vec![6],
            edges.iter_children(&5).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&6).next().is_none());
        assert!(!edges.has_child(&6));
        assert_eq!(
            vec![10],
            edges.iter_children(&7).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![9],
            edges.iter_children(&8).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&9).next().is_none());
        assert!(!edges.has_child(&9));
        assert!(edges.iter_children(&10).next().is_none());
        assert!(!edges.has_child(&10));
        assert_eq!(
            vec![13],
            edges.iter_children(&12).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&13).next().is_none());
        assert!(!edges.has_child(&13));
        assert!(edges.iter_children(&14).next().is_none());
        assert!(!edges.has_child(&14));

        assert!(edges.parent(&0).is_none());
        assert_eq!(0, *edges.parent(&1).unwrap());
        assert!(edges.parent(&2).is_none());
        assert!(edges.iter_parents(&2).next().is_none());
        assert!(!edges.has_parent(&2));
        assert_eq!(2, *edges.parent(&3).unwrap());
        assert_eq!(2, *edges.parent(&4).unwrap());
        assert_eq!(1, *edges.parent(&5).unwrap());
        assert_eq!(5, *edges.parent(&6).unwrap());
        assert_eq!(0, *edges.parent(&7).unwrap());
        assert!(edges.parent(&8).is_none());
        assert_eq!(8, *edges.parent(&9).unwrap());
        assert_eq!(7, *edges.parent(&10).unwrap());
        assert!(edges.parent(&11).is_none());
        assert!(edges.parent(&12).is_none());
        assert_eq!(12, *edges.parent(&13).unwrap());
        assert!(edges.parent(&14).is_none());

        // /0/1
        // | /7
        // /2/3
        // | /4
        // /5/6
        // /10
        // /8/9
        // /11
        // /12/13
        // /14

        assert_eq!(
            vec![(1, &5), (7, &10)],
            edges
                .disconnect_parents_of_children(vec![5, 10].iter())
                .collect::<Vec<_>>()
        );

        assert!(edges.iter_children(&11).next().is_none());
        assert!(!edges.has_child(&11));
        assert!(edges.parent(&11).is_none());

        assert_eq!(
            vec![1, 7],
            edges.iter_children(&0).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&1).next().is_none());
        assert!(!edges.has_child(&1));
        assert_eq!(
            vec![3, 4],
            edges.iter_children(&2).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&3).next().is_none());
        assert!(!edges.has_child(&3));
        assert!(edges.iter_children(&4).next().is_none());
        assert!(!edges.has_child(&4));
        assert_eq!(
            vec![6],
            edges.iter_children(&5).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&6).next().is_none());
        assert!(!edges.has_child(&6));
        assert!(edges.iter_children(&7).next().is_none());
        assert!(!edges.has_child(&7));
        assert_eq!(
            vec![9],
            edges.iter_children(&8).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&9).next().is_none());
        assert!(!edges.has_child(&9));
        assert!(edges.iter_children(&10).next().is_none());
        assert!(!edges.has_child(&10));
        assert_eq!(
            vec![13],
            edges.iter_children(&12).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&13).next().is_none());
        assert!(!edges.has_child(&13));
        assert!(edges.iter_children(&14).next().is_none());
        assert!(!edges.has_child(&14));

        assert!(edges.parent(&0).is_none());
        assert_eq!(0, *edges.parent(&1).unwrap());
        assert!(edges.parent(&2).is_none());
        assert!(edges.iter_parents(&2).next().is_none());
        assert!(!edges.has_parent(&2));
        assert_eq!(2, *edges.parent(&3).unwrap());
        assert_eq!(2, *edges.parent(&4).unwrap());
        assert!(edges.parent(&5).is_none());
        assert_eq!(5, *edges.parent(&6).unwrap());
        assert_eq!(0, *edges.parent(&7).unwrap());
        assert!(edges.parent(&8).is_none());
        assert_eq!(8, *edges.parent(&9).unwrap());
        assert!(edges.parent(&10).is_none());
        assert!(edges.parent(&11).is_none());
        assert!(edges.parent(&12).is_none());
        assert_eq!(12, *edges.parent(&13).unwrap());
        assert!(edges.parent(&14).is_none());

        // /0/1
        // | /7
        // /2
        // /3
        // /4
        // /5
        // /6
        // /10
        // /8/9
        // /11
        // /12/13
        // /14

        assert_eq!(
            vec![(&2, 3), (&2, 4), (&5, 6)],
            edges
                .disconnect_children_of_parents(vec![2, 5].iter())
                .collect::<Vec<_>>()
        );

        assert!(edges.iter_children(&11).next().is_none());
        assert!(!edges.has_child(&11));
        assert!(edges.parent(&11).is_none());

        assert_eq!(
            vec![1, 7],
            edges.iter_children(&0).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&1).next().is_none());
        assert!(!edges.has_child(&1));
        assert!(edges.iter_children(&2).next().is_none());
        assert!(!edges.has_child(&2));
        assert!(edges.iter_children(&3).next().is_none());
        assert!(!edges.has_child(&3));
        assert!(edges.iter_children(&4).next().is_none());
        assert!(!edges.has_child(&4));
        assert!(edges.iter_children(&5).next().is_none());
        assert!(!edges.has_child(&5));
        assert!(edges.iter_children(&6).next().is_none());
        assert!(!edges.has_child(&6));
        assert!(edges.iter_children(&7).next().is_none());
        assert!(!edges.has_child(&7));
        assert_eq!(
            vec![9],
            edges.iter_children(&8).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&9).next().is_none());
        assert!(!edges.has_child(&9));
        assert!(edges.iter_children(&10).next().is_none());
        assert!(!edges.has_child(&10));
        assert_eq!(
            vec![13],
            edges.iter_children(&12).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&13).next().is_none());
        assert!(!edges.has_child(&13));
        assert!(edges.iter_children(&14).next().is_none());
        assert!(!edges.has_child(&14));

        assert!(edges.parent(&0).is_none());
        assert_eq!(0, *edges.parent(&1).unwrap());
        assert!(edges.parent(&2).is_none());
        assert!(edges.iter_parents(&2).next().is_none());
        assert!(!edges.has_parent(&2));
        assert!(edges.parent(&3).is_none());
        assert!(edges.parent(&4).is_none());
        assert!(edges.parent(&5).is_none());
        assert!(edges.parent(&6).is_none());
        assert_eq!(0, *edges.parent(&7).unwrap());
        assert!(edges.parent(&8).is_none());
        assert_eq!(8, *edges.parent(&9).unwrap());
        assert!(edges.parent(&10).is_none());
        assert!(edges.parent(&11).is_none());
        assert!(edges.parent(&12).is_none());
        assert_eq!(12, *edges.parent(&13).unwrap());
        assert!(edges.parent(&14).is_none());
    }

    #[test]
    fn test_multi_parent_bidirected_edges() {
        let mut edges = MultiParentBidirectedEdges::<u32, u32>::new();

        assert_eq!(edges.num_parents(), 0);
        assert_eq!(edges.num_children(), 0);

        assert!(!edges.disconnect(&1, &5));

        assert!(edges.connect(1, 5));

        assert!(!edges.connect(1, 5));

        assert_eq!(edges.num_parents(), 1);
        assert_eq!(edges.num_children(), 1);

        assert!(edges.disconnect(&1, &5));

        assert_eq!(edges.num_parents(), 0);
        assert_eq!(edges.num_children(), 0);

        // /0/1/2/3
        // | | | /4
        // | | /5/6
        // | /7/8/9
        // | | /10
        // /11/12/13
        // |  /14

        assert!(edges.connect(0, 1));
        assert!(edges.connect(1, 2));
        assert!(edges.connect(2, 3));
        assert!(edges.connect(2, 4));
        assert!(edges.connect(1, 5));
        assert!(edges.connect(5, 6));
        assert!(edges.connect(0, 7));
        assert!(edges.connect(7, 8));
        assert!(edges.connect(7, 10));
        assert!(edges.connect(8, 9));
        assert!(edges.connect(11, 12));
        assert!(edges.connect(11, 14));
        assert!(edges.connect(12, 13));

        assert_eq!(
            vec![1, 7],
            edges.iter_children(&0).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![2, 5],
            edges.iter_children(&1).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![3, 4],
            edges.iter_children(&2).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&3).next().is_none());
        assert!(!edges.has_child(&3));
        assert!(edges.iter_children(&4).next().is_none());
        assert!(!edges.has_child(&4));
        assert_eq!(
            vec![6],
            edges.iter_children(&5).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&6).next().is_none());
        assert!(!edges.has_child(&6));
        assert_eq!(
            vec![8, 10],
            edges.iter_children(&7).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![9],
            edges.iter_children(&8).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&9).next().is_none());
        assert!(!edges.has_child(&9));
        assert!(edges.iter_children(&10).next().is_none());
        assert!(!edges.has_child(&10));
        assert_eq!(
            vec![12, 14],
            edges.iter_children(&11).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![13],
            edges.iter_children(&12).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&13).next().is_none());
        assert!(!edges.has_child(&13));
        assert!(edges.iter_children(&14).next().is_none());
        assert!(!edges.has_child(&14));

        assert!(edges.iter_parents(&0).next().is_none());
        assert!(!edges.has_parent(&0));
        assert_eq!(0, *edges.iter_parents(&1).next().unwrap());
        assert_eq!(1, *edges.iter_parents(&2).next().unwrap());
        assert_eq!(2, *edges.iter_parents(&3).next().unwrap());
        assert_eq!(2, *edges.iter_parents(&4).next().unwrap());
        assert_eq!(1, *edges.iter_parents(&5).next().unwrap());
        assert_eq!(5, *edges.iter_parents(&6).next().unwrap());
        assert_eq!(0, *edges.iter_parents(&7).next().unwrap());
        assert_eq!(7, *edges.iter_parents(&8).next().unwrap());
        assert_eq!(8, *edges.iter_parents(&9).next().unwrap());
        assert_eq!(7, *edges.iter_parents(&10).next().unwrap());
        assert!(edges.iter_parents(&11).next().is_none());
        assert!(!edges.has_parent(&11));
        assert_eq!(11, *edges.iter_parents(&12).next().unwrap());
        assert_eq!(12, *edges.iter_parents(&13).next().unwrap());
        assert_eq!(11, *edges.iter_parents(&14).next().unwrap());

        // /0/1/2/3
        // | | | /4
        // | | /5/6
        // | /7/8/9
        // | | /10
        // /11/2/3
        // |  | /4
        // |  /12/13
        // |  /14

        assert!(edges.connect(11, 2));

        assert_eq!(
            vec![1, 7],
            edges.iter_children(&0).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![2, 5],
            edges.iter_children(&1).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![3, 4],
            edges.iter_children(&2).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&3).next().is_none());
        assert!(!edges.has_child(&3));
        assert!(edges.iter_children(&4).next().is_none());
        assert!(!edges.has_child(&4));
        assert_eq!(
            vec![6],
            edges.iter_children(&5).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&6).next().is_none());
        assert!(!edges.has_child(&6));
        assert_eq!(
            vec![8, 10],
            edges.iter_children(&7).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![9],
            edges.iter_children(&8).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&9).next().is_none());
        assert!(!edges.has_child(&9));
        assert!(edges.iter_children(&10).next().is_none());
        assert!(!edges.has_child(&10));
        assert_eq!(
            vec![2, 12, 14],
            edges.iter_children(&11).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![13],
            edges.iter_children(&12).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&13).next().is_none());
        assert!(!edges.has_child(&13));
        assert!(edges.iter_children(&14).next().is_none());
        assert!(!edges.has_child(&14));

        assert!(edges.iter_parents(&0).next().is_none());
        assert!(!edges.has_parent(&0));
        assert_eq!(0, *edges.iter_parents(&1).next().unwrap());
        assert_eq!(vec![&1, &11], edges.iter_parents(&2).collect::<Vec<_>>());
        assert_eq!(2, *edges.iter_parents(&3).next().unwrap());
        assert_eq!(2, *edges.iter_parents(&4).next().unwrap());
        assert_eq!(1, *edges.iter_parents(&5).next().unwrap());
        assert_eq!(5, *edges.iter_parents(&6).next().unwrap());
        assert_eq!(0, *edges.iter_parents(&7).next().unwrap());
        assert_eq!(7, *edges.iter_parents(&8).next().unwrap());
        assert_eq!(8, *edges.iter_parents(&9).next().unwrap());
        assert_eq!(7, *edges.iter_parents(&10).next().unwrap());
        assert!(edges.iter_parents(&11).next().is_none());
        assert!(!edges.has_parent(&11));
        assert_eq!(11, *edges.iter_parents(&12).next().unwrap());
        assert_eq!(12, *edges.iter_parents(&13).next().unwrap());
        assert_eq!(11, *edges.iter_parents(&14).next().unwrap());

        // /0/1/2/3
        // | | | /4
        // | | /5/6
        // | /7/8/9
        // | | /10
        // /11/12/13
        // |  /14

        assert!(edges.disconnect(&11, &2));

        assert_eq!(
            vec![1, 7],
            edges.iter_children(&0).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![2, 5],
            edges.iter_children(&1).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![3, 4],
            edges.iter_children(&2).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&3).next().is_none());
        assert!(!edges.has_child(&3));
        assert!(edges.iter_children(&4).next().is_none());
        assert!(!edges.has_child(&4));
        assert_eq!(
            vec![6],
            edges.iter_children(&5).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&6).next().is_none());
        assert!(!edges.has_child(&6));
        assert_eq!(
            vec![8, 10],
            edges.iter_children(&7).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![9],
            edges.iter_children(&8).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&9).next().is_none());
        assert!(!edges.has_child(&9));
        assert!(edges.iter_children(&10).next().is_none());
        assert!(!edges.has_child(&10));
        assert_eq!(
            vec![12, 14],
            edges.iter_children(&11).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![13],
            edges.iter_children(&12).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&13).next().is_none());
        assert!(!edges.has_child(&13));
        assert!(edges.iter_children(&14).next().is_none());
        assert!(!edges.has_child(&14));

        assert!(edges.iter_parents(&0).next().is_none());
        assert!(!edges.has_parent(&0));
        assert_eq!(0, *edges.iter_parents(&1).next().unwrap());
        assert_eq!(1, *edges.iter_parents(&2).next().unwrap());
        assert_eq!(2, *edges.iter_parents(&3).next().unwrap());
        assert_eq!(2, *edges.iter_parents(&4).next().unwrap());
        assert_eq!(1, *edges.iter_parents(&5).next().unwrap());
        assert_eq!(5, *edges.iter_parents(&6).next().unwrap());
        assert_eq!(0, *edges.iter_parents(&7).next().unwrap());
        assert_eq!(7, *edges.iter_parents(&8).next().unwrap());
        assert_eq!(8, *edges.iter_parents(&9).next().unwrap());
        assert_eq!(7, *edges.iter_parents(&10).next().unwrap());
        assert!(edges.iter_parents(&11).next().is_none());
        assert!(!edges.has_parent(&11));
        assert_eq!(11, *edges.iter_parents(&12).next().unwrap());
        assert_eq!(12, *edges.iter_parents(&13).next().unwrap());
        assert_eq!(11, *edges.iter_parents(&14).next().unwrap());

        // /0/1/2/3
        // | | | /4
        // | | /5/6
        // | /7/8/9
        // | | /10
        // /11
        // /12/13
        // /14

        assert_eq!(
            vec![12, 14],
            edges.disconnect_children_of_parent(&11).collect::<Vec<_>>()
        );

        assert!(edges.iter_children(&11).next().is_none());
        assert!(!edges.has_child(&11));
        assert!(edges.iter_parents(&11).next().is_none());
        assert!(!edges.has_parent(&11));

        assert_eq!(
            vec![1, 7],
            edges.iter_children(&0).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![2, 5],
            edges.iter_children(&1).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![3, 4],
            edges.iter_children(&2).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&3).next().is_none());
        assert!(!edges.has_child(&3));
        assert!(edges.iter_children(&4).next().is_none());
        assert!(!edges.has_child(&4));
        assert_eq!(
            vec![6],
            edges.iter_children(&5).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&6).next().is_none());
        assert!(!edges.has_child(&6));
        assert_eq!(
            vec![8, 10],
            edges.iter_children(&7).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![9],
            edges.iter_children(&8).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&9).next().is_none());
        assert!(!edges.has_child(&9));
        assert!(edges.iter_children(&10).next().is_none());
        assert!(!edges.has_child(&10));
        assert_eq!(
            vec![13],
            edges.iter_children(&12).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&13).next().is_none());
        assert!(!edges.has_child(&13));
        assert!(edges.iter_children(&14).next().is_none());
        assert!(!edges.has_child(&14));

        assert!(edges.iter_parents(&0).next().is_none());
        assert!(!edges.has_parent(&0));
        assert_eq!(0, *edges.iter_parents(&1).next().unwrap());
        assert_eq!(1, *edges.iter_parents(&2).next().unwrap());
        assert_eq!(2, *edges.iter_parents(&3).next().unwrap());
        assert_eq!(2, *edges.iter_parents(&4).next().unwrap());
        assert_eq!(1, *edges.iter_parents(&5).next().unwrap());
        assert_eq!(5, *edges.iter_parents(&6).next().unwrap());
        assert_eq!(0, *edges.iter_parents(&7).next().unwrap());
        assert_eq!(7, *edges.iter_parents(&8).next().unwrap());
        assert_eq!(8, *edges.iter_parents(&9).next().unwrap());
        assert_eq!(7, *edges.iter_parents(&10).next().unwrap());
        assert!(edges.iter_parents(&11).next().is_none());
        assert!(!edges.has_parent(&11));
        assert!(edges.iter_parents(&12).next().is_none());
        assert!(!edges.has_parent(&12));
        assert_eq!(12, *edges.iter_parents(&13).next().unwrap());
        assert!(edges.iter_parents(&14).next().is_none());
        assert!(!edges.has_parent(&14));

        // /0/1/5/6
        // | /7/8/9
        // | | /10
        // /2/3
        // | /4
        // /11
        // /12/13
        // /14

        assert!(edges.connect(11, 2));
        assert_eq!(
            vec![1, 11],
            edges.disconnect_parents_of_child(&2).collect::<Vec<_>>()
        );

        assert_eq!(
            vec![1, 7],
            edges.iter_children(&0).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![5],
            edges.iter_children(&1).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![3, 4],
            edges.iter_children(&2).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&3).next().is_none());
        assert!(!edges.has_child(&3));
        assert!(edges.iter_children(&4).next().is_none());
        assert!(!edges.has_child(&4));
        assert_eq!(
            vec![6],
            edges.iter_children(&5).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&6).next().is_none());
        assert!(!edges.has_child(&6));
        assert_eq!(
            vec![8, 10],
            edges.iter_children(&7).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![9],
            edges.iter_children(&8).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&9).next().is_none());
        assert!(!edges.has_child(&9));
        assert!(edges.iter_children(&10).next().is_none());
        assert!(!edges.has_child(&10));
        assert!(edges.iter_children(&11).next().is_none());
        assert!(!edges.has_child(&11));
        assert_eq!(
            vec![13],
            edges.iter_children(&12).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&13).next().is_none());
        assert!(!edges.has_child(&13));
        assert!(edges.iter_children(&14).next().is_none());
        assert!(!edges.has_child(&14));

        assert!(edges.iter_parents(&0).next().is_none());
        assert!(!edges.has_parent(&0));
        assert_eq!(0, *edges.iter_parents(&1).next().unwrap());
        assert!(edges.iter_parents(&2).next().is_none());
        assert!(!edges.has_parent(&2));
        assert_eq!(2, *edges.iter_parents(&3).next().unwrap());
        assert_eq!(2, *edges.iter_parents(&4).next().unwrap());
        assert_eq!(1, *edges.iter_parents(&5).next().unwrap());
        assert_eq!(5, *edges.iter_parents(&6).next().unwrap());
        assert_eq!(0, *edges.iter_parents(&7).next().unwrap());
        assert_eq!(7, *edges.iter_parents(&8).next().unwrap());
        assert_eq!(8, *edges.iter_parents(&9).next().unwrap());
        assert_eq!(7, *edges.iter_parents(&10).next().unwrap());
        assert!(edges.iter_parents(&11).next().is_none());
        assert!(!edges.has_parent(&11));
        assert!(edges.iter_parents(&12).next().is_none());
        assert!(!edges.has_parent(&12));
        assert_eq!(12, *edges.iter_parents(&13).next().unwrap());
        assert!(edges.iter_parents(&14).next().is_none());
        assert!(!edges.has_parent(&14));

        // /0/1
        // | /7/8/9
        // /2/3
        // | /4
        // /5/6
        // /10
        // /11
        // /12/13
        // /14

        assert_eq!(
            vec![(1, &5), (7, &10)],
            edges
                .disconnect_parents_of_children(vec![5, 10].iter())
                .collect::<Vec<_>>()
        );

        assert_eq!(
            vec![1, 7],
            edges.iter_children(&0).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&1).next().is_none());
        assert!(!edges.has_child(&1));
        assert_eq!(
            vec![3, 4],
            edges.iter_children(&2).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&3).next().is_none());
        assert!(!edges.has_child(&3));
        assert!(edges.iter_children(&4).next().is_none());
        assert!(!edges.has_child(&4));
        assert_eq!(
            vec![6],
            edges.iter_children(&5).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&6).next().is_none());
        assert!(!edges.has_child(&6));
        assert_eq!(
            vec![8],
            edges.iter_children(&7).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![9],
            edges.iter_children(&8).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&9).next().is_none());
        assert!(!edges.has_child(&9));
        assert!(edges.iter_children(&10).next().is_none());
        assert!(!edges.has_child(&10));
        assert!(edges.iter_children(&11).next().is_none());
        assert!(!edges.has_child(&11));
        assert_eq!(
            vec![13],
            edges.iter_children(&12).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&13).next().is_none());
        assert!(!edges.has_child(&13));
        assert!(edges.iter_children(&14).next().is_none());
        assert!(!edges.has_child(&14));

        assert!(edges.iter_parents(&0).next().is_none());
        assert!(!edges.has_parent(&0));
        assert_eq!(0, *edges.iter_parents(&1).next().unwrap());
        assert!(edges.iter_parents(&2).next().is_none());
        assert!(!edges.has_parent(&2));
        assert_eq!(2, *edges.iter_parents(&3).next().unwrap());
        assert_eq!(2, *edges.iter_parents(&4).next().unwrap());
        assert!(edges.iter_parents(&5).next().is_none());
        assert!(!edges.has_parent(&5));
        assert_eq!(5, *edges.iter_parents(&6).next().unwrap());
        assert_eq!(0, *edges.iter_parents(&7).next().unwrap());
        assert_eq!(7, *edges.iter_parents(&8).next().unwrap());
        assert_eq!(8, *edges.iter_parents(&9).next().unwrap());
        assert!(edges.iter_parents(&10).next().is_none());
        assert!(!edges.has_parent(&10));
        assert!(edges.iter_parents(&11).next().is_none());
        assert!(!edges.has_parent(&11));
        assert!(edges.iter_parents(&12).next().is_none());
        assert!(!edges.has_parent(&12));
        assert_eq!(12, *edges.iter_parents(&13).next().unwrap());
        assert!(edges.iter_parents(&14).next().is_none());
        assert!(!edges.has_parent(&14));

        // /0/1
        // | /7/8/9
        // /2
        // /3
        // /4
        // /5
        // /6
        // /10
        // /11
        // /12/13
        // /14

        assert_eq!(
            vec![(&2, 3), (&2, 4), (&5, 6)],
            edges
                .disconnect_children_of_parents(vec![2, 5].iter())
                .collect::<Vec<_>>()
        );

        assert_eq!(
            vec![1, 7],
            edges.iter_children(&0).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&1).next().is_none());
        assert!(!edges.has_child(&1));
        assert!(edges.iter_children(&2).next().is_none());
        assert!(!edges.has_child(&2));
        assert!(edges.iter_children(&3).next().is_none());
        assert!(!edges.has_child(&3));
        assert!(edges.iter_children(&4).next().is_none());
        assert!(!edges.has_child(&4));
        assert!(edges.iter_children(&5).next().is_none());
        assert!(!edges.has_child(&5));
        assert!(edges.iter_children(&6).next().is_none());
        assert!(!edges.has_child(&6));
        assert_eq!(
            vec![8],
            edges.iter_children(&7).copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![9],
            edges.iter_children(&8).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&9).next().is_none());
        assert!(!edges.has_child(&9));
        assert!(edges.iter_children(&10).next().is_none());
        assert!(!edges.has_child(&10));
        assert!(edges.iter_children(&11).next().is_none());
        assert!(!edges.has_child(&11));
        assert_eq!(
            vec![13],
            edges.iter_children(&12).copied().collect::<Vec<_>>()
        );
        assert!(edges.iter_children(&13).next().is_none());
        assert!(!edges.has_child(&13));
        assert!(edges.iter_children(&14).next().is_none());
        assert!(!edges.has_child(&14));

        assert!(edges.iter_parents(&0).next().is_none());
        assert!(!edges.has_parent(&0));
        assert_eq!(0, *edges.iter_parents(&1).next().unwrap());
        assert!(edges.iter_parents(&2).next().is_none());
        assert!(!edges.has_parent(&2));
        assert!(edges.iter_parents(&3).next().is_none());
        assert!(!edges.has_parent(&3));
        assert!(edges.iter_parents(&4).next().is_none());
        assert!(!edges.has_parent(&4));
        assert!(edges.iter_parents(&5).next().is_none());
        assert!(!edges.has_parent(&5));
        assert!(edges.iter_parents(&6).next().is_none());
        assert!(!edges.has_parent(&6));
        assert_eq!(0, *edges.iter_parents(&7).next().unwrap());
        assert_eq!(7, *edges.iter_parents(&8).next().unwrap());
        assert_eq!(8, *edges.iter_parents(&9).next().unwrap());
        assert!(edges.iter_parents(&10).next().is_none());
        assert!(!edges.has_parent(&10));
        assert!(edges.iter_parents(&11).next().is_none());
        assert!(!edges.has_parent(&11));
        assert!(edges.iter_parents(&12).next().is_none());
        assert!(!edges.has_parent(&12));
        assert_eq!(12, *edges.iter_parents(&13).next().unwrap());
        assert!(edges.iter_parents(&14).next().is_none());
        assert!(!edges.has_parent(&14));
    }
}
