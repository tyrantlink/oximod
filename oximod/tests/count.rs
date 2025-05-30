use mongodb::bson::{doc, oid::ObjectId};
use oximod::{set_global_client, Model};
use testresult::TestResult;
use serde::{Deserialize, Serialize};

// Run test: cargo nextest run counts_matching_documents_correctly
#[tokio::test]
async fn counts_matching_documents_correctly() -> TestResult {
    dotenv::dotenv().ok();
    let mongodb_uri = std::env::var("MONGODB_URI").expect("Failed to find MONGODB_URI");

    set_global_client(mongodb_uri).await.unwrap_or_else(|e| panic!("{}", e));

    #[derive(Model, Serialize, Deserialize, Debug)]
    #[db("test")]
    #[collection("count")]
    pub struct User {
        #[serde(skip_serializing_if = "Option::is_none")]
        _id: Option<ObjectId>,
        name: String,
        age: i32,
        active: bool,
    }

    User::clear().await?;

    let users = vec![
        User {
            _id: None,
            name: "User1".to_string(),
            age: 30,
            active: true,
        },
        User {
            _id: None,
            name: "User3".to_string(),
            age: 30,
            active: false,
        },
        User {
            _id: None,
            name: "User3".to_string(),
            age: 25,
            active: true,
        },
    ];

    for user in users {
        user.save().await?;
    }

    let count = User::count(doc! { "age": 30 }).await?;
    assert_eq!(count, 2);

    Ok(())
}
