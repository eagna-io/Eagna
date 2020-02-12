use crop_domain::user::services::auth::UserAuthService;
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();

    match args {
        Args::Credential { pass } => {
            let cred = UserAuthService::derive_credentials(pass.as_str());
            println!("salt : {:?}", cred.salt_hex());
            println!("cred : {:?}", cred.cred_hex());
        }
    }
}

#[derive(StructOpt, Debug)]
enum Args {
    Credential {
        #[structopt(short, long)]
        pass: String,
    },
}
