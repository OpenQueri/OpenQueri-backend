use std::error::Error;

use redis::Client;
use redis::AsyncCommands;
use std::pin::Pin;


pub struct RedisStats{
    client: Option<Client>,
}

impl RedisStats {

    pub fn new() -> Self{
        let client = match redis::Client::open("redis://127.0.0.1:6379") {
            Ok(redis_connect) => Some(redis_connect),

            Err(e) => {
                eprintln!("Redis error: {}", e);
                None
            }
        };
        Self{
            client,
        }
    }

    pub async fn add_cout(&self){
        match &self.client {
            Some(client) => {

            let result = async {
                let mut conn = client.get_multiplexed_async_connection().await?;
                let key = "stats:counter_requests";
                let val = 1;
                let _: () = conn.incr(key, val).await?;
                let _: () = conn.publish(format!("{}:channel", key), val).await?;

                Ok::<(), Box<dyn Error>>(()) 
            }.await;

            match result {
                Ok(_) => println!("Ok Add +"),
                Err(_) => println!("Error"),         
            }

            }
            None => println!("No redis client"),
        }

    }

    pub async fn get_stats(&self) -> u32 {

        match &self.client {
            Some(client) => {

                let get_stats = async {
                    let mut conn = client.get_multiplexed_async_connection().await?;
                    let key = "stats:counter_requests";
                    let val: Option<u32> = conn.get(key).await?;

                    Ok::<Option<u32>, Box<dyn Error>>(val)
                }.await;


                let val = match get_stats {
                    Ok(ok) => {
                        match ok {
                            Some(val) => val,
                            None => 0,
                        }
                    }
                    Err(e) => {
                        eprintln!("Error get stats Redis: {}", e);
                        0 as u32
                    }
                };

                
                return val;
            }
            None => 0 as u32,
        }
}
    
}