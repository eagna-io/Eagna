use libeagna::domain::user::services::auth::UserAuthService;

fn main() {
    let cred = UserAuthService::derive_credentials("hogehoge");
    println!("salt : {:?}", cred.salt_hex());
    println!("cred : {:?}", cred.cred_hex());
}
