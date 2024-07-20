use winresource::WindowsResource;

fn main() {
    if cfg!(target_os = "windows") {
        WindowsResource::new()
            .set_manifest_file("app.manifest")
            .compile()
            .unwrap();
    }
}
