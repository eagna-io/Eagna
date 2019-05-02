#[macro_use]
extern crate serde_derive;

use utils::user;

#[test]
fn test_apis() {
    spawn_server();

    let token = test_post_access_token();
    test_get_me(token.as_str());
    test_get_me_markets(token.as_str());
}

fn spawn_server() {
    std::thread::spawn(|| {
        librohan::Server::new_with_env().run("localhost:12098");
    });
}

// Returns access token
fn test_post_access_token() -> String {
    let mut data = std::collections::HashMap::new();
    data.insert("email", user::Alice.email);
    data.insert("hashed_pass", user::Alice.hashed_pass);

    let client = reqwest::Client::new();
    let mut res = client
        .post("http://localhost:12098/access_token")
        .json(&data)
        .send()
        .unwrap();
    assert_eq!(res.status().as_u16(), 200);

    #[derive(Deserialize)]
    struct RespBody {
        access_token: String,
    }

    let body = res.json::<RespBody>().unwrap();
    assert_eq!(body.access_token.len(), 64);

    body.access_token
}

fn test_get_me(token: &str) {
    let client = reqwest::Client::new();
    let mut res = client
        .get("http://localhost:12098/me")
        .bearer_auth(token)
        .send()
        .unwrap();
    assert_eq!(res.status().as_u16(), 200);

    #[derive(Deserialize)]
    struct RespBody {
        id: i32,
        name: String,
        email: String,
    }

    let body = res.json::<RespBody>().unwrap();
    assert_eq!(body.name, user::Alice.name);
    assert_eq!(body.email, user::Alice.email);
}

fn test_get_me_markets(token: &str) {
    let client = reqwest::Client::new();
    let mut res = client
        .get("http://localhost:12098/me/markets")
        .bearer_auth(token)
        .send()
        .unwrap();
    assert_eq!(res.status().as_u16(), 200);

    #[derive(Deserialize)]
    struct RespMarket {
        id: i32,
        title: String,
        short_desc: String,
        status: String,
        open_time: String,
        close_time: String,
    }

    let body = res.json::<Vec<RespMarket>>().unwrap();
    assert_eq!(body.len(), 1);
}
