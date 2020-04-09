use crop_server::routes;
use schemars::schema_for;
use std::fs::write;

macro_rules! write_json_schema {
    ($path: expr, $obj: ty) => {{
        let schema = schema_for!($obj);
        write($path, serde_json::to_string_pretty(&schema).unwrap()).unwrap();
    }};
}

fn main() {
    /*
    write_json_schema!(
        "api/contest/poll/post-res.json",
        routes::contest::poll::post::Response
    );
    */
}
