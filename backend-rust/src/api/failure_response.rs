#[derive(Debug, Serialize)]
pub struct FailureResponse<'a> {
    pub error: FailureData<'a>,
}

#[derive(Debug, Serialize)]
pub struct FailureData<'a> {
    pub code: i32,
    pub message: &'a str,
}

macro_rules! try_or_res {
    ($result:expr, $status_code:expr, $err_code:expr, $err_msg:expr) => {
        match $result {
            Ok(r) => r,
            Err(_e) => {
                let data = crate::api::failure_response::FailureResponse {
                    error: crate::api::failure_response::FailureData {
                        code: $err_code,
                        message: $err_msg,
                    },
                };
                return rouille::Response::json(&data).with_status_code($status_code);
            }
        }
    };
}
