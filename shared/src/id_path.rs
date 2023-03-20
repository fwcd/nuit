#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct IdPath {
    components: Vec<usize>,
}

impl IdPath {
    pub fn root() -> Self {
        Self { components: Vec::new() }
    }

    pub fn child(&self, i: usize) -> Self {
        let mut components = self.components.clone();
        components.push(i);
        Self { components }
    }
}
