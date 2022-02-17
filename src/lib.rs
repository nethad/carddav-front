use tera::Tera;

pub mod routes;

pub struct AppData {
    pub templates: Tera,
}

pub fn setup_app_data() -> AppData {
    let tera = Tera::new("templates/**/*").unwrap();
    AppData { templates: tera }
}
