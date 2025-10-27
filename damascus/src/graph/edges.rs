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
    Parent: Clone + Hash + Ord,
    Child: Clone + Hash + Ord,
{
    fn num_parents(&self) -> usize;
    fn num_children(&self) -> usize;

    fn clear(&mut self);

    fn parents<'a>(&'a self, child: Child) -> impl Iterator<Item = &'a Parent> + 'a
    where
        Parent: 'a;
    fn children<'a>(&'a self, parent: Parent) -> impl Iterator<Item = &'a Child> + 'a
    where
        Child: 'a;

    fn disconnect(&mut self, parent: Parent, child: Child) -> bool;
    fn connect(&mut self, parent: Parent, child: Child);

    fn disconnect_parents_of_child<'a>(
        &'a mut self,
        child: Child,
    ) -> impl Iterator<Item = (Parent, Child)> + 'a
    where
        Parent: 'a,
        Child: 'a;
    fn disconnect_children_of_parent<'a>(
        &'a mut self,
        parent: Parent,
    ) -> impl Iterator<Item = (Parent, Child)> + 'a
    where
        Parent: 'a,
        Child: 'a;

    fn disconnect_children_of_parents<'a>(
        &'a mut self,
        parents: impl Iterator<Item = Parent> + 'a,
    ) -> impl Iterator<Item = (Parent, Child)> + 'a
    where
        Parent: 'a,
        Child: 'a,
    {
        parents.flat_map(|parent| {
            self.disconnect_children_of_parent(parent)
                .collect::<Vec<_>>()
        })
    }

    fn disconnect_parents_of_children<'a>(
        &'a mut self,
        children: impl Iterator<Item = Child> + 'a,
    ) -> impl Iterator<Item = (Parent, Child)> + 'a
    where
        Parent: 'a,
        Child: 'a,
    {
        children.flat_map(|child| self.disconnect_parents_of_child(child).collect::<Vec<_>>())
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SingleParentBidirectedEdges<Parent, Child>
where
    Parent: Copy + Hash + Ord,
    Child: Copy + Hash + Ord,
{
    parents: HashMap<Child, Parent>,
    children: HashMap<Parent, BTreeSet<Child>>,
}

impl<Parent: Copy + Hash + Ord, Child: Copy + Hash + Ord> BidirectedEdges<Parent, Child>
    for SingleParentBidirectedEdges<Parent, Child>
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

    fn parents<'a>(&'a self, child: Child) -> impl Iterator<Item = &'a Parent> + 'a
    where
        Parent: 'a,
    {
        self.parents.get(&child).into_iter()
    }

    fn children<'a>(&'a self, parent: Parent) -> impl Iterator<Item = &'a Child> + 'a
    where
        Child: 'a,
    {
        self.children.get(&parent).into_iter().flatten().into_iter()
    }

    fn disconnect_parents_of_child<'a>(
        &'a mut self,
        child: Child,
    ) -> impl Iterator<Item = (Parent, Child)> + 'a
    where
        Parent: 'a,
        Child: 'a,
    {
        self.parents.remove(&child).into_iter().map(move |parent| {
            let mut all_children_removed = false;
            if let Some(children) = self.children.get_mut(&parent) {
                children.remove(&child);
                all_children_removed = children.len() == 0;
            }
            if all_children_removed {
                self.children.remove(&parent);
            }

            (parent, child)
        })
    }

    fn disconnect_children_of_parent<'a>(
        &'a mut self,
        parent: Parent,
    ) -> impl Iterator<Item = (Parent, Child)> + 'a
    where
        Parent: 'a,
        Child: 'a,
    {
        // Since children can only have this one parent, we can reuse `disconnect_child`
        self.children(parent)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .flat_map(move |child| self.disconnect_parents_of_child(child).collect::<Vec<_>>())
    }

    fn disconnect(&mut self, parent: Parent, child: Child) -> bool {
        let mut disconnected = false;
        if let Some(current_parent) = self.parents.get(&child)
            && parent == *current_parent
        {
            disconnected = self.parents.remove(&child).is_some();

            if let Some(children) = self.children.get_mut(&parent) {
                disconnected |= children.remove(&child);
            }
        }
        disconnected
    }

    fn connect(&mut self, parent: Parent, child: Child) {
        if let Some(current_parent) = self.parent(child).copied() {
            if current_parent == parent {
                return;
            }
            let mut all_children_removed = false;
            if let Some(children) = self.children.get_mut(&current_parent) {
                children.remove(&child);
                all_children_removed = children.len() == 0;
            }
            if all_children_removed {
                self.children.remove(&current_parent);
            }
        }
        self.parents.insert(child, parent);
        if let Some(children) = self.children.get_mut(&parent) {
            children.insert(child);
        } else {
            let mut children = BTreeSet::<Child>::new();
            children.insert(child);
            self.children.insert(parent, children);
        }
    }
}

