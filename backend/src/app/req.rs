use rouille::Request;

/// "foo"というkeyのパラメータを取得するとき
/// - "foo=bar"         => Some(["bar"])
/// - "foo=bar%2Choge"  => Some(["bar", "hoge"])
/// - "hoge=bar"        => None
///
/// # NOTE
/// "%2C"は","のパーセントエンコーディング (","はUTF-8で0x2C)
pub fn get_params<'a>(req: &'a Request, key: &str) -> impl Iterator<Item = &'a str> {
    req.raw_query_string()
        .split("&")
        .map(|q| q.split("="))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_params() {
        let req = rouille::Request::fake_http("GET", "/?foo=bar&hoge=fuga%2Cfuga2", vec![], vec![]);

        let mut param_foo = get_params(&req, "foo").unwrap();
        assert_eq!(param_foo.next(), Some("bar"));

        let mut param_hoge = get_params(&req, "hoge").unwrap();
        assert_eq!(param_hoge.next(), Some("fuga"));
        assert_eq!(param_hoge.next(), Some("fuga2"));

        assert!(get_params(&req, "hoo").is_none());
    }
}
