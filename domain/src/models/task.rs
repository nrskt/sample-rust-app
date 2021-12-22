use common::uuid::Uuid;

use super::{Assingable, Entity, Estimatable, HasTitle, Item, StoryPoint};

pub struct Task {
    id: Uuid,
    title: String,
    point: Option<StoryPoint>,
    assignee: Option<String>,
}

impl Task {
    pub fn new(title: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.to_string(),
            point: None,
            assignee: None,
        }
    }
}

impl Entity for Task {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl Item for Task {}

impl HasTitle for Task {
    fn title(&self) -> &str {
        &self.title
    }
}

impl Estimatable for Task {
    fn point(&self) -> Option<StoryPoint> {
        self.point
    }

    fn estimate(&mut self, point: StoryPoint) {
        self.point = Some(point);
    }
}

impl Assingable for Task {
    fn assignee(&self) -> Option<&str> {
        self.assignee.as_deref()
    }

    fn assign(&mut self, assignee: &str) {
        self.assignee = Some(assignee.to_string())
    }
}
