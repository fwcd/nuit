use std::{mem, collections::{HashSet, HashMap}};

use crate::{Id, IdPath, IdPathBuf, Identified};

use super::Node;

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

    pub(crate) fn between(new: &Node, old: &Node) -> Self {
        let mut diff = Self::new();
        diff.traverse(&IdPathBuf::root(), new, old);
        diff
    }

    fn traverse_identified(&mut self, id_path: &IdPath, new: &Identified<Node>, old: &Identified<Node>) {
        if new.id() != old.id() {
            self.added.push(id_path.child(new.id().clone()));
            self.removed.push(id_path.child(old.id().clone()));
            return;
        }
        self.traverse(&id_path.child(new.id().clone()), new.value(), old.value());
    }

    fn traverse(&mut self, id_path: &IdPath, new: &Node, old: &Node) {
        if mem::discriminant(new) != mem::discriminant(old) {
            self.added.push(id_path.to_owned());
            self.removed.push(id_path.to_owned());
            return;
        }
        match (new, old) {
            (Node::Empty {}, Node::Empty {}) => {},
            (Node::Shape { shape: s1 }, Node::Shape { shape: s2 }) => {
                if s1 != s2 {
                    self.changed.push(id_path.to_owned());
                }
            },
            (Node::Text { content: c1 }, Node::Text { content: c2 }) |
            (Node::TextField { content: c1 }, Node::TextField { content: c2 }) => {
                if c1 != c2 {
                    self.changed.push(id_path.to_owned());
                }
            },
            (Node::Picker { title: t1, selection: s1, content: c1 }, Node::Picker { title: t2, selection: s2, content: c2 }) => {
                if t1 != t2 || s1 != s2 {
                    self.changed.push(id_path.to_owned());
                }
                self.traverse_identified(id_path, c1, c2);
            },
            (Node::Button { label: l1 }, Node::Button { label: l2 }) => self.traverse_identified(id_path, l1, l2),
            (Node::Child { wrapped: w1 }, Node::Child { wrapped: w2 }) |
            (Node::VStack { wrapped: w1 }, Node::VStack { wrapped: w2 }) |
            (Node::HStack { wrapped: w1 }, Node::HStack { wrapped: w2 }) |
            (Node::ZStack { wrapped: w1 }, Node::ZStack { wrapped: w2 }) |
            (Node::List { wrapped: w1 }, Node::List { wrapped: w2 }) => self.traverse_identified(id_path, w1, w2),
            (Node::Modified { wrapped: w1, modifier: m1 }, Node::Modified { wrapped: w2, modifier: m2 }) => {
                if m1 != m2 {
                    self.changed.push(id_path.to_owned());
                }
                self.traverse_identified(id_path, w1, w2);
            },
            (Node::Group { children: cs1 }, Node::Group { children: cs2 }) => {
                let new_children: HashMap<&Id, &Identified<Node>> = cs1.iter().map(|c| (c.id(), c)).collect();
                let old_children: HashMap<&Id, &Identified<Node>> = cs2.iter().map(|c| (c.id(), c)).collect();

                let new_ids: HashSet<&Id> = new_children.keys().cloned().collect();
                let old_ids: HashSet<&Id> = old_children.keys().cloned().collect();

                for &child_id in new_ids.difference(&old_ids) {
                    self.added.push(id_path.child(child_id.clone()));
                }

                for &child_id in old_ids.difference(&new_ids) {
                    self.removed.push(id_path.child(child_id.clone()));
                }

                for &child_id in new_ids.intersection(&old_ids) {
                    self.traverse_identified(id_path, new_children[child_id], old_children[child_id]);
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

    #[allow(dead_code)]
    pub(crate) fn changed(&self) -> impl Iterator<Item = &IdPath> {
        self.changed.iter().map(|a| a.as_ref())
    }
}
