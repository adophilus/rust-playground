use std::sync::Arc;

#[derive(Debug)]
struct User<'a> {
    first_name: &'a str,
    last_name: &'a str,
}

impl std::fmt::Display for User<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn get_user<'a>() -> Arc<User<'a>> {
    let user = Arc::new(User {
        first_name: "Uchenna",
        last_name: "Ofoma",
    });

    std::thread::spawn({
        let user = user.clone();
        move || {
            println!("User(from thread): {}", user);
            user.clone()
        }
    });

    user
}

fn main() {
    let mut user = get_user();
    let user = Arc::get_mut(&mut user);
    let user = user.unwrap();
    user.first_name = "Austin";
    user.last_name = "Chris";
    println!("User: {}", user);
}

