pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    // @Note: to consume the old state, the `request_review` method needs to
    // take ownership of the state value. This is where the `Option` in the
    // `state` field comes in: we call the `take` method to take the `Some`
    // value out of the `state` field and leave a `None` in its place,
    // because Rust doesn't let us have unpopulated fields in structs.
    //
    // This lets us move the `state` value out of `Post` rather than borrowing it.
    // Then we'll set the post's `state` value to the result of this operation.
    pub fn request_review(&mut self) {
        // @Note: we need to set `state` to `None` temporarily rather than
        // setting it directly with code like shown below to get ownership
        // of the `state` value.:
        //  |
        //  |   self.state = self.state.request_review();
        //
        // This ensures `Post` can't use the old `state` value after we've
        // transformed it into a new state.
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// @Note: one downside of the state pattern is that, because the states implement the
// transitions between states, some of the states are coupled to each other.
//
// Another downside is that we've duplicated some logic. To eliminate some of the
// duplication, we might try to make default implementations for the `request_review`
// and `approve` methods on the `State` trait that return `self`; however, this would
// violate object safety, because the trait doesn't know what the concrete `self` will be.

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    #[allow(unused_variables)]
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}