use std::borrow::Cow;
use crate::search::model::Order;
use crate::search::repository::SearchRepository;

#[derive(Clone)]
pub struct SearchService {
    repository: SearchRepository,
}

impl SearchService {
    pub fn new(repository: SearchRepository) -> SearchService {
        SearchService { repository }
    }

    pub async fn search<'a>(&self, q: Option<&'a str>) -> Result<Vec<Order>, Cow<'a, str>> {
        self.repository
            .search(q)
            .await
            .map(|response| response.hits.hits.into_iter().map(|x| x._source).collect())
    }
}