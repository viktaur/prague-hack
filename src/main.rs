use actix_web::{post, web::{self, Json}, App, HttpResponse, HttpServer};
use crate::io::{write_to_pca, angle_to_pulse_width};
use serde::Deserialize;

mod io;

const SERVO_CHANNEL: u8 = 0; // Servo connected to channel 0
const PORT: u16 = 8080;
const MIN_ANGLE: f32 = -90.0;
const MAX_ANGLE: f32 = 90.0;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server!");
    println!("Listening on port, {}", PORT);
    HttpServer::new(|| App::new()
        .service(deploy)
        .service(reset)
        .service(close)
    )
        // .bind(("127.0.0.1", PORT))?
        .bind(("0.0.0.0", PORT))?
        .run()
        .await
}

#[derive(Deserialize, Debug)]
struct DeployPayload {
    angle: f32,
}

#[post("/deploy")]
async fn deploy(payload: Json<DeployPayload>) -> HttpResponse {
    println!("Received: {:?}", payload);

    let pwm_value = angle_to_pulse_width(payload.angle);

    if let Err(e) = write_to_pca(SERVO_CHANNEL, pwm_value) {
        eprintln!("I2C Error: {:?}", e);
        HttpResponse::InternalServerError().body("Oh no, you messed up!");
    }

    HttpResponse::Ok().body("Completed!")
}

#[post("/reset")]
async fn reset() -> HttpResponse {
    println!("Received reset command");

    let pwm_value = angle_to_pulse_width(MAX_ANGLE);

    if let Err(e) = write_to_pca(SERVO_CHANNEL, pwm_value) {
        eprintln!("I2C Error: {:?}", e);
        HttpResponse::InternalServerError().body("Oh no, you messed up!");
    }

    HttpResponse::Ok().body("Completed!")
}

#[post("/close")]
async fn close() -> HttpResponse {
    println!("Received close command");

    let pwm_value = angle_to_pulse_width(MIN_ANGLE);

    if let Err(e) = write_to_pca(SERVO_CHANNEL, pwm_value) {
        eprintln!("I2C Error: {:?}", e);
        HttpResponse::InternalServerError().body("Oh no, you messed up!");
    }

    HttpResponse::Ok().body("Completed!")
}
