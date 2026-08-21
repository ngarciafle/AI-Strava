uniffi::setup_scaffolding!();

//IDEAS
//Implement the ability to create phases to restart rithm



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
    rithm: f64,
    time: f64,
    rithms: Vec<f64>,
}

#[derive(uniffi::Object)]
struct Training {
    vec_points: Vec<Point>,
    distance: f64,
    elevation_gain: f64,
    elevation_loss: f64,
    rithm: f64,
    time: f64,
    rithms: Vec<f64>,
}

impl Training {
    const EARTH_RAD: f64 = 6371000.0;
    const THRESHOLD_LOW: f64 = 1.0;
    const THRESHOLD_HIGH: f64 = 1000.0;

    fn calc_dist(lat: f64, lon: f64, lat2: f64, lon2: f64) -> f64 {
        let d_lat = (lat2 - lat).to_radians();
        let d_lon = (lon2 - lon).to_radians();

        let a = (d_lat / 2.0).sin().powi(2)
            + lat.to_radians().cos() * lat2.to_radians().cos() * (d_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        Self::EARTH_RAD * c
    }

    fn calc_elevation_gain_loss(alt1: f64, alt2: f64) -> (f64, f64) {
        let mut gain = if alt2 > alt1 { alt2 - alt1 } else { 0.0 };
        let mut loss = if alt1 > alt2 { alt1 - alt2 } else { 0.0 };

        if gain > Self::THRESHOLD_HIGH || gain < Self::THRESHOLD_LOW {
            gain = 0.0;
        } 

        if loss > Self::THRESHOLD_HIGH || loss < Self::THRESHOLD_LOW {
            loss = 0.0;
        }

        (gain, loss)
    }
}

#[uniffi::export]
impl Training {

    #[uniffi::constructor]
    pub fn new() -> Self {
        Training {
            vec_points: Vec::new(),
            distance: 0.0,
            elevation_gain: 0.0,
            elevation_loss: 0.0,
            rithm: 0.0,
            time: 0.0,
            rithms: Vec::new(),
        }
    }

    pub fn register_new_point(&mut self, latitude: f64, longitude: f64, altitude: f64, time: f64) -> StatsTraining {
        self.vec_points.push(Point {
            latitude,
            longitude,
            altitude,
        });

        let last_point = match self.vec_points.last() {
            Some(point) => point,
            None => return StatsTraining { distance: 0.0, elevation_gain: 0.0, elevation_loss: 0.0, rithm: 0.0, time: 0.0, rithms: Vec::new() },
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

        self.rithm = self.distance / time;
        self.time = time;

        if (time % 10 == 0.0) {
            self.rithms.push(self.rithm);
        } else {
            *self.rithms.last_mut().unwrap() = self.rithm;
        }

        println!("New point received: {}, {}", latitude, longitude);
        StatsTraining {
            distance: self.distance,
            elevation_gain: self.elevation_gain,
            elevation_loss: self.elevation_loss,
            rithm: self.rithm,
            time: self.time,
            rithms: self.rithms.clone(),
        }
    }
    
    pub fn end_tracking(&mut self) {
        // send results to db

    }
}