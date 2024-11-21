pub mod model {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Blog {
        pub id: String,
        pub title: String,
        pub content: String,
        pub link: String,
        pub image_url: String,
        pub topic: String,
        pub info: String,
        pub preprocessed: String,
    }
}
