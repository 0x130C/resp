use services::router::Route;


pub enum Page {
    Content,
    Login,
    NotFound,
    Loading,
    Admin
}

impl<T> Into<Route<T>> for Page
    where
        T: Default,
{
    fn into(self) -> Route<T> {
        Route {
            fragment: Some(
                match self {
                    Page::Content => "content",
                    Page::NotFound => "error",
                    Page::Loading => "loading",
                    Page::Login => "login",
                    Page::Admin => "admin"
                }.into(),
            ),
            path_segments: vec![],
            query: None,
            state: T::default()
        }
    }
}

impl<T> Into<Page> for Route<T> {
    fn into(self) -> Page {
        match self.fragment {
            Some(f) => match f.as_str() {
                "content" => Page::Content,
                "loading" => Page::Loading,
                "login" => Page::Login,
                "admin" => Page::Admin,
                _ => Page::NotFound,
            },
            _ => Page::NotFound,
        }
    }
}

