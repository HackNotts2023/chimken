use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Votes(usize, usize);

impl Votes {
    #[inline]
    pub fn upvotes(&self) -> usize {
        self.0
    }

    #[inline]
    pub fn downvotes(&self) -> usize {
        self.1
    }

    #[inline]
    pub fn ratio(&self) -> f32 {
        self.upvotes() as f32 / self.downvotes() as f32
    }
}