use mongodb::bson::{ doc, oid::ObjectId };
use oximod::{set_global_client, Model};
use testresult::TestResult;
use serde::{ Deserialize, Serialize };

// Run test: cargo nextest run finds_multiple_matching_documents
#[tokio::test]
async fn finds_multiple_matching_documents() -> TestResult {
    dotenv::dotenv().ok();
    let mongodb_uri = std::env::var("MONGODB_URI").expect("Failed to find MONGODB_URI");

    set_global_client(mongodb_uri).await.unwrap_or_else(|e| panic!("{}", e));

    #[derive(Model, Serialize, Deserialize, Debug)]
    #[db("test")]
    #[collection("find")]
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
            age: 28,
            active: true,
        },
        User {
            _id: None,
            name: "User2".to_string(),
            age: 28,
            active: true,
        },
        User {
            _id: None,
            name: "User3".to_string(),
            age: 35,
            active: true,
        }
    ];

    for user in users {
        user.save().await?;
    }

    let matched_users = User::find(doc! { "age": 28 }).await?;
    assert_eq!(matched_users.len(), 2);

    let names: Vec<String> = matched_users
        .into_iter()
        .map(|u| u.name)
        .collect();
    assert!(names.contains(&"User1".to_string()));
    assert!(names.contains(&"User2".to_string()));

    Ok(())
}
