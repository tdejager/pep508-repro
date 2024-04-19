use pep508_rs::{Requirement, VersionOrUrl};
use std::str::FromStr;

fn main() {
    let spec_str = "mypkg @ file:///C:/path/to/my_pkg";
    let spec = Requirement::from_str(spec_str).unwrap();
    let raw = uv_fs::normalize_url_path(pep508_rs::strip_host("///C:/path/to/my_pkg"));
    if let Some(VersionOrUrl::Url(url)) = spec.version_or_url {
        let path = url.path();
        assert_eq!(path, raw);
        println!("{path} == {raw}");
    } else {
        assert!(false, "expected url. Found: {spec_str}");
    }
}