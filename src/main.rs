use chrono::Local;
use rand::seq::IteratorRandom;
use rand::Rng;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::{self, File};
use std::io::BufReader;

fn main() {
    let mut rng = rand::thread_rng();

    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    loop {
        let files = fs::read_dir("gaslight-selection").unwrap();
        let file = files
            .choose(&mut rng)
            .expect("Please add gaslight material in the gaslight selection folder")
            .unwrap();
        let name = file.file_name();
        let file = file.path();

        let file = BufReader::new(File::open(file).unwrap());

        println!("{} - Playing {}.", Local::now(), name.to_string_lossy());

        let source = Decoder::new(file).unwrap();
        let _ = stream_handle.play_raw(source.convert_samples());
        let tmp: u64 = rng.gen_range(600..3600);
        std::thread::sleep(std::time::Duration::from_secs(tmp));
    }
}
