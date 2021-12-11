use std::path::PathBuf;

use mongodb::{
    error::Error,
    options::{AuthMechanism, ClientOptions, Credential, Tls, TlsOptions},
    Client,
};

pub(crate) async fn connect() -> Result<Client, Error> {
    let mut client_options = ClientOptions::parse(
        "mongodb+srv://splatoon-srs.tvfx6.mongodb.net/sets?retryWrites=true&w=majority",
    )
    .await?;
    client_options.credential = Some(
        Credential::builder()
            .mechanism(AuthMechanism::MongoDbX509)
            .build(),
    );
    let tls_options = TlsOptions::builder()
        .cert_key_file_path(PathBuf::from(".secrets/cert.pem"))
        .build();
    client_options.tls = Some(Tls::Enabled(tls_options));
    Ok(Client::with_options(client_options)?)
}
