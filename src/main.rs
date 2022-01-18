use anyhow::Context;
use futures::{stream, StreamExt};
use reqwest_middleware::ClientBuilder;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use std::io;
use std::io::BufRead;
use std::time::Duration;

const TIMEOUT_SECS: u64 = 2;
const RETRIES: u32 = 2;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // create Reqwest Client
    let reqwest = reqwest::Client::builder()
        .timeout(Duration::from_secs(TIMEOUT_SECS))
        .build()?;
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(RETRIES);
    let client = ClientBuilder::new(reqwest)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    // read from stdin
    let mut requests = vec![];
    for line in lines.by_ref() {
        requests.push(line?);
    }

    // make requests
    let bodies = stream::iter(requests)
        .map(|url| {
            let client = &client;
            async move {
                let response = client
                    .get(url.clone())
                    .send()
                    .await
                    .with_context(|| url.clone())?;

                response
                    .bytes()
                    .await
                    .map(|bytes| (url.clone(), bytes))
                    .with_context(|| url.clone())
            }
        })
        .buffer_unordered(num_cpus::get());

    // collect results
    bodies
        .for_each(|b| async {
            match b {
                Ok((url, bytes)) => {
                    println!("URL: {}", url);
                    match String::from_utf8(bytes.to_vec()) {
                        Ok(str) => {
                            println!("Body:");
                            println!("{}", str);
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }
                    println!();
                }
                Err(err) => {
                    eprintln!("Cannot get URL: {}", err);
                    err.chain()
                        .skip(1)
                        .for_each(|cause| eprintln!("cause: {}", cause));
                    std::process::exit(1);
                }
            }
        })
        .await;

    Ok(())
}
