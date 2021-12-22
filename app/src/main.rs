use app::App;
use application::{AddItemCmd, BacklogApplication};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = App::default();

    let backlog = app.add_item(Box::new(Cmd)).await?;
    println!("{:?}", backlog);
    Ok(())
}

struct Cmd;

impl AddItemCmd for Cmd {
    fn item_type(&self) -> application::ItemType {
        application::ItemType::Story
    }

    fn title(&self) -> &str {
        "dummy title"
    }
}
