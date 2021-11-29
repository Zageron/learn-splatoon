use mongodb::{error::Error, options::ClientOptions, Client};

pub(crate) async fn connect() -> Result<Client, Error> {
    let client_options =
        ClientOptions::parse("mongodb+srv://rust-app-2:r4tR9Soq3mZO76NT@splatoon-srs.tvfx6.mongodb.net/sets?retryWrites=true&w=majority").await?;
    Ok(Client::with_options(client_options)?)
}
