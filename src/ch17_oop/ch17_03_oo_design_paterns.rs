pub fn main() {
    use super::ch17_03_post::Post;
    let text = "中午我吃了沙拉";
    let mut post = Post::new();
    post.add_text(text);
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!(text, post.content());
}

pub fn main_a() {
    use super::ch17_03_post_a::Post;
    let text = "中午我吃了沙拉";
    let mut post = Post::new();
    post.add_text(text);
    let post = post.request_review();
    let post = post.approve();
    assert_eq!(text, post.content());
}
