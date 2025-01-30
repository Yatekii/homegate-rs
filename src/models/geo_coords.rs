use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeoCoords {
    pub latitude: f64,
    pub longitude: f64,
}
