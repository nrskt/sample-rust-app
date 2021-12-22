mod error;

use common::async_trait::async_trait;
use domain::models::Backlog;
pub use error::PortError;
use mockall::automock;

#[async_trait]
#[automock]
pub trait BacklogRepository {
    async fn get(&self) -> Result<Option<Backlog>, PortError>;
    async fn save(&self, backlog: Backlog) -> Result<(), PortError>;
}
