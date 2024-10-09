use winresource::WindowsResource;

fn main() -> std::io::Result<()> {
    if cfg!(target_os = "windows") {
        let mut res = WindowsResource::new();

        res.set_icon("icon.ico")
            .set_version_info(winresource::VersionInfo::PRODUCTVERSION, 0x0001000000000000)
            .compile()?;
    }
    
    Ok(())
}
