use papers_openalex::{Domain, Field, ListParams, OpenAlexClient, Subfield, Topic};

pub struct OpenAlexFetcher {
    client: OpenAlexClient,
}

impl OpenAlexFetcher {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: OpenAlexClient::with_api_key(api_key),
        }
    }

    pub async fn fetch_all_domains(&self) -> Result<Vec<Domain>, Box<dyn std::error::Error>> {
        self.fetch_all(|page| async move {
            let params = ListParams::builder().per_page(200).page(page).build();
            self.client.list_domains(&params).await
        })
        .await
    }

    pub async fn fetch_all_fields(&self) -> Result<Vec<Field>, Box<dyn std::error::Error>> {
        self.fetch_all(|page| async move {
            let params = ListParams::builder().per_page(200).page(page).build();
            self.client.list_fields(&params).await
        })
        .await
    }

    pub async fn fetch_all_subfields(&self) -> Result<Vec<Subfield>, Box<dyn std::error::Error>> {
        self.fetch_all(|page| async move {
            let params = ListParams::builder().per_page(200).page(page).build();
            self.client.list_subfields(&params).await
        })
        .await
    }

    pub async fn fetch_all_topics(&self) -> Result<Vec<Topic>, Box<dyn std::error::Error>> {
        self.fetch_all(|page| async move {
            let params = ListParams::builder().per_page(200).page(page).build();
            self.client.list_topics(&params).await
        })
        .await
    }

    async fn fetch_all<T, F, Fut>(&self, mut fetch_fn: F) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        F: FnMut(u32) -> Fut,
        Fut: std::future::Future<Output = papers_openalex::Result<papers_openalex::ListResponse<T>>>,
    {
        let mut all = Vec::new();
        let mut page = 1u32;

        loop {
            let response = fetch_fn(page).await?;
            let count = response.results.len();
            all.extend(response.results);

            if count == 0 || all.len() >= response.meta.count as usize {
                break;
            }
            page += 1;
        }

        Ok(all)
    }
}
