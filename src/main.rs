use vclipper::{run, Time};

fn main() {
    run("test-video.mp4", vec!(Time::new(0, 5),Time::new(10, 15)));
}
