fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .type_attribute("UserReply", "#[derive(serde::Serialize)]")
        .out_dir("src/grpc")
        .compile(
            &["src/user/jwt.proto", "src/user/user.proto"],
            &["proto"],
        )
        .unwrap();
    Ok(())
}
