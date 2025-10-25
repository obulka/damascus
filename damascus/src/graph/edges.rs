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

impl<ParentId: Ord + Copy + Hash + 'static, ChildId: Ord + Copy + Hash + 'static>
    BidirectionalSingleParentEdges<ParentId, ChildId>
{
    pub fn len(&self) -> usize {
        self.parents.len()
    }

    pub fn clear(&mut self) {
        self.children.clear();
        self.parents.clear();
    }

    pub fn parent_owned(&self, child_id: ChildId) -> Option<ParentId> {
        self.parent(child_id).copied()
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

    pub fn connect(&mut self, parent_id: ParentId, child_id: ChildId) {
        if let Some(parent_parent_id) = self.parent_owned(child_id) {
            if parent_parent_id == parent_id {
                return;
            }
            let mut all_children_removed = false;
            if let Some(children) = self.children.get_mut(&parent_parent_id) {
                children.remove(&child_id);
                all_children_removed = children.len() == 0;
            }
            if all_children_removed {
                self.children.remove(&parent_parent_id);
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

    pub fn disconnect_parents<'a, 'b>(
        &'b mut self,
        parent_ids: impl Iterator<Item = ParentId> + 'a + 'b,
    ) -> impl Iterator<Item = (ParentId, ChildId)> + 'b
    where
        ParentId: 'a,
        ChildId: 'a,
    {
        self.disconnect_children(
            parent_ids
                .filter_map(|parent_id| self.children(parent_id))
                .flatten()
                .copied()
                .collect::<BTreeSet<ChildId>>()
                .into_iter(),
        )
    }

    pub fn disconnect_children<'a, 'b>(
        &'b mut self,
        child_ids: impl Iterator<Item = ChildId> + 'a + 'b,
    ) -> impl Iterator<Item = (ParentId, ChildId)> + 'b
    where
        ParentId: 'a,
        ChildId: 'a,
    {
        child_ids.filter_map(|child_id| {
            if let Some(parent_id) = self.disconnect_child(child_id) {
                Some((parent_id, child_id))
            } else {
                None
            }
        })
    }
}
