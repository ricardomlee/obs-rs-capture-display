use clap::Parser;
use obs_client::Capture;
use std::{
    thread::sleep,
    time::{Duration, SystemTime},
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    window_name: String,

    #[clap(long)]
    fps: u16,
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

    let duration = Duration::from_millis((1000 / args.fps).into());
    let mut last_time = SystemTime::now();

    let mut fps = fps_counter::FPSCounter::new();
    loop {
        let cur_time = SystemTime::now();
        let diff_time = cur_time.duration_since(last_time).expect("can not get time diff");
        if diff_time >= duration {
            let (buffer, (width, height)) = capture.capture_frame::<u8>().unwrap();
            println!("{:?} | {:?}x{:?} | {:?}", fps.tick(), width, height, buffer.len());

            last_time = cur_time;
        } else {
            sleep(duration - diff_time);
        }
    }
}
