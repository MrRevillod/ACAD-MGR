mod academic;
mod auth;
mod imports;
mod shared;
mod university;

use sword::Application;

#[tokio::main]
async fn main() {
    let application = Application::builder()
        .with_module::<shared::SharedModule>()
        .with_module::<university::UniversityModule>()
        .with_module::<academic::AcademicModule>()
        .with_module::<auth::AuthModule>()
        .build();

    application.run().await
}
