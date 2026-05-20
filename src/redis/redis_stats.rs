use std::error::Error;
use redis::{Client, AsyncCommands};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub struct RedisStats {
    pub client: Option<Client>,
    pub buffer: Arc<AtomicUsize>,
}

impl RedisStats {
    pub fn new() -> Self {
        let client = match redis::Client::open("redis://127.0.0.1:6379") {
            Ok(redis_connect) => Some(redis_connect),
            Err(e) => {
                eprintln!("Redis error: {}", e);
                None
            }
        };
        
        let buffer = Arc::new(AtomicUsize::new(0));
        let stats = Self { client, buffer };
        
        stats.start_sync_worker();
        
        stats
    }

    pub async fn add_cout(&self) {
        self.buffer.fetch_add(1, Ordering::Relaxed);
    }

    fn start_sync_worker(&self) {
        let buffer = self.buffer.clone();
        let client_opt = self.client.clone();

        tokio::spawn(async move {
            let Some(client) = client_opt else { return };
            
            let mut conn = match client.get_multiplexed_async_connection().await {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Worker Redis connection error: {}", e);
                    return;
                }
            };

            loop {
                sleep(Duration::from_millis(1000)).await;

                let count = buffer.swap(0, Ordering::Relaxed);
                if count > 0 {
                    let key = "stats:counter_requests";
                    let res: redis::RedisResult<()> = async {
                        let _: () = conn.incr(key, count).await?;
                        let _: () = conn.publish(format!("{}:channel", key), count).await?;
                        Ok(())
                    }.await;

                    if let Err(e) = res {
                        eprintln!("Sync Error: {}", e);
                    }
                }
            }
        });
    }

    pub async fn get_stats(&self) -> u32 {
        match &self.client {
            Some(client) => {

                let result = async {
                    let mut conn = client.get_multiplexed_async_connection().await?;
                    let key = "stats:counter_requests";
                    let val: Option<u32> = conn.get(key).await?;
                    Ok::<u32, Box<dyn Error + Send + Sync>>(val.unwrap_or(0))
                }.await;

                match result {
                    Ok(val) => val,
                    Err(e) => {
                        eprintln!("Error get stats Redis: {}", e);
                        0
                    }
                }
            }
            None => 0,
        }
    }
}
