use std::env;
use egg_mode::tweet::DraftTweet;

#[tokio::main]
async fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // let message = &args[1].clone() ;
        let con_token = egg_mode::KeyPair::new("consumer key", "consumer secret");
        let access_token = egg_mode::KeyPair::new("access token key", "access token secret");
        let token = egg_mode::Token::Access {
            consumer: con_token,
            access: access_token,
        };
        let _tweet = DraftTweet::new("ちくわ大明神").send(&token).await.unwrap();
        // println!("{}", &message);
    };
}
