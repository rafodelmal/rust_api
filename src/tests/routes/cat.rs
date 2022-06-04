use reqwest;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::models::cat::PublicCat;
use crate::tests::setup::with_app;
use crate::tests::utils::create_user;
use crate::tests::utils::create_user_token;

#[test]
fn post_cat_route() {
  #[derive(Debug, Serialize, Deserialize)]
  struct Body {
    name: String,
  }

  let body = Body {
    name: "Tigrin".to_owned(),
  };

  with_app(async move {
    let user = create_user("nico@test.com").await.unwrap();
    let token = create_user_token(user.clone()).await.unwrap();

    let client = reqwest::Client::new();
    let res = client
      .post("http://localhost:8088/cats")
      .header("Authorization", format!("Bearer {}", token))
      .json(&body)
      .send()
      .await
      .unwrap();

    // Status code:
    let status_code = res.status();
    let actual = status_code;
    let expected = StatusCode::OK;
    assert_eq!(actual, expected);

    // Body:
    let body = res.json::<PublicCat>().await.unwrap();
    assert_eq!(body.name, "Tigrin");
    assert_eq!(body.user, user.id.unwrap(), "Cat should belong to user");
  });
}