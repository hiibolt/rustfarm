use std::fs::read_to_string;

fn main() {
    use std::io::Write;
    // only build the resource for release builds
    // as calling rc.exe might be slow
    let mut res = winres::WindowsResource::new();

    // Vanity
    res.set_icon("build/icon.ico");

    // Metadata
    let manifest = match read_to_string("build/manifest.xml") {
        Ok(manifest) => manifest,
        Err(error) => {
            write!(std::io::stderr(), "{}", error).unwrap();
            std::process::exit(1);
        }
    };
    res.set_manifest(&manifest);

    // Compile the resources
    match res.compile() {
        Err(error) => {
            write!(std::io::stderr(), "{}", error).unwrap();
            std::process::exit(1);
        }
        Ok(_) => {}
    }
}