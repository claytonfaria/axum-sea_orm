use serde::Deserialize;

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub gender: String,
    pub age: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub gender: Option<String>,
    pub age: Option<i32>,
}
