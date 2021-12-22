mod backlog;
mod item;
mod task;
mod user_story;

pub use backlog::{AddItem, Backlog};
pub use item::{Assingable, Estimatable, HasTitle, Item, StoryPoint};
pub use task::Task;
pub use user_story::UserStory;

use common::uuid::Uuid;

pub trait Entity {
    fn id(&self) -> Uuid;
}
