#[derive(Clone)]
pub struct Cookie {
    pub name: &'static str,
    pub weight: i16,
}

pub struct CookieMachine {
    cookie: Cookie,
}

impl CookieMachine {
    pub fn new(cookie: Cookie) -> CookieMachine {
        CookieMachine { cookie }
    }

    pub fn make_cookie(&self) -> Cookie {
        self.cookie.clone()
    }
}
