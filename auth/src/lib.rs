mod models;
mod procedure;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use crate::{procedure::{hash_pwd, verifiy_hashpwd, validate_register_model}, models::RegisterModel};

    use super::{models, procedure};

    #[tokio::test]
    async fn verify_pwd_works() {
        let result = hash_pwd("123456".to_owned()).unwrap();
        let result = verifiy_hashpwd("123456".to_owned(), result.clone());

        assert_eq!(result, true);
    }
    #[tokio::test]
    async fn validator_register_model_works()->Result<(),Box<dyn std::error::Error>> {
        let model=RegisterModel{user_name:Some("patrick".to_owned()),email:"chenhq@163.com".to_owned(),password:"Abc123456".to_owned()};
        let result = validate_register_model(&model)?;
        assert_eq!(result,true);
        Ok(())
    }
    #[tokio::test]
    async fn validator_register_model_email_invalid() {
        let model=RegisterModel{user_name:Some("patrick".to_owned()),email:"chen@hq163com".to_owned(),password:"Abc123456".to_owned()};
        let result = validate_register_model(&model);
        
        assert_eq!(result.is_err(), true);
    }
}
