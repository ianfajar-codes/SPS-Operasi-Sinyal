mod signal;
mod compute;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignalParams {
    pub a1: f64,
    pub f1: f64,
    pub phi1: f64,
    pub a2: f64,
    pub f2: f64,
    pub phi2: f64,
    pub operation: String,
    pub samples: usize,
    pub fs: f64,
}

#[derive(Serialize)]
pub struct SignalData {
    pub t: Vec<f64>,
    pub x1: Vec<f64>,
    pub x2: Vec<f64>,
    pub y: Vec<f64>,
    pub max_amplitude: f64,
    pub rms: f64,
}

async fn process_signal(params: web::Json<SignalParams>) -> impl Responder {
    let t = signal::generate_time_vector(params.samples, params.fs);
    
    let x1 = signal::generate_signal(params.a1, params.f1, params.phi1, &t);
    let x2 = signal::generate_signal(params.a2, params.f2, params.phi2, &t);
    
    let y = compute::perform_operation(&x1, &x2, &params.operation);
    
    let max_amplitude = compute::calculate_max_amplitude(&y);
    let rms = compute::calculate_rms(&y);
    
    let response = SignalData {
        t,
        x1,
        x2,
        y,
        max_amplitude,
        rms,
    };
    
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("===========================================");
    println!("Signal Processing Server");
    println!("===========================================");
    println!("Server berjalan di: http://127.0.0.1:8080");
    println!("Endpoint: POST /process");
    println!("===========================================\n");
    
    HttpServer::new(|| {
        let cors = Cors::permissive();
        
        App::new()
            .wrap(cors)
            .route("/process", web::post().to(process_signal))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
