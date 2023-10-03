pub mod user;

use poem::Route;
use poem_openapi::{OpenApiService, Tags};

use self::user::UserApi;

#[derive(Tags)]
pub enum ApiTags {
    /// Operations about user
    User,
}

pub struct App;

impl App {
    pub fn new() -> Route {
        // If you want to add another API, resource etc, you can use the following code as a template:
        //  let api_service =
        //     OpenApiService::new((UserApi::default(), AnotherApi::default()), "Aquarius", "1.0")
        //         .server("http://localhost:3000/api");

        let api_service = OpenApiService::new(UserApi::default(), "Aquarius", "1.0")
            .server("http://localhost:3000/api");

        let ui = api_service.swagger_ui();

        Route::new().nest("/api", api_service).nest("/", ui)
    }
}
