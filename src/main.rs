use zerotier_controller;

#[tokio::main]
async fn main() {
    println!("{}", zerotier_controller::get_address().await.unwrap());
}
