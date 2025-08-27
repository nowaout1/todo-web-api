fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(
            &["./todo-service-proto/v1/todo.proto"],
            &["./todo-service-proto/v1/"],
        )?;

    Ok(())
}
