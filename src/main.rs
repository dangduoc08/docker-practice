use amiquip::Connection;
use docker_practice::config;
use postgres::{Client as PostgresClient, NoTls};
use redis::{Client as RedisClient, ConnectionLike};
use std::thread;

fn main() {
  let mut join_handlers: Vec<thread::JoinHandle<()>> = Vec::new();

  let psql_join = thread::spawn(|| {
    let psql_conn: String = format!(
      "host={} user={} password={}",
      config::POSTGRE_HOST,
      config::POSTGRE_USER,
      config::POSTGRE_PASS
    );
    let psql_client = match PostgresClient::connect(psql_conn.as_str(), NoTls) {
      Ok(c) => c,
      Err(e) => {
        println!("PostgreSQL connect failed!");
        panic!("{}", e)
      }
    };
    if !psql_client.is_closed() {
      println!("PostgreSQL has connected!");
    }
  });
  join_handlers.push(psql_join);

  let redis_join = thread::spawn(|| {
    let redis_url: String = format!("redis://{}", config::REDIS_HOST);
    match RedisClient::open(redis_url) {
      Ok(mut c) => {
        if c.check_connection() {
          println!("Redis has connected!");
        } else {
          println!("Redis connect failed!");
          panic!()
        }
      }
      Err(e) => panic!("{}", e),
    };
  });
  join_handlers.push(redis_join);

  let rabbit_join = thread::spawn(|| {
    let rabbitmq_url: String = format!("amqp://guest:guest@{}:5672", config::RABBITMQ_HOST);
    match Connection::insecure_open(rabbitmq_url.as_str()) {
      Ok(_conn) => {
        println!("RabbitMQ has connected!");
      }
      Err(e) => {
        println!("RabbitMQ connect failed!");
        panic!("{}", e)
      }
    }
  });
  join_handlers.push(rabbit_join);

  for join_handle in join_handlers {
    join_handle.join().unwrap();
  }
}
