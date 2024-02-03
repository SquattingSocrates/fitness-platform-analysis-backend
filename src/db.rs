use bson::Document;
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client, Collection};

#[derive(Clone, Debug)]
pub struct DB {
    pub collection: Collection<Document>,
}

// type Result<T> = std::result::Result<T, MyError>;

impl DB {
    pub async fn init() -> Result<Self, mongodb::error::Error> {
        dotenv().ok();
        let mongodb_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let database_name =
            std::env::var("MONGO_INITDB_DATABASE").expect("MONGO_INITDB_DATABASE must be set.");
        let collection_name =
            std::env::var("MONGODB_COLLECTION").expect("MONGODB_COLLECTION must be set.");

        let mut client_options = ClientOptions::parse(mongodb_uri).await?;
        client_options.app_name = Some(database_name.to_string());

        let client = Client::with_options(client_options)?;
        let database = client.database(database_name.as_str());

        let collection = database.collection::<Document>(collection_name.as_str());

        println!("✅ Database connected successfully");

        Ok(Self { collection })
    }
}
