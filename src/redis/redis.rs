use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitSink;
use futures_util::SinkExt; 
use std::time::Duration;
use redis::AsyncCommands; // Обязательно добавляем для поддержки простых команд вроде get/del

pub struct RedisListener {
    email: String,
    role: String,
    sender: SplitSink<WebSocket, Message>, 
}

impl RedisListener {
    pub async fn new(
        sender: SplitSink<WebSocket, Message>,
        email: String,
        role: String,
    ) -> Result<Self, redis::RedisError> {
        // Нам больше не нужен тяжелый pubsub при инициализации
        Ok(Self {
            email,
            role,
            sender,
        })
    }

    // ВАЖНО: теперь метод принимает &mut self, поэтому ошибка [E0507] на строке 119 ИСЧЕЗНЕТ!
    pub async fn start_listening(&mut self) -> Result<(), redis::RedisError> {
        // Подключаемся к редису как к обычной базе данных
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let mut con = client.get_multiplexed_tokio_connection().await?;

        // Ключ в редисе будет называться по роли или по email
        let redis_key = match self.role.as_str() {
            "Admin" => "update:Admin".to_string(),
            _ => format!("update:{}", self.email),
        };

        println!("🤖 [Redis] Запущен интервал проверки ключа '{}' раз в 5 секунд...", redis_key);

        loop {
            // Спим 5 секунд, чтобы не насиловать проц
            tokio::time::sleep(Duration::from_secs(5)).await;

            // GETDEL берет значение и СРАЗУ удаляет его из Редиса, чтобы не срабатывать повторно
            // Доступно в Redis 6.2+. Если редис старый, можно заменить на GET и DEL по отдельности
            let value: Option<String> = redis::cmd("GETDEL")
                .arg(&redis_key)
                .query_async(&mut con)
                .await
                .unwrap_or(None);

            if let Some(payload) = value {
                if payload == "1" || payload == "true" {
                    println!("🔥 [Redis Trigger] Ключ сработал для '{}'!", redis_key);
                    
                    do_work();

                    // Склеиваем наш джейсончик
                    let json_payload = serde_json::json!({
                        "type": "REFRESH_DATA",
                        "status": true
                    }).to_string();

                    // Отправляем JSON на фронтенд. Так как мы на &mut self, пишем self.sender
                    if self.sender.send(Message::Text(json_payload.into())).await.is_err() {
                        println!("🔌 [WS] Клиент отключился. Глушим таймер.");
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}

fn do_work() {
    println!("⚙️ [Engine] Работа началась и успешно выполнена.");
}
