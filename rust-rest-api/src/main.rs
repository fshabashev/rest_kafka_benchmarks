use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use kafka::producer::{Producer, Record};
use std::sync::Mutex;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Index page test")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


fn send_line_to_kafka(producer:  &mut Producer, data_to_send: &str){
    producer.send(&Record::from_value("test", data_to_send.as_bytes())).unwrap();
}

struct AppState {
    kafka_producer: Mutex<Producer>
}

fn get_kafka_producer() -> Producer {
    let hosts = vec!["localhost:29092".to_owned()];
    Producer::from_hosts(hosts)
        .create()
        .unwrap()
}

#[post("/kafka_endpoint")]
async fn kafka_endpoint(req_body: String, data: web::Data<AppState>) -> impl Responder {
    let mut producer = data.kafka_producer.lock().unwrap();
    let record = Record::from_value("test", req_body.as_bytes());
    (*producer).send(&record).unwrap();
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                kafka_producer: Mutex::new(get_kafka_producer())
            }).clone())
            .service(hello)
            .service(echo)
            .service(kafka_endpoint)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}