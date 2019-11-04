use libeagna::domain::user::services::auth::UserAuthService;

fn main() {
    let cred = UserAuthService::derive_credentials("hogehoge");
    println!("{:?}", cred);
}
