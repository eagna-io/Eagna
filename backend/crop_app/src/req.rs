use rouille::Request;

/// "foo"というkeyのパラメータを取得するとき
/// - "foo=bar"         => ["bar"]
/// - "foo=bar%2Choge"  => ["bar", "hoge"]
/// - "hoge=bar"        => []
///
/// # NOTE
/// "%2C"は","のパーセントエンコーディング (","はUTF-8で0x2C)
pub fn get_params<'a>(req: &'a Request, key: &str) -> impl Iterator<Item = &'a str> {
    req.raw_query_string()
        .split('&')
        .map(|q| q.split('='))
        .find_map(|mut q| {
            if q.next()? == key {
                Some(q.next()?)
            } else {
                None
            }
        })
        .map(|v| v.split("%2C"))
        .into_iter()
        .flatten()
}

pub fn get_param<'a>(req: &'a Request, key: &str) -> Option<&'a str> {
    get_params(req, key).next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_params() {
        let req = rouille::Request::fake_http("GET", "/?foo=bar&hoge=fuga%2Cfuga2", vec![], vec![]);

        let mut param_foo = get_params(&req, "foo");
        assert_eq!(param_foo.next(), Some("bar"));
        assert_eq!(param_foo.next(), None);

        let mut param_hoge = get_params(&req, "hoge");
        assert_eq!(param_hoge.next(), Some("fuga"));
        assert_eq!(param_hoge.next(), Some("fuga2"));
        assert_eq!(param_hoge.next(), None);

        assert_eq!(get_params(&req, "hoo").next(), None);
    }
}
