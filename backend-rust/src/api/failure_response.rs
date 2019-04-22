use rouille::Response;

#[derive(Debug, Serialize)]
pub struct FailureData<'a> {
    pub error: InnerFailureData<'a>,
}

#[derive(Debug, Serialize)]
pub struct InnerFailureData<'a> {
    pub code: i32,
    pub message: &'a str,
}

#[derive(Debug)]
pub enum FailureResponse {
    InvalidPayload,
    Unauthorized,
    ServerError,
}

impl FailureResponse {
    pub fn unpack(&self) -> (u16, i32, &str) {
        use FailureResponse::*;
        match self {
            InvalidPayload => (400, 0, "Invalid payload"),
            Unauthorized => (401, 1, "Invalid token"),
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
            }
        };
        Response::json(&data).with_status_code(status_code)
    }
}
