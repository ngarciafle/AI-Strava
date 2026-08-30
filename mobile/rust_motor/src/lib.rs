use std::sync::Mutex;

uniffi::setup_scaffolding!();

//IDEAS
//Implement the ability to create phases to restart rithm

#[derive(uniffi::Object)]
struct Point {
    latitude: f64,
    longitude: f64,
    altitude: f64,
}

#[derive(uniffi::Record)]
struct StatsTraining {
    distance: f64,
    elevation_gain: f64,
    elevation_loss: f64,
    rithm: f64,
    time: f64,
    timeRound: f64,
    rithms: Vec<f64>,
    times: Vec<f64>
}

struct TrainingState {
    vec_points: Vec<Point>,
    distance: f64,
    elevation_gain: f64,
    elevation_loss: f64,
    rithm: f64,
    time: f64,
    timeRound: f64,
    rithms: Vec<f64>,
    times: Vec<f64>,
}

#[derive(uniffi::Object)]
struct Training {
    state: Mutex<TrainingState>,
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
            state: Mutex::new(TrainingState {
                vec_points: Vec::new(),
                distance: 0.0,
                elevation_gain: 0.0,
                elevation_loss: 0.0,
                rithm: 0.0,
                time: 0.0,
                timeRound: 0.0,
                rithms: Vec::new(),
                times: Vec::new()
            })
        }
    }

    pub fn register_new_point(&self, latitude: f64, longitude: f64, altitude: f64, time: f64, timeRound: f64) -> StatsTraining {
        let mut state = self.state.lock().unwrap();
        
        let last_point = match state.vec_points.last() {
            Some(point) => Some((point.latitude, point.longitude, point.altitude)),
            None => (latitude, longitude, altitude).into(),
        };  

        state.vec_points.push(Point {
            latitude,
            longitude,
            altitude,
        });


        if let Some((last_lat, last_lon, last_alt)) = last_point {
            let distance = Self::calc_dist(
                last_lat,
                last_lon,
                latitude,
                longitude,
            );    
            state.distance += distance;

            (state.elevation_gain, state.elevation_loss) = Self::calc_elevation_gain_loss(
                last_alt,
                altitude,
            );    
        }
        

        // Rithm calcs the rithm of the last km
        let rithm: f64 = timeRound / (state.distance % 10.0);
        state.rithm = rithm;
        state.time = time;

        // Its not well implemented -> need to create a vec of distances or sth to calc rithms every km or zone
        if state.distance % 10.0 == 0.0 || state.rithms.is_empty() {
            state.rithms.push(rithm);
            state.times.push(timeRound);
            state.timeRound = 0.0
        } else {
            if let Some(last) = state.rithms.last_mut() {
                *last = rithm;
            }
        }

        println!("New point received: {}, {}", latitude, longitude);

        StatsTraining {
            distance: state.distance,
            elevation_gain: state.elevation_gain,
            elevation_loss: state.elevation_loss,
            rithm: state.rithm,
            time: state.time,
            timeRound: state.timeRound,
            rithms: state.rithms.clone(),
            times: state.times.clone(),
        }
    }
    
    pub fn end_training(&self) {
        // send results to db

    }
}