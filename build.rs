use {
    std::env,
    winres::WindowsResource,
};

fn main() {
    slint_build::compile("ui/main.slint").unwrap();
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        // icon for windows' exe
        WindowsResource::new()
            .set_icon("./icon.ico")
            .compile().unwrap();
    }
}
