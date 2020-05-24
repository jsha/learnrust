use hyper::Uri;
use reqwest::{blocking, Url};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use hyper::{Body, Client, Request};

    let client = Client::new();

    //let uri = "hxtps://example.com".parse::<Uri>().unwrap();

    //let req = Request::builder()
    //.method("GET")
    //.uri(uri)
    //.body(Body::from(""))
    //.expect("builder");
    //client.request(req).await?;

    let url = Url::parse("hxxp://example.com").unwrap();
    let uri2: http::Uri = url.as_str().parse().expect("yah");

    let req2 = Request::builder()
        .method("GET")
        .uri(uri2)
        .body(Body::from(""))
        .expect("builder");
    client.request(req2).await?;

    Ok(())
}
