# AI-Strava
Implement in an Android App -> AI, DB optimization...

## Motor-rust steps
1. cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -o ../app/src/main/jniLibs build
2. cargo build
3. cargo run --bin uniffi_bindgen -- generate --library target/debug/rust_motor.dll --language kotlin --out-dir ../app/src/main/java/com/example/mobile/

## Backend
Backend will run on a pc opening a port with ngrok. Trying to simplify the process.
