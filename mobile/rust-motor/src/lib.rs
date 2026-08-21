uniffi::setup_scaffolding!();

#[derive(uniffi::Object)]
struct Point {
    latitude: f64,
    longitude: f64,
    altitude: f64,
}

#[derive(uniffi::Object)]
struct StatsTraining {
    distance: f64,
    elevation_gain: f64,
    elevation_loss: f64,
}

// #[derive(uniffi::Object)]
// struct Training {
//     vec_points: Vec<Point>,
//     distance: f64,
//     elevation_gain: f64,
//     elevation_loss: f64,
// }

#[uniffi::export]
impl Training {
    const earth_rad: f64 = 6371000.0;
    const threshold_low: f64 = 1;
    const threshold_high: f64 = 1000;

    #[uniffi::constructor]
    pub fn new() -> Self {
        Training {
            vec_points: Vec::new(),
            distance: 0.0,
            elevation_gain: 0.0,
            elevation_loss: 0.0,
        }
    }

    #[uniffi::export]
    pub fn register_new_point(&mut self, latitude: f64, longitude: f64, altitude: f64) -> StatsTraining {
        self.vec_points.push(Point {
            latitude,
            longitude,
            altitude,
        });

        let last_point = match self.vec_points.last() {
            Some(point) => point,
            None => return StatsTraining { distance: 0.0, elevation_gain: 0.0, elevation_loss: 0.0 },
        };    
        
        self.distance += Self::calc_dist(
            last_point.latitude,
            last_point.longitude,
            latitude,
            longitude,
        );    

        (self.elevation_gain, self.elevation_loss) = Self::calc_elevation_gain_loss(
            last_point.altitude,
            altitude,
        );    

        println!("New point received: {}, {}", latitude, longitude);
        StatsTraining {
            distance: self.distance,
            elevation_gain: self.elevation_gain,
            elevation_loss: self.elevation_loss,
        }
    }
    
    #[uniffi::export]
    pub fn end_tracking(&mut self) {
        // send results to db

    }

    fn calc_dist(lat: f64, lon: f64, lat2: f64, lon2: f64) -> f64 {
        let d_lat = (lat2 - lat).to_radians();
        let d_lon = (lon2 - lon).to_radians();

        let a = (d_lat / 2.0).sin().powi(2)
            + lat.to_radians().cos() * lat2.to_radians().cos() * (d_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        Self::earth_rad * c
    }

    fn calc_elevation_gain_loss(alt1: f64, alt2: f64) -> (f64, f64) {
        let mut gain = if alt2 > alt1 { alt2 - alt1 } else { 0.0 };
        let mut loss = if alt1 > alt2 { alt1 - alt2 } else { 0.0 };

        if gain > Self::threshold_high && gain < Self::threshold_low {
            gain = 0.0;
        } 

        if loss > Self::threshold_high && loss < Self::threshold_low {
            loss = 0.0;
        }

        (gain, loss)
    }
}