fn return_two() -> i32 {
    2
}

fn return_six() -> i32 {
    4 + return_two()
}

fn throw_panic() {
    panic!("expected panic")
}

// #[derive(PartialEq)]
#[derive(Debug)]
enum YoutubeStreamer {
    Rusman,
    Julio,
    Ramzi,
}

impl PartialEq for YoutubeStreamer {
    fn eq(&self, other: &YoutubeStreamer) -> bool {
        match (self, other) {
            // RUSMAN RAMZI CONNECTION!
            (YoutubeStreamer::Rusman, YoutubeStreamer::Ramzi) => true,
            (YoutubeStreamer::Ramzi, YoutubeStreamer::Rusman) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_six_ok() {
        assert_eq!(return_six(), 6)
    }

    #[test]
    #[should_panic]
    fn throw_panic_ok() {
        throw_panic()
    }

    #[test]
    fn youtube_streamer_eq() {
        assert_eq!(YoutubeStreamer::Rusman, YoutubeStreamer::Ramzi);
        assert_eq!(YoutubeStreamer::Ramzi, YoutubeStreamer::Rusman);
        assert_ne!(YoutubeStreamer::Rusman, YoutubeStreamer::Julio);
    }
}
