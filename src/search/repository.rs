use std::borrow::Cow;
use elasticsearch::{Elasticsearch, SearchParts};
use serde_json::json;
use crate::search::model::{ElasticResponse, Order};
use crate::settings;

#[derive(Clone)]
pub struct SearchRepository {
    client: Elasticsearch,
}

impl SearchRepository {
    pub fn new(client: Elasticsearch) -> SearchRepository {
        SearchRepository {
            client
        }
    }

    pub async fn search<'a>(&self, q: Option<&'a str>) -> Result<ElasticResponse<Order>, Cow<'a, str>> {
        let body = q.map(|n| json!({
            "query": {
                "term": {
                    "sku": {
                        "value": n
                    }
                }
            }
        })).unwrap_or(json!({
            "query": {
                "match_all": {}
            }
        }));

        let result = self.client
            .search(SearchParts::Index(&[&settings::SETTINGS.search_index]))
            .from(0)
            .size(10)
            .body(body)
            .send()
            .await;

        match result {
            Ok(response) => {
                let response_result = response.json::<ElasticResponse<Order>>().await;
                match response_result {
                    Ok(text) => Ok(text),
                    Err(e) => Err(Cow::from(e.to_string()))
                }
            }
            Err(e) => Err(Cow::from(e.to_string()))
        }
    }
}