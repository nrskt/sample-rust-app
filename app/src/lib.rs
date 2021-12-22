use adaptor::HashMapDb;
use application::BacklogApplication;
use ports::ProvideBacklogRepository;

#[derive(Debug, Clone, Default)]
pub struct App {
    db: HashMapDb,
}

impl BacklogApplication for App {}

impl ProvideBacklogRepository for App {
    type Repository = HashMapDb;

    fn provide(&self) -> &Self::Repository {
        &self.db
    }
}
