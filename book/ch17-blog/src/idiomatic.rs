// @Note: by implementing the state pattern exactly as it's defined for object-oriented
// languages, as in object_oriented.rs, we're not taking as full advantage of Rust's
// strengths as we could. Here we make some changes that can make invalid states and
// transitions into compile time errors.
//
// Rather than encapsulating the states and transitions completely, so outside code
// has no knowledge of them, we'll encode the states into different types.

// @Note: the `Post` struct will represent a published post.
//
// The `request_review` and `approve` methods take ownership of `self`, thus consuming
// the `DraftPost` and `PendingReviewPost` instances and transforming them into a
// `PendingReviewPost` and a published `Post`, respectively.

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
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
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
