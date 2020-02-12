use rouille::Response;

#[derive(Debug)]
pub enum FailureResponse {
    ResourceNotFound,
    InvalidPayload,
    Unauthorized,
    Conflict,
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
            Conflict => (409, 3, "Conflict"),
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

impl<E> From<E> for FailureResponse
where
    anyhow::Error: From<E>,
{
    fn from(e: E) -> FailureResponse {
        log::info!("Convert error into failure response");
        log::info!("    {:?}", anyhow::Error::from(e));
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
