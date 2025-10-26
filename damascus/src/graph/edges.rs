// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{
    cmp::Ord,
    collections::{BTreeSet, HashMap},
    hash::Hash,
};

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BidirectionalSingleParentEdges<ParentId, ChildId>
where
    ParentId: Ord + Copy + Hash,
    ChildId: Ord + Copy + Hash,
{
    parents: HashMap<ChildId, ParentId>,
    children: HashMap<ParentId, BTreeSet<ChildId>>,
}

impl<ParentId: Ord + Copy + Hash, ChildId: Ord + Copy + Hash>
    BidirectionalSingleParentEdges<ParentId, ChildId>
{
    pub fn len(&self) -> usize {
        self.parents.len()
    }

    pub fn clear(&mut self) {
        self.children.clear();
        self.parents.clear();
    }

    pub fn parent(&self, child_id: ChildId) -> Option<&ParentId> {
        self.parents.get(&child_id)
    }

    pub fn cloned_children(&self, parent_id: ParentId) -> BTreeSet<ChildId> {
        let mut child_ids = BTreeSet::<ChildId>::new();
        if let Some(children) = self.children(parent_id) {
            child_ids = children.clone();
        }
        child_ids
    }

    pub fn has_children(&self, parent_id: ParentId) -> bool {
        self.children.contains_key(&parent_id)
    }

    pub fn children(&self, parent_id: ParentId) -> Option<&BTreeSet<ChildId>> {
        self.children.get(&parent_id)
    }

    pub fn children_mut(&mut self, parent_id: ParentId) -> Option<&mut BTreeSet<ChildId>> {
        self.children.get_mut(&parent_id)
    }

    pub fn disconnect_child(&mut self, child_id: ChildId) -> Option<ParentId> {
        if let Some(parent_id) = self.parents.remove(&child_id) {
            let mut all_children_removed = false;
            if let Some(children) = self.children.get_mut(&parent_id) {
                children.remove(&child_id);
                all_children_removed = children.len() == 0;
            }
            if all_children_removed {
                self.children.remove(&parent_id);
            }
            Some(parent_id)
        } else {
            None
        }
    }

    pub fn disconnect_parent(&mut self, parent_id: ParentId) -> BTreeSet<ChildId> {
        let children = self.cloned_children(parent_id);
        for child_id in children.iter() {
            self.disconnect_child(*child_id);
        }
        children
    }

    pub fn disconnect(&mut self, parent_id: ParentId, child_id: ChildId) -> bool {
        let mut disconnected = false;
        if let Some(current_parent_id) = self.parents.get(&child_id)
            && parent_id == *current_parent_id
        {
            disconnected = self.parents.remove(&child_id).is_some();

            if let Some(child_ids) = self.children.get_mut(&parent_id) {
                disconnected |= child_ids.remove(&child_id);
            }
        }
        disconnected
    }

    pub fn connect(&mut self, parent_id: ParentId, child_id: ChildId) {
        if let Some(current_parent_id) = self.parent(child_id).copied() {
            if current_parent_id == parent_id {
                return;
            }
            let mut all_children_removed = false;
            if let Some(children) = self.children.get_mut(&current_parent_id) {
                children.remove(&child_id);
                all_children_removed = children.len() == 0;
            }
            if all_children_removed {
                self.children.remove(&current_parent_id);
            }
        }
        self.parents.insert(child_id, parent_id);
        if let Some(children) = self.children.get_mut(&parent_id) {
            children.insert(child_id);
        } else {
            let mut children = BTreeSet::<ChildId>::new();
            children.insert(child_id);
            self.children.insert(parent_id, children);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (ParentId, ChildId)> + '_ {
        self.parents.iter().map(|(child, parent)| (*parent, *child))
    }

    pub fn iter_children(&self) -> impl Iterator<Item = (ParentId, &BTreeSet<ChildId>)> + '_ {
        self.children
            .iter()
            .map(|(parent, children)| (*parent, children))
    }

    pub fn disconnect_parents(
        &mut self,
        parent_ids: impl Iterator<Item = ParentId>,
    ) -> impl Iterator<Item = (ParentId, ChildId)> + '_ {
        self.disconnect_children(
            parent_ids
                .filter_map(|parent_id| self.children(parent_id))
                .flatten()
                .copied()
                .collect::<BTreeSet<ChildId>>()
                .into_iter(),
        )
    }

    pub fn disconnect_children<'a>(
        &'a mut self,
        child_ids: impl Iterator<Item = ChildId> + 'a,
    ) -> impl Iterator<Item = (ParentId, ChildId)> + 'a {
        child_ids.filter_map(|child_id| {
            if let Some(parent_id) = self.disconnect_child(child_id) {
                Some((parent_id, child_id))
            } else {
                None
            }
        })
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BidirectionalMultiParentEdges<ParentId, ChildId>
where
    ParentId: Ord + Copy + Hash,
    ChildId: Ord + Copy + Hash,
{
    parents: HashMap<ChildId, BTreeSet<ParentId>>,
    children: HashMap<ParentId, BTreeSet<ChildId>>,
}

