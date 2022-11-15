use serde::{Serialize};

#[macro_use(defer)]
extern crate scopeguard;


pub struct KeyFinder {
    // TODO: state goes here
}

impl KeyFinder {
    pub fn new() -> Self {
        KeyFinder {  }
    }

    pub fn set_frame_rate(&mut self, _frame_rate:u32 ) {

    }

}

// use a type alias so we can change this later for opaque struct
type KeyFinderAudioDataPtr = *mut ::libc::c_void;

extern "C" {

    // intializer for the audio data
    pub fn kfwrapper__init_audio_data(frame_rate: u32) -> KeyFinderAudioDataPtr;

    // destructor for the audio data
    pub fn kfwrapper__destroy_audio_data(audio_data: KeyFinderAudioDataPtr);

    // add a number of samples to the audio data
    pub fn kfwrapper__add_to_samples(audio_data: KeyFinderAudioDataPtr, data: *const f32, data_size: u64 );

    // returns the current key of the audio data
    pub fn kfwrapper__key_of_audio(audio_data: KeyFinderAudioDataPtr) -> i32;

}

/*

// Static because it retains useful resources for repeat use
static KeyFinder::KeyFinder k;

// Build an empty audio object
KeyFinder::AudioData a;

// Prepare the object for your audio stream
a.setFrameRate(yourAudioStream.framerate);
a.setChannels(yourAudioStream.channels);
a.addToSampleCount(yourAudioStream.length);

// Copy your audio into the object
for (int i = 0; i < yourAudioStream.length; i++) {
  a.setSample(i, yourAudioStream[i]);
}

// Run the analysis
KeyFinder::key_t key = k.keyOfAudio(a);

*/


#[derive(Serialize, Debug, Copy, Clone)]
pub enum SongKey {
    CMaj,
    DfMaj,
    DMaj,
    EfMaj,
    EMaj,
    FMaj,
    GfMaj,
    GMaj,
    AfMaj,
    AMaj,
    BfMaj,
    BMaj,

    CMin,
    DfMin,
    DMin,
    EfMin,
    EMin,
    FMin,
    GfMin,
    GMin,
    AfMin,
    AMin,
    BfMin,
    BMin,

    Unknown,


}

impl SongKey {
    // Converts a LibKeyFinder key_t into a SongKey
    pub fn from_key_t(i:i32) -> SongKey {
        match i {
            0 => SongKey::AMaj,
            1 => SongKey::AMin,

            2 => SongKey::BfMaj,
            3 => SongKey::BfMin,

            4 => SongKey::BMaj,
            5 => SongKey::BMin,

            6 => SongKey::CMaj,
            7 => SongKey::CMin,


            8 => SongKey::DfMaj,
            9 => SongKey::DfMin,

            10 => SongKey::DMaj,
            11 => SongKey::DMin,

            12 => SongKey::EfMaj,
            13 => SongKey::EfMin,

            14 => SongKey::EMaj,
            15 => SongKey::EMin,

            16 => SongKey::FMaj,
            17 => SongKey::FMin,

            18 => SongKey::GfMaj,
            19 => SongKey::GfMin,

            20 => SongKey::GMaj,
            21 => SongKey::GMin,

            22 => SongKey::AfMaj,
            23 => SongKey::AfMin,

            _ => SongKey::Unknown,
        }
    }

    // Converts the key to a circle-of-fifths compatible notation
    pub fn to_circle_of_fifths(&self) -> String {
        String::from(match self {
            Self::AMaj => "11B",
            Self::AMin => "8A",

            Self::BfMaj => "6B",
            Self::BfMin => "3A",

            Self::BMaj => "1B",
            Self::BMin => "10A",

            Self::CMaj => "8B",
            Self::CMin => "5A",

            Self::DfMaj => "3B",
            Self::DfMin => "12A",

            Self::DMaj => "10B",
            Self::DMin => "7A",

            Self::EfMaj => "5B",
            Self::EfMin => "2A",

            Self::EMaj => "12B",
            Self::EMin => "9A",

            Self::FMaj => "7B",
            Self::FMin => "4A",

            Self::GfMaj => "2B",
            Self::GfMin => "11A",

            Self::GMaj => "9B",
            Self::GMin => "6A",

            Self::AfMaj => "4B",
            Self::AfMin => "1A",

            Self::Unknown => "Unknown",
        })
    }
}



#[derive(Serialize, Debug)]
pub struct SongMeta {
    pub path: String,
    pub artist: String,
    pub title: String,
    pub key: SongKey,

    // The circle-of-fifths key
    pub cof_key: String,
}




