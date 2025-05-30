use mongodb::bson::{doc, oid::ObjectId};
use oximod::{set_global_client, Model};
use testresult::TestResult;
use serde::{Deserialize, Serialize};

// Run test: cargo nextest run checks_existence_of_matching_document
#[tokio::test]
async fn checks_existence_of_matching_document() -> TestResult {
    dotenv::dotenv().ok();
    let mongodb_uri = std::env::var("MONGODB_URI").expect("Failed to find MONGODB_URI");

    set_global_client(mongodb_uri).await.unwrap_or_else(|e| panic!("{}", e));

    #[derive(Model, Serialize, Deserialize, Debug)]
    #[db("test")]
    #[collection("exists")]
    pub struct User {
        #[serde(skip_serializing_if = "Option::is_none")]
        _id: Option<ObjectId>,
        name: String,
        age: i32,
        active: bool,
    }

    User::clear().await?;

    let user = User {
        _id: None,
        name: "User1".to_string(),
        age: 27,
        active: true,
    };

    user.save().await?;

    let exists = User::exists(doc! { "name": "User1" }).await?;
    assert!(exists);

    let not_exists = User::exists(doc! { "name": "SomeoneWhoDoesNotExist" }).await?;
    assert!(!not_exists);

    Ok(())
}