impl<ParentId: Ord + Copy + Hash, ChildId: Ord + Copy + Hash>
    BidirectionalMultiParentEdges<ParentId, ChildId>
{
    pub fn len(&self) -> usize {
        self.parents.len()
    }

    pub fn clear(&mut self) {
        self.children.clear();
        self.parents.clear();
    }

    pub fn parents(&self, child_id: ChildId) -> Option<&BTreeSet<ParentId>> {
        self.parents.get(&child_id)
    }

    pub fn cloned_children(&self, parent_id: ParentId) -> BTreeSet<ChildId> {
        let mut child_ids = BTreeSet::<ChildId>::new();
        if let Some(children) = self.children(parent_id) {
            child_ids = children.clone();
        }
        child_ids
    }

    pub fn has_children(&self, parent_id: ParentId) -> bool {
        self.children.contains_key(&parent_id)
    }

    pub fn children(&self, parent_id: ParentId) -> Option<&BTreeSet<ChildId>> {
        self.children.get(&parent_id)
    }

    pub fn children_mut(&mut self, parent_id: ParentId) -> Option<&mut BTreeSet<ChildId>> {
        self.children.get_mut(&parent_id)
    }

    pub fn disconnect_child(&mut self, child_id: ChildId) -> BTreeSet<ParentId> {
        let mut disconnected_edges = BTreeSet::<ParentId>::new();
        if let Some(parent_ids) = self.parents.remove(&child_id) {
            for parent_id in parent_ids.iter() {
                disconnected_edges.insert(*parent_id);

                let mut all_children_removed = false;
                if let Some(children) = self.children.get_mut(&parent_id) {
                    children.remove(&child_id);
                    all_children_removed = children.len() == 0;
                }
                if all_children_removed {
                    self.children.remove(&parent_id);
                }
            }
        }
        disconnected_edges
    }

    pub fn disconnect_parent(&mut self, parent_id: ParentId) -> BTreeSet<ChildId> {
        let mut disconnected_edges = BTreeSet::<ChildId>::new();
        if let Some(child_ids) = self.children.remove(&parent_id) {
            for child_id in child_ids.iter() {
                disconnected_edges.insert(*child_id);

                let mut all_parents_removed = false;
                if let Some(parents) = self.parents.get_mut(&child_id) {
                    parents.remove(&parent_id);
                    all_parents_removed = parents.len() == 0;
                }
                if all_parents_removed {
                    self.parents.remove(&child_id);
                }
            }
        }
        disconnected_edges
    }

    pub fn disconnect(&mut self, parent_id: ParentId, child_id: ChildId) -> bool {
        let mut disconnected = false;
        if let Some(child_ids) = self.children.get_mut(&parent_id) {
            disconnected |= child_ids.remove(&child_id);
        }
        if let Some(parent_ids) = self.parents.get_mut(&child_id) {
            disconnected |= parent_ids.remove(&parent_id);
        }
        disconnected
    }

    pub fn connect(&mut self, parent_id: ParentId, child_id: ChildId) {
        if let Some(children) = self.children.get_mut(&parent_id) {
            children.insert(child_id);
        } else {
            let mut children = BTreeSet::<ChildId>::new();
            children.insert(child_id);
            self.children.insert(parent_id, children);
        }

        if let Some(parents) = self.parents.get_mut(&child_id) {
            parents.insert(parent_id);
        } else {
            let mut parents = BTreeSet::<ParentId>::new();
            parents.insert(parent_id);
            self.parents.insert(child_id, parents);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&BTreeSet<ParentId>, ChildId)> + '_ {
        self.parents.iter().map(|(child, parent)| (parent, *child))
    }

    pub fn iter_children(&self) -> impl Iterator<Item = (ParentId, &BTreeSet<ChildId>)> + '_ {
        self.children
            .iter()
            .map(|(parent, children)| (*parent, children))
    }

    // pub fn disconnect_parents(
    //     &mut self,
    //     parent_ids: impl Iterator<Item = ParentId>,
    // ) -> impl Iterator<Item = (ParentId, ChildId)> + '_ {
    //     parent_ids
    //         .map(|parent_id| {
    //             self.disconnect_parent(parent_id)
    //                 .iter()
    //                 .map(move |child_id| (parent_id, *child_id))
    //         })
    //         .flatten()
    // }

    // pub fn disconnect_children<'a>(
    //     &'a mut self,
    //     child_ids: impl Iterator<Item = ChildId> + 'a,
    // ) -> impl Iterator<Item = (ParentId, ChildId)> + 'a {
    //     child_ids.filter_map(|child_id| {
    //         if let Some(parent_ids) = self.disconnect_child(child_id) {
    //             Some((parent_id, child_id))
    //         } else {
    //             None
    //         }
    //     })
    // }
}