fn process_mp3_file(path: &str) -> Option<SongMeta> {


    use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
    use symphonia::core::errors::Error;
    use symphonia::core::formats::FormatOptions;
    use symphonia::core::io::MediaSourceStream;
    use symphonia::core::meta::MetadataOptions;
    use symphonia::core::probe::Hint;

    print!("File: {}\n", path);

    // Open the media source.
    let src = std::fs::File::open(&path).expect("failed to open media");

    // Create the media source stream.
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    // Create a probe hint using the file's extension. [Optional]
    let mut hint = Hint::new();
    hint.with_extension("mp3");

    // Use the default options for metadata and format readers.
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    // Probe the media source.
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .expect("unsupported format");

    // Get the instantiated format reader.
    let mut format = probed.format;

    // Find the first audio track with a known (decodeable) codec.
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .expect("no supported audio tracks");

    // find the sample rate
    let sample_rate = match track.codec_params.sample_rate {
        Some(rate) => rate,
        None => {
            panic!("Cannot find sample rate for track")
        },
    };

    // create audio data from samplerate
    let audio_data = unsafe { kfwrapper__init_audio_data(sample_rate) };
    defer! {
        unsafe { kfwrapper__destroy_audio_data(audio_data) }
    }

    print!("Sample rate: {}", sample_rate);

    // Use the default options for the decoder.
    let dec_opts: DecoderOptions = Default::default();

    // Create a decoder for the track.
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)
        .expect("unsupported codec");

    // Store the track identifier, it will be used to filter packets.
    let track_id = track.id;

    // The metadata for our song
    let mut song_meta = SongMeta {
        path: String::from(path),
        artist: String::from(""),
        title: String::from(""),
        key: SongKey::Unknown,
        cof_key: String::from("Unknown"),
     };

    // The decode loop.
    loop {
        // Get the next packet from the media format.
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::ResetRequired) => {
                // The track list has been changed. Re-examine it and create a new set of decoders,
                // then restart the decode loop. This is an advanced feature and it is not
                // unreasonable to consider this "the end." As of v0.5.0, the only usage of this is
                // for chained OGG physical streams.
                unimplemented!();
            }
            Err(_err) => {
                // A unrecoverable error occured, halt decoding.
                return Some(song_meta);
            }
        };

        // Consume any new metadata that has been read since the last packet.
        while !format.metadata().is_latest() {
            // Pop the old head of the metadata queue.
            format.metadata().pop();
            print!("--METADATA--\n");

            // Consume the new metadata at the head of the metadata queue.
            if let Some(rev) = format.metadata().current() {
                // Consume the new metadata at the head of the metadata queue.

                // TODO: get metadata from tags (but they don't seem to work for now)
                print!("\nTags: {:?}\n", rev.tags());
            }
        }

        // If the packet does not belong to the selected track, skip over it.
        if packet.track_id() != track_id {
            continue;
        }

        // Decode the packet into audio samples.
        match decoder.decode(&packet) {
            Ok(_decoded) => {
                // Consume the decoded audio samples (see below).
                use symphonia::core::audio::{AudioBufferRef, Signal};
                match _decoded {
                    AudioBufferRef::F32(buf) => {
                        let planes = buf.planes();
                        print!(".");
                        // check if we have audio channels
                        if planes.planes().len() == 0 {
                            print!("No audio channles available");
                            return None;
                        }

                        // use the first channel only (as we are mono)
                        let plane = planes.planes()[0];
                        unsafe { kfwrapper__add_to_samples(
                            audio_data,
                            plane.as_ptr(),
                            plane.len().try_into().unwrap()
                        ) };




                        // for plane in planes.planes() {
                        //     unsafe { kfwrapper__add_to_samples(
                        //         audio_data,
                        //         plane.as_ptr(),
                        //         plane.len().try_into().unwrap()
                        //     ) }
                            // // TODO: We have the block of samples here one channel, send to libKeyfinder here
                            // for &_sample in plane.iter() {

                            //     // Do something with `sample`.
                            // }
                        // }

                        // TODO: update the song key from the libkeyfinder instance
                        // let song_key = SongKey::Unknown;
                        let int_song_key = unsafe { kfwrapper__key_of_audio(audio_data) };
                        let song_key = SongKey::from_key_t(int_song_key);


                        song_meta.key = song_key;
                        song_meta.cof_key = song_key.to_circle_of_fifths();
                    }
                    _ => {
                        // Repeat for the different sample formats.
                        unimplemented!()
                    }
                }
            }
            Err(Error::IoError(_)) => {
                // The packet failed to decode due to an IO error, skip the packet.
                continue;
            }
            Err(Error::DecodeError(_)) => {
                // The packet failed to decode due to invalid data, skip the packet.
                continue;
            }
            Err(err) => {
                // An unrecoverable error occured, halt decoding.
                panic!("Decode error - {}", err);
            }
        }
    }
}



fn main() {

    let args: Vec<String> = std::env::args().collect();

    let song_meta = process_mp3_file(&args[1]);


    // let song_meta = process_mp3_file("/Users/gyulalaszlo/Music/Reaper Projects/set_preparation/dj-2022-09-08/songs/13-shelter97-156bpm.mp3");

    print!("Song meta: {:?}\n ", song_meta)
}
