use std::{mem, collections::{HashSet, HashMap}};

use serde::{Serialize, Deserialize};

use crate::{Identified, Modifier, IdPathBuf, IdPath};

/// A UI component tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Node {
    // Primitive
    Empty {}, // Intentionally not a unit variant for uniform serialization
    Text { content: String },
    TextField { content: String },
    Button { label: Box<Identified<Node>> },

    // Aggregation
    Group { children: Vec<Identified<Node>> },

    // Layout
    VStack { wrapped: Box<Identified<Node>> },
    HStack { wrapped: Box<Identified<Node>> },
    ZStack { wrapped: Box<Identified<Node>> },
    List { wrapped: Box<Identified<Node>> },

    // Modifier
    Modified { wrapped: Box<Identified<Node>>, modifier: Modifier, }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NodeDiff {
    added: Vec<IdPathBuf>,
    removed: Vec<IdPathBuf>,
    changed: Vec<IdPathBuf>,
}

// TODO: Investigate whether we could use a proc-macro to derive the tree
// diffing (especially traverse)? We'd have to make sure to take care e.g.
// of Vecs, which require special handling, see the Group case in traverse.

impl NodeDiff {
    fn new() -> Self {
        Self {
            added: Vec::new(),
            removed: Vec::new(),
            changed: Vec::new(),
        }
    }

    pub(crate) fn between(new: &Identified<Node>, old: &Identified<Node>) -> Self {
        let mut diff = Self::new();
        diff.traverse(new, old);
        diff
    }

    fn traverse(&mut self, new: &Identified<Node>, old: &Identified<Node>) {
        if new.id_path() != old.id_path() || mem::discriminant(new.value()) != mem::discriminant(old.value()) {
            self.added.push(new.id_path().to_owned());
            self.removed.push(old.id_path().to_owned());
            return;
        }
        let id_path = new.id_path(); // == old.id_path()
        match (new.value(), old.value()) {
            (Node::Empty {}, Node::Empty {}) => {},
            (Node::Text { content: c1 }, Node::Text { content: c2 }) |
            (Node::TextField { content: c1 }, Node::TextField { content: c2 }) => if c1 != c2 {
                self.changed.push(id_path.to_owned());
            },
            (Node::Button { label: l1 }, Node::Button { label: l2 }) => self.traverse(l1, l2),
            (Node::VStack { wrapped: w1 }, Node::VStack { wrapped: w2 }) |
            (Node::HStack { wrapped: w1 }, Node::HStack { wrapped: w2 }) |
            (Node::ZStack { wrapped: w1 }, Node::ZStack { wrapped: w2 }) |
            (Node::List { wrapped: w1 }, Node::List { wrapped: w2 }) => self.traverse(w1, w2),
            (Node::Modified { wrapped: w1, modifier: m1 }, Node::Modified { wrapped: w2, modifier: m2 }) => {
                if m1 != m2 {
                    self.changed.push(id_path.to_owned());
                }
                self.traverse(w1, w2);
            },
            (Node::Group { children: cs1 }, Node::Group { children: cs2 }) => {
                let new_children: HashMap<&IdPath, &Identified<Node>> = cs1.iter().map(|c| (c.id_path(), c)).collect();
                let old_children: HashMap<&IdPath, &Identified<Node>> = cs2.iter().map(|c| (c.id_path(), c)).collect();

                let new_paths: HashSet<&IdPath> = new_children.keys().cloned().collect();
                let old_paths: HashSet<&IdPath> = old_children.keys().cloned().collect();

                for &child_path in new_paths.difference(&old_paths) {
                    self.added.push(child_path.to_owned());
                }

                for &child_path in old_paths.difference(&new_paths) {
                    self.removed.push(child_path.to_owned());
                }

                for &child_path in new_paths.intersection(&old_paths) {
                    self.traverse(new_children[child_path], old_children[child_path]);
                }
            },
            (x, y) => if x == y {
                panic!("There seems to be a missing traverse case for ({:?}, {:?})", x, y)
            } else {
                unreachable!()
            }
        }
    }

    pub(crate) fn added(&self) -> impl Iterator<Item = &IdPath> {
        self.added.iter().map(|a| a.as_ref())
    }

    pub(crate) fn removed(&self) -> impl Iterator<Item = &IdPath> {
        self.removed.iter().map(|a| a.as_ref())
    }

    pub(crate) fn changed(&self) -> impl Iterator<Item = &IdPath> {
        self.changed.iter().map(|a| a.as_ref())
    }
}
