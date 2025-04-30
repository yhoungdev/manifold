mod client;
use dotenv::dotenv;



#[tokio::main]
async fn main() {
    dotenv().ok();
    let plunk_pk:String = std::env::var("PLUNK_PRIVATE_KEY")
        .expect("Please set PLUNK_PRIVATE_KEY in .env");
    println!("{:?}", plunk_pk);
}
