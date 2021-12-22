use super::Item;

pub struct Backlog {
    items: Vec<Box<dyn Item>>,
}

pub trait AddItem {
    fn add_item(&mut self, item: Box<dyn Item>);
}

impl Backlog {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn items(&self) -> &[Box<dyn Item>] {
        &self.items
    }
}

impl Default for Backlog {
    fn default() -> Self {
        Self::new()
    }
}

impl AddItem for Backlog {
    fn add_item(&mut self, item: Box<dyn Item>) {
        self.items.push(item)
    }
}
