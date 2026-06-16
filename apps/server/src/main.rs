use orcid::Client as OrcidClient;

#[tokio::main]
async fn main() {
    let author = "0000-0003-0792-7733";

    let orcid_client = OrcidClient::new();
    let author = orcid_client.author(author).await.unwrap();
    let works = author.works();

    dbg!(works);
}
