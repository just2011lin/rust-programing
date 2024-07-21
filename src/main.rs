use rust_programing::Post;
fn main() {
    let mut post = Post::new();
    post.add_text("今天中午我吃了鸡蛋");

    let post = post.request_review();
    let post = post.approve();
    println!("content: {}", post.content());
}
