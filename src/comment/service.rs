use crate::comment::model::CommentResponse;
use crate::comment::repository::CommentRepository;

#[derive(Clone)]
pub struct CommentService {
    repository: CommentRepository,
}

impl CommentService {
    pub fn new(repository: CommentRepository) -> CommentService {
        CommentService { repository }
    }

    pub async fn get(&self, name: Option<&String>, size: i64) -> Result<Vec<CommentResponse>, String> {
        let results = self.repository.get(name, size).await;

        match results {
            Ok(vec) => Ok(
                vec.into_iter().map(|item| CommentResponse::new(item)).collect()
            ),
            Err(e) => Err(e)
        }
    }

    pub async fn get_by_id(&self, id: &str) -> Result<CommentResponse, String> {
        self.repository.get_by_id(id).await.map(|comment| CommentResponse::new(comment))
    }
}