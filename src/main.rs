use amiquip::Connection;
use docker_practice::config;
use postgres::{Client as PostgresClient, NoTls};
use redis::{Client as RedisClient, ConnectionLike};

fn main() {
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

  let redis_url: String = format!("redis://{}:6379", config::REDIS_HOST);
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
}
