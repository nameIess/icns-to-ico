fn main() {
    // Only compile Windows resources on Windows targets
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        embed_resource::compile("resources/app.rc", embed_resource::NONE);
    }
}
