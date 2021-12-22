use common::uuid::Uuid;

use super::{Assingable, Entity, Estimatable, HasTitle, Item, StoryPoint};

#[derive(Debug, Clone)]
pub struct UserStory {
    id: Uuid,
    title: String,
    point: Option<StoryPoint>,
    assignee: Option<String>,
}

impl UserStory {
    pub fn new(title: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.to_string(),
            point: None,
            assignee: None,
        }
    }
}

impl Item for UserStory {}

impl Entity for UserStory {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl HasTitle for UserStory {
    fn title(&self) -> &str {
        &self.title
    }
}

impl Estimatable for UserStory {
    fn point(&self) -> Option<StoryPoint> {
        self.point
    }

    fn estimate(&mut self, point: StoryPoint) {
        self.point = Some(point);
    }
}

impl Assingable for UserStory {
    fn assignee(&self) -> Option<&str> {
        self.assignee.as_deref()
    }

    fn assign(&mut self, assignee: &str) {
        self.assignee = Some(assignee.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estimate() {
        let mut story = UserStory::new("test");
        let point = StoryPoint::new(1).unwrap();
        story.estimate(point);
        assert_eq!(story.point(), Some(point))
    }
}
