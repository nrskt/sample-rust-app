mod error;

use common::{async_trait::async_trait, futures::future::ready};
use domain::models::{AddItem, Item, Task, UserStory};
pub use error::ApplicationError;
use mockall::{automock, mock};
use ports::BacklogRepository;

#[async_trait]
pub trait BacklogApplication: BacklogRepository {
    async fn add_item(&self, command: Box<dyn AddItemCmd>) -> Result<(), ApplicationError> {
        let item: Box<dyn Item> = match command.item_type() {
            ItemType::Story => Box::new(UserStory::new(command.title())),
            ItemType::Task => Box::new(Task::new(command.title())),
        };

        let mut backlog = self.get().await?.unwrap_or_default();
        backlog.add_item(item);
        self.save(backlog).await?;
        Ok(())
    }
}

#[automock]
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
    use super::*;
    use domain::models::*;
    use ports::*;

    mock! {
        Test {}

        #[async_trait]
        impl BacklogRepository for Test {
            async fn get(&self) -> Result<Option<Backlog>, PortError>;
            async fn save(&self, backlog: Backlog) -> Result<(), PortError>;
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
