use mongodb::{error::Error, options::ClientOptions, Client};

pub(crate) async fn connect() -> Result<Client, Error> {
    let client_options =
        ClientOptions::parse("mongodb+srv://rust-app-2:r4tR9Soq3mZO76NT@splatoon-srs.tvfx6.mongodb.net/sets?retryWrites=true&w=majority").await?;
    Ok(Client::with_options(client_options)?)
}

#[cfg(test)]
mod tests {
    use std::env;

    use mongodb::bson::doc;

    use super::*;

    #[actix_rt::test]
    async fn test_database_connection() {
        println!("{:?}", env::current_dir().unwrap());
        let client = connect().await;
        assert!(client.is_ok());
        assert!(client
            .unwrap()
            .database("sets")
            .run_command(doc! {"ping": 1}, None)
            .await
            .is_ok());
    }
}
