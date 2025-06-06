use actix_web::{post, web::{self, Json}, App, HttpResponse, HttpServer};

use crate::io::angle_to_pulse_width;

mod io;

const SERVO_CHANNEL: u8 = 0; // Servo connected to channel 0

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(deploy))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[post("/deploy")]
async fn deploy(>) -> HttpResponse {
    // let angle = payload.angle.clamp(0.0, 180.0);
    let pwm_value = angle_to_pulse_width(angle);

    if let Err(e) = write_to_pca9685(SERVO_CHANNEL, pwm_value) {
        eprintln!("I2C Error: {:?}", e);
        HttpResponse::InternalServerError();
    }

    HttpResponse::Ok()
}
