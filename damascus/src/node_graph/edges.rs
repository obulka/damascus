// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::{HashMap, HashSet};

use slotmap::SparseSecondaryMap;

use super::{inputs::InputId, outputs::OutputId};

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Edges {
    parents: SparseSecondaryMap<InputId, OutputId>,
    children: SparseSecondaryMap<OutputId, HashSet<InputId>>,
}

impl Edges {
    pub fn len(&self) -> usize {
        self.parents.len()
    }

    pub fn parent_owned(&self, input_id: InputId) -> Option<OutputId> {
        self.parent(input_id).copied()
    }

    pub fn parent(&self, input_id: InputId) -> Option<&OutputId> {
        self.parents.get(input_id)
    }

    pub fn children_owned(&self, output_id: OutputId) -> HashSet<InputId> {
        let mut child_ids = HashSet::<InputId>::new();
        if let Some(children) = self.children(output_id) {
            child_ids = children.clone();
        }
        child_ids
    }

    pub fn children(&self, output_id: OutputId) -> Option<&HashSet<InputId>> {
        self.children.get(output_id)
    }

    pub fn children_mut(&mut self, output_id: OutputId) -> Option<&mut HashSet<InputId>> {
        self.children.get_mut(output_id)
    }

    pub fn disconnect_input(&mut self, input_id: InputId) -> Option<OutputId> {
        if let Some(output_id) = self.parents.remove(input_id) {
            let mut all_children_removed = false;
            if let Some(children) = self.children.get_mut(output_id) {
                children.remove(&input_id);
                all_children_removed = children.len() == 0;
            }
            if all_children_removed {
                self.children.remove(output_id);
            }
            return Some(output_id);
        }
        None
    }

    pub fn disconnect_output(&mut self, output_id: OutputId) -> HashSet<InputId> {
        let children = self.children_owned(output_id);
        for input_id in children.iter() {
            self.disconnect_input(*input_id);
        }
        children
    }

    pub fn connect(&mut self, output_id: OutputId, input_id: InputId) {
        if let Some(parent_output_id) = self.parent_owned(input_id) {
            if parent_output_id == output_id {
                return;
            }
            let mut all_children_removed = false;
            if let Some(children) = self.children.get_mut(parent_output_id) {
                children.remove(&input_id);
                all_children_removed = children.len() == 0;
            }
            if all_children_removed {
                self.children.remove(parent_output_id);
            }
        }
        self.parents.insert(input_id, output_id);
        if let Some(children) = self.children.get_mut(output_id) {
            children.insert(input_id);
        } else {
            let mut children = HashSet::<InputId>::new();
            children.insert(input_id);
            self.children.insert(output_id, children);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (InputId, OutputId)> + '_ {
        self.parents.iter().map(|(input, output)| (input, *output))
    }

    pub fn iter_children(&self) -> impl Iterator<Item = (OutputId, &HashSet<InputId>)> + '_ {
        self.children
            .iter()
            .map(|(output, inputs)| (output, inputs))
    }

    pub fn disconnect_outputs<'a>(
        &mut self,
        output_ids: impl Iterator<Item = &'a OutputId> + 'a,
    ) -> HashMap<InputId, OutputId> {
        let mut disconnected = HashMap::<InputId, OutputId>::new();
        for output_id in output_ids {
            for input_id in self.children_owned(*output_id).into_iter() {
                self.disconnect_input(input_id);
                disconnected.insert(input_id, *output_id);
            }
        }
        disconnected
    }

    pub fn disconnect_inputs<'a>(
        &mut self,
        input_ids: impl Iterator<Item = &'a InputId> + 'a,
    ) -> HashMap<InputId, OutputId> {
        let mut disconnected = HashMap::<InputId, OutputId>::new();
        for input_id in input_ids {
            if let Some(output_id) = self.disconnect_input(*input_id) {
                disconnected.insert(*input_id, output_id);
            }
        }
        disconnected
    }
}
