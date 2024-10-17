#![windows_subsystem = "windows"]

mod overlay;
mod data;

use overlay::OverlayApp;
use winit::event_loop::{ControlFlow, EventLoop};


fn init_logger() -> Result<(), flexi_logger::FlexiLoggerError> {
    let log_spec = flexi_logger::LogSpecBuilder::new()
        .default(log::LevelFilter::Warn)
        .module("centered_windows", log::LevelFilter::Debug)
        .build();

    let file_spec = flexi_logger::FileSpec::default()
        .directory("log")
        .use_timestamp(false);

    flexi_logger::Logger::with(log_spec)
        .log_to_file(file_spec)
        .format(|writer, now, record| {
            writer.write_fmt(format_args!(
                "[{} {}]: {}",
                now.now().format("%Y-%m-%d %H:%M:%S"),  // Timestamp.
                record.level(),                             // Log Level.
                &record.args(),                             // Message.
            ))
        })
        .rotate(
            flexi_logger::Criterion::Age(flexi_logger::Age::Day),
            flexi_logger::Naming::TimestampsDirect,
            flexi_logger::Cleanup::KeepLogFiles(7),
        )
        .start()?;

    Ok(())
}

fn main() {
    init_logger().expect("Failed to initialize logger");

    let event_loop = EventLoop::new().unwrap();
    let mut app = OverlayApp::new();
    
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut app)
        .expect("Centered Windows encountered an error");
}
