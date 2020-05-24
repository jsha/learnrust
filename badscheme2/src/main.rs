use hyper::{body::HttpBody as _, Client, Uri};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Make a GET /ip to 'http://httpbin.org'
    let res = client
        .get(Uri::from_static("hxxp://httpbin.org/ip"))
        .await?;

    // And then, if the request gets a response...
    println!("status: {}", res.status());

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(res).await?;

    println!("body: {:?}", buf);
    Ok(())
}
