use reqwest::Client as HttpClient;
use serenity::prelude::TypeMapKey;

pub struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}
