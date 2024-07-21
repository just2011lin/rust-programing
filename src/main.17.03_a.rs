use rust_programing::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("今天中午我吃了沙拉");
    println!("content: {}", post.content());
    post.request_review();
    println!("content: {}", post.content());
    post.approve();
    println!("content: {}", post.content());
}
