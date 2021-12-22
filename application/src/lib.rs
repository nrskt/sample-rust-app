mod error;

use common::async_trait::async_trait;
use domain::models::{AddItem, Backlog, Item, Task, UserStory};
pub use error::ApplicationError;
use ports::{BacklogRepository, ProvideBacklogRepository};

#[async_trait]
pub trait BacklogApplication: ProvideBacklogRepository {
    async fn add_item(&self, command: Box<dyn AddItemCmd>) -> Result<Backlog, ApplicationError> {
        let backlog_repo = self.provide();

        let item: Box<dyn Item> = match command.item_type() {
            ItemType::Story => Box::new(UserStory::new(command.title())),
            ItemType::Task => Box::new(Task::new(command.title())),
        };

        let mut backlog = backlog_repo.get().await?.unwrap_or_default();
        backlog.add_item(item);
        backlog_repo.save(backlog.clone()).await?;
        Ok(backlog)
    }
}

pub trait AddItemCmd: Send {
    fn item_type(&self) -> ItemType;
    fn title(&self) -> &str;
}

#[derive(Debug, Clone, Copy)]
pub enum ItemType {
    Story,
    Task,
}

#[cfg(test)]
mod tests {
    use domain::models::*;
    use mockall::*;
    use ports::*;

    use super::*;

    mock! {
        Test {}

        #[async_trait]
        impl BacklogRepository for Test {
            async fn get(&self) -> Result<Option<Backlog>, PortError>;
            async fn save(&self, backlog: Backlog) -> Result<(), PortError>;
        }
    }

    impl ProvideBacklogRepository for MockTest {
        type Repository = MockTest;

        fn provide(&self) -> &Self::Repository {
            &self
        }
    }

    impl BacklogApplication for MockTest {}

    struct Cmd {}

    impl AddItemCmd for Cmd {
        fn item_type(&self) -> ItemType {
            ItemType::Story
        }

        fn title(&self) -> &str {
            ""
        }
    }

    #[tokio::test]
    async fn test_add_item() {
        let mut mock = MockTest::new();
        mock.expect_get().with().times(1).returning(|| Ok(None));
        mock.expect_save()
            .withf(|b| b.items().len() == 1)
            .times(1)
            .returning(|_| Ok(()));
        let cmd = Box::new(Cmd {});

        let r = mock.add_item(cmd).await;
        assert!(r.is_ok());
    }
}
