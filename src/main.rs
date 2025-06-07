use actix_web::{post, web::{self, Json}, App, HttpResponse, HttpServer};
use crate::io::{write_to_pca, angle_to_pulse_width};
use serde::Deserialize;

mod io;

const SERVO_CHANNEL: u8 = 0; // Servo connected to channel 0
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server!");
    println!("Listening on port, {}", PORT);
    HttpServer::new(|| App::new().service(deploy))
        .bind(("127.0.0.1", PORT))?
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

    let angle = payload.angle.clamp(-90.0, 90.0);
    let pwm_value = angle_to_pulse_width(angle);

    if let Err(e) = write_to_pca(SERVO_CHANNEL, pwm_value) {
        eprintln!("I2C Error: {:?}", e);
        HttpResponse::InternalServerError().body("Oh no, you messed up!");
    }

    HttpResponse::Ok().body("Completed!")
}
