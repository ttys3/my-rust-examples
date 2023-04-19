
use reqwest::{Client, header};
use tokio::io::{self, AsyncWriteExt};
use futures_util::stream::StreamExt;

// use async stream to get body of https://bing.com/translator
// and print headers and body to stdout

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "https://bing.com/translator";

    let client = Client::new();
    let res = client.get(url)
        .header(header::USER_AGENT, "reqwest")
        .send().await?;

    println!("Status: {:?}", res.status());
    println!("Version: {:?}", res.version());

    println!("begin headers: ------------------");
    // iterate over headers and print headers key value pairs
    for (key, value) in res.headers() {
        println!("{}: {:?}", key, value);
    }
    println!("end headers: ------------------");

    // print body to stdout using async stream
    // while let Some(chunk) = res.chunk().await? {
    //     io::stdout().write_all(&chunk).await?;
    // }

    let mut stream = res.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item?;
        io::stdout().write_all(&chunk).await?;
    }

    Ok(())
}