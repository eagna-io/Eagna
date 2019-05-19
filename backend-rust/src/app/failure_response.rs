use rouille::Response;

#[derive(Debug)]
pub enum FailureResponse {
    ResourceNotFound,
    InvalidPayload,
    Unauthorized,
    ServerError,
}

/* ## Error code 規約
 *
 * ### 0 ~ 100
 * クライアントの不備によるエラー
 *
 * ### 100 ~
 * サーバーの不備によるエラー
 */
impl FailureResponse {
    pub fn unpack(&self) -> (u16, i32, &str) {
        use FailureResponse::*;
        match self {
            ResourceNotFound => (404, 0, "Resource not found"),
            InvalidPayload => (400, 1, "Invalid payload"),
            Unauthorized => (401, 2, "Invalid token"),
            ServerError => (500, 100, "Server error"),
        }
    }
}

impl Into<Response> for FailureResponse {
    fn into(self) -> Response {
        let (status_code, err_code, err_msg) = self.unpack();
        let data = FailureData {
            error: InnerFailureData {
                code: err_code,
                message: err_msg,
            },
        };
        Response::json(&data).with_status_code(status_code)
    }
}

impl<E: failure::AsFail> From<E> for FailureResponse {
    fn from(e: E) -> FailureResponse {
        println!("Convert error into failure response");
        println!("{:?}", e.as_fail());
        FailureResponse::ServerError
    }
}

#[derive(Debug, Serialize)]
struct FailureData<'a> {
    error: InnerFailureData<'a>,
}

#[derive(Debug, Serialize)]
struct InnerFailureData<'a> {
    code: i32,
    message: &'a str,
}
