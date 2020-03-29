use std::process::Command;

pub fn run(input_video: &str, times: Vec<Time>) {
    let status = Command::new("ffmpeg.exe")
        .args(&["-i", input_video])
        .args(&["-filter_complex", &create_filter_complex_argument(times)[..]])
        .args(&["-map", "[out]"])
        .arg("out.".to_string() + get_extension_from_filename(input_video))
        .status()
        .expect("failed to start ffmpeg")
    ;
    println!("process exited with: {}", status);
}

pub struct Time {
    start: u32,
    end: u32,
}

impl Time {
    pub fn new(start: u32, end: u32) -> Time {
        return Time { start, end };
    }
}

fn create_filter_complex_argument(slices_in_seconds: Vec<Time>) -> String {
    slices_in_seconds
        .iter()
        .enumerate()
        .map(|enum_time| format!("[0:v]trim={0}:{1},setpts=PTS-STARTPTS[v{2}];[0:a]atrim={0}:{1},asetpts=PTS-STARTPTS[a{2}];", enum_time.1.start, enum_time.1.end, enum_time.0))
        .collect::<Vec<String>>()
        .join("") +
        &slices_in_seconds.iter().enumerate().map(|enum_time| format!("[v{0}][a{0}]", enum_time.0))
            .collect::<Vec<String>>()
            .join("") + &format!("concat=n={}:v=1:a=1[out]", slices_in_seconds.len())[..]
}

fn get_extension_from_filename(filename: &str) -> &str {
    filename.split(".").last().unwrap()
}


#[cfg(test)]
mod test_lib {
    use super::*;

    #[test]
    fn create_args_empy_slices() {
        assert_eq!(
            "[0:v]trim=0:15,setpts=PTS-STARTPTS[v0];\
[0:a]trim=0:15,setpts=PTS-STARTPTS[a0];\
[0:v]trim=25:30,setpts=PTS-STARTPTS[v1];\
[0:a]trim=25:30,setpts=PTS-STARTPTS[a1];\
[0:v]trim=45:70,setpts=PTS-STARTPTS[v2];\
[0:a]trim=45:70,setpts=PTS-STARTPTS[a2];\
[v0][a0][v1][a1][v2][a2]concat=n=3:v=1:a=1[out]",
            create_filter_complex_argument(
                vec!(
                    Time::new(0, 15),
                    Time::new(25, 30),
                    Time::new(45, 70))
            )
        );
    }

    #[test]
    fn get_file_extension() {
        assert_eq!("avi", get_extension_from_filename("myfile.avi"));
        assert_eq!("", get_extension_from_filename(""));
        assert_eq!("avi", get_extension_from_filename("a.b.avi"));
        assert_eq!("avi", get_extension_from_filename("..avi"));
    }
}