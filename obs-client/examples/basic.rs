use clap::Parser;
use obs_client::Capture;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    window_name: String,
}

fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .init()
        .unwrap();

    let args = Args::parse();

    let mut capture = Capture::new(&args.window_name);
    if capture.try_launch().is_err() {
        println!("Failed to launch the capture {}", &args.window_name);
        return;
    }

    let mut fps = fps_counter::FPSCounter::new();
    loop {
        let (buffer, (width, height)) = capture.capture_frame::<u8>().unwrap();
        println!("{:?} | {:?}x{:?} | {:?}", fps.tick(), width, height, buffer.len());
    }
}
