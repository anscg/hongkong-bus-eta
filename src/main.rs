use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use rayon::prelude::*;

#[derive(Deserialize)]
struct RouteStopList {
    r#type: String,
    version: String,
    generated_timestamp: String,
    data: Vec<RouteMetadata>
}

#[derive(Deserialize)]
struct StopData {
    r#type: String,
    version: String,
    generated_timestamp: String,
    data: StopMetadata
}

#[derive(Deserialize)]
struct RouteMetadata {
    route: String,
    bound: String,
    service_type: String,
    seq: String,
    stop: String,
}

#[derive(Deserialize)]
struct StopMetadata {
    stop: String,
    name_en: String,
    name_tc: String,
    name_sc: String,
    lat: String,
    long: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let main_request  = reqwest::get("https://data.etabus.gov.hk/v1/transport/kmb/route-stop")
        .await?
        .json::<RouteStopList>()
        .await?;
    println!("{} {} {}", main_request.r#type, main_request.version, main_request.generated_timestamp);

    for route_meta in main_request.data {
        let stop_resp: StopData = reqwest::get("https://data.etabus.gov.hk/v1/transport/kmb/stop/".to_owned() + &route_meta.stop)
            .await?
            .json::<StopData>()
            .await?;
    }

    Ok(())
}