impl<Parent: Copy + Hash + Ord, Child: Copy + Hash + Ord>
    SingleParentBidirectedEdges<Parent, Child>
{
    pub fn parent(&self, child: Child) -> Option<&Parent> {
        self.parents.get(&child)
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MultiParentBidirectedEdges<Parent, Child>
where
    Parent: Copy + Hash + Ord,
    Child: Copy + Hash + Ord,
{
    parents: HashMap<Child, BTreeSet<Parent>>,
    children: HashMap<Parent, BTreeSet<Child>>,
}

impl<Parent: Copy + Hash + Ord, Child: Copy + Hash + Ord> BidirectedEdges<Parent, Child>
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

    fn parents<'a>(&'a self, child: Child) -> impl Iterator<Item = &'a Parent> + 'a
    where
        Parent: 'a,
    {
        self.parents.get(&child).into_iter().flatten().into_iter()
    }

    fn children<'a>(&'a self, parent: Parent) -> impl Iterator<Item = &'a Child> + 'a
    where
        Child: 'a,
    {
        self.children.get(&parent).into_iter().flatten().into_iter()
    }

    fn disconnect_parents_of_child<'a>(
        &'a mut self,
        child: Child,
    ) -> impl Iterator<Item = (Parent, Child)> + 'a
    where
        Parent: 'a,
        Child: 'a,
    {
        self.parents
            .remove(&child)
            .into_iter()
            .flatten()
            .map(move |parent| {
                let mut all_children_removed = false;
                if let Some(children) = self.children.get_mut(&parent) {
                    children.remove(&child);
                    all_children_removed = children.len() == 0;
                }
                if all_children_removed {
                    self.children.remove(&parent);
                }

                (parent, child)
            })
    }

    fn disconnect_children_of_parent<'a>(
        &'a mut self,
        parent: Parent,
    ) -> impl Iterator<Item = (Parent, Child)> + 'a
    where
        Parent: 'a,
        Child: 'a,
    {
        self.children
            .remove(&parent)
            .into_iter()
            .flatten()
            .map(move |child| {
                let mut all_parents_removed = false;
                if let Some(parents) = self.parents.get_mut(&child) {
                    parents.remove(&parent);
                    all_parents_removed = parents.len() == 0;
                }
                if all_parents_removed {
                    self.parents.remove(&child);
                }

                (parent, child)
            })
    }

    fn disconnect(&mut self, parent: Parent, child: Child) -> bool {
        let mut disconnected = false;
        if let Some(children) = self.children.get_mut(&parent) {
            disconnected |= children.remove(&child);
        }
        if let Some(parents) = self.parents.get_mut(&child) {
            disconnected |= parents.remove(&parent);
        }
        disconnected
    }

    fn connect(&mut self, parent: Parent, child: Child) {
        if let Some(children) = self.children.get_mut(&parent) {
            children.insert(child);
        } else {
            let mut children = BTreeSet::<Child>::new();
            children.insert(child);
            self.children.insert(parent, children);
        }

        if let Some(parents) = self.parents.get_mut(&child) {
            parents.insert(parent);
        } else {
            let mut parents = BTreeSet::<Parent>::new();
            parents.insert(parent);
            self.parents.insert(child, parents);
        }
    }
}
