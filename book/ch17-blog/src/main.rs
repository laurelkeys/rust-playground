use ch17_blog as blog;

fn main() {
    // Using the state design pattern we implement a blog post
    // workflow in an incremental way, which looks like:
    //
    //  1. A blog post starts as an empty draft.
    //  2. When the draft is done, a review of the post is requested.
    //  3. When the post is approved, it gets published.
    //  4. Only published blog posts return content to print, so unapproved
    //     posts can't accidentally be published.
    //
    // The logic related to the rules lives in the state objects
    // rather than being scattered throughout `Post`.

    // Object-oriented implementation.
    {
        use blog::object_oriented::*;

        let mut post = Post::new();

        // Draft state.
        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        // Pending review state.
        post.request_review();
        assert_eq!("", post.content());

        // Published state.
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }

    // "Idiomatic" Rust implementation.
    {
        use blog::idiomatic::*;

        let mut post = Post::new(); // `DraftPost`

        post.add_text("I ate a salad for lunch today");

        let post = post.request_review(); // `PendingReviewPost`

        let post = post.approve(); // `Post`

        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
