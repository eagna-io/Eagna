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
     * GET /contests
     */
    write_json_schema!(
        "api/contests__get__res.json",
        routes::contests::get::ResBody
    );

    /*
     * POST /contests
     */
    write_json_schema!(
        "api/contests__post__req.json",
        routes::contests::post::ReqBody
    );
    write_json_schema!(
        "api/contests__post__res.json",
        routes::contests::post::ResBody
    );

    /*
     * GET /contests/:id
     */
    write_json_schema!(
        "api/contests_id__get__res.json",
        routes::contests::_id::get::ResBody
    );

    /*
     * POST /contests/:id/polls
     */
    write_json_schema!(
        "api/contests_id_polls__post__req.json",
        routes::contests::_id::polls::post::ReqBody
    );
    write_json_schema!(
        "api/contests_id_polls__post__res.json",
        routes::contests::_id::polls::post::ResBody
    );

    /*
     * PATCH /contests/:id/polls/:id
     */
    write_json_schema!(
        "api/contests_id_polls_id__patch__req.json",
        routes::contests::_id::polls::_id::patch::ReqBody
    );

    /*
     * POST /admins/me/access_tokens
     */
    write_json_schema!(
        "api/admins_me_access_tokens__post__req.json",
        routes::admins::me::access_tokens::post::ReqBody
    );
    write_json_schema!(
        "api/admins_me_access_tokens__post__res.json",
        routes::admins::me::access_tokens::post::ResBody
    );
}
