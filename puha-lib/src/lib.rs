/// Returns a greeting string from `puha-lib`.
pub fn greet() -> &'static str {
    "Hello from puha-lib!"
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    name: String,
    description: String,
}

#[derive(Default)]
pub struct ItemBuilder {
    name: String,
    description: String,
}

impl ItemBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn build(self) -> Item {
        Item {
            name: self.name,
            description: self.description,
        }
    }
}

impl Item {
    pub fn builder() -> ItemBuilder {
        ItemBuilder::new()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = description.into();
    }
}

#[derive(Debug, Clone)]
pub struct Space {
    name: String,
    items: Vec<Item>,
    spaces: Vec<Space>,
    root: bool,
}

#[derive(Default)]
pub struct SpaceBuilder {
    name: String,
    items: Vec<Item>,
    spaces: Vec<Space>,
    root: bool,
}

impl SpaceBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn root(mut self, is_root: bool) -> Self {
        self.root = is_root;
        self
    }

    pub fn items(mut self, items: Vec<Item>) -> Self {
        self.items = items;
        self
    }

    pub fn spaces(mut self, spaces: Vec<Space>) -> Self {
        self.spaces = spaces;
        self
    }

    pub fn push_item(mut self, item: Item) -> Self {
        self.items.push(item);
        self
    }

    pub fn push_space(mut self, space: Space) -> Self {
        self.spaces.push(space);
        self
    }

    pub fn build(self) -> Space {
        Space {
            name: self.name,
            items: self.items,
            spaces: self.spaces,
            root: self.root,
        }
    }
}

impl Space {
    pub fn builder() -> SpaceBuilder {
        SpaceBuilder::new()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn items(&self) -> &[Item] {
        &self.items
    }

    pub fn spaces(&self) -> &[Space] {
        &self.spaces
    }

    pub fn root(&self) -> bool {
        self.root
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn set_root(&mut self, root: bool) {
        self.root = root;
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn add_space(&mut self, space: Space) {
        self.spaces.push(space);
    }

    pub fn find_space<'a>(&'a self, name: &str) -> Option<&'a Space> {
        if self.name == name {
            return Some(self);
        }
        for space in &self.spaces {
            if let Some(found) = space.find_space(name) {
                return Some(found);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_returns_expected() {
        assert_eq!(greet(), "Hello from puha-lib!");
    }

    #[test]
    fn build_and_find_space() {
        let item = Item::builder()
            .name("item1")
            .description("desc")
            .build();

        let child = Space::builder()
            .name("child")
            .push_item(item.clone())
            .build();

        let root = Space::builder()
            .name("root")
            .root(true)
            .push_space(child.clone())
            .build();

        let found = root.find_space("child").expect("space not found");
        assert_eq!(found.name(), "child");
        assert_eq!(found.items().len(), 1);
        assert_eq!(found.items()[0], item);
    }
}
