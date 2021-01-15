pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    approvals: u8,
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            approvals: 0,
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> PendingReviewPost {
        PendingReviewPost {
            approvals: self.approvals + 1,
            content: self.content,
        }
    }

    pub fn commit(self) -> Option<Post> {
        if self.approvals >= 2 {
            return Some(Post {
                content: self.content,
            });
        }

        None
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
