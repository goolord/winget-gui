fn main() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
    windows_bindgen::bindgen([
        "--in",
        "default",
        &format!("{root}/winmd/Microsoft.Management.Deployment.winmd"),
        "--out",
        &format!("{root}/src/bindings.rs"),
        "--filter",
        "Microsoft.Management.Deployment",
        // Referenced by InstallOptions.AllowedArchitectures.
        "Windows.System.ProcessorArchitecture",
    ]);
}
