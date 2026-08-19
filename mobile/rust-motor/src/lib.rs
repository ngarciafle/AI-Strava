#[uniffi::export]
pub fn register_new_point(latitude: f64, longitude: f64, altitude: f64) {
    //Create a vec
    // Calc dist & altitude...
    // Send some feedback IRT
    println!("New point received: {}, {}", latitude, longitude);
}