use http;
use std::env;
use url;

fn main() {
    for u in env::args().skip(1) {
        let parsed_url = url::Url::parse(&u).unwrap();
        let serialized_url = parsed_url.to_string();

        let parsed_uri = u.parse::<http::Uri>().unwrap();
        let serialized_uri = parsed_uri.to_string();
        print!(
            "url: {}\nparsed_url: {}\nserialized_url: {}\n\nparsed_uri: {}\nserialized_uri: {}\n",
            u, parsed_url, serialized_url, parsed_uri, serialized_uri
        );
    }
}
