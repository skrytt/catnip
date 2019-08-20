pub mod models {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct RootCat {
        pub cat: Cat
    }

    #[derive(Debug, Deserialize)]
    pub struct Cat {
        #[serde(default)]
        pub breeds: Vec<::serde_json::Value>,
        #[serde(default)]
        pub categories: Vec<::serde_json::Value>,
        pub height: i64,
        pub id: String,
        pub url: String,
        pub width: i64,
    }
}


