uniffi::setup_scaffolding!();

#[derive(uniffi::Object)]
struct Point {
    latitude: f64,
    longitude: f64,
    altitude: f64,
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
    pub fn register_new_point(latitude: f64, longitude: f64, altitude: f64) {
        //Create a vec
        // Calc dist & altitude...
        // Send some feedback IRT
        vec_points.push(Point {
            latitude,
            longitude,
            altitude,
        });

        println!("New point received: {}, {}", latitude, longitude);
    }
    
    #[uniffi::export]
    pub fn end_tracking() {
    
    }
}