use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TestBody {
    pub name: String,
}

#[derive(Serialize)]
pub struct TestObject {
    pub name: String,
}
