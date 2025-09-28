fn main() {
    napi_build::setup();

    // Add post-install hook for cargo install
    if std::env::var("CARGO_FEATURE_INSTALL").is_ok() {
        println!("cargo:warning=Running post-install setup...");

        // The actual PATH setup will be handled by the install script
        // This just provides a hook point for cargo install
    }
}
