use actix_web::{Result, Json};

#[derive(Deserialize,Serialize, Debug)]
pub struct Info {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
}

pub fn info(info: Json<Info>) -> Result<Json<Info>> {
    println!("=========={:?}=========", info);
    Ok(Json(Info{
                username: info.username.clone(),
                email: info.email.clone(),
                password: info.password.clone(),
                confirm_password: info.confirm_password.clone(),
            }))
}

