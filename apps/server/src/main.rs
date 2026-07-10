mod academic;
mod auth;
mod research;
mod shared;
mod university;

use crate::shared::LoggerLayer;
use sword::Application;

#[tokio::main]
async fn main() {
    let application = Application::builder()
        .with_module::<shared::SharedModule>()
        .with_module::<university::UniversityModule>()
        .with_module::<academic::AcademicModule>()
        .with_module::<auth::AuthModule>()
        .with_module::<research::ResearchModule>()
        .with_layer(LoggerLayer())
        .build();

    application.run().await;
}
