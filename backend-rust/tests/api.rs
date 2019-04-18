#[macro_use]
extern crate serde_derive;

#[test]
fn test_apis() {
    spawn_server();

    let pg_conn = librohan::PgConnectionFactory::new_with_env()
        .establish_connection()
        .unwrap();

    test_post_access_token(&pg_conn);
}

fn spawn_server() {
    std::thread::spawn(|| {
        librohan::Server::new_with_env().run("localhost:12098");
    });
}

fn test_post_access_token(pg_conn: &diesel::pg::PgConnection) {
    let new_user = utils::NewUser {
        name: "Rohan",
        email: "rohan@rohanmarket.com",
        hashed_pass: "somethinghashedpassword",
    };
    utils::create_user(pg_conn, &new_user);

    let mut data = std::collections::HashMap::new();
    data.insert("email", new_user.email);
    data.insert("hashed_pass", new_user.hashed_pass);

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
}
