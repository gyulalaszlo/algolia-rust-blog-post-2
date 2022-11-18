use serde::{Deserialize, Serialize};

#[macro_use(defer)]
extern crate scopeguard;

pub struct KeyFinder {
    // TODO: state goes here
}

impl KeyFinder {
    pub fn new() -> Self {
        KeyFinder {}
    }

    pub fn set_frame_rate(&mut self, _frame_rate: u32) {}
}

// use a type alias so we can change this later for opaque struct
type KeyFinderAudioDataPtr = *mut ::libc::c_void;

/*

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
*/
// intializer for the audio data
pub fn kfwrapper__init_audio_data(frame_rate: u32) -> KeyFinderAudioDataPtr {
    std::ptr::null_mut()
}

// destructor for the audio data
pub fn kfwrapper__destroy_audio_data(audio_data: KeyFinderAudioDataPtr) {}

// add a number of samples to the audio data
pub fn kfwrapper__add_to_samples(
    audio_data: KeyFinderAudioDataPtr,
    data: *const f32,
    data_size: u64,
) {
}

// returns the current key of the audio data
pub fn kfwrapper__key_of_audio(audio_data: KeyFinderAudioDataPtr) -> i32 {
    1
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
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

impl std::convert::From<String> for SongKey {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "AMaj" => SongKey::AMaj,
            "AMin" => SongKey::AMin,

            "BfMaj" => SongKey::BfMaj,
            "BfMin" => SongKey::BfMin,

            "BMaj" => SongKey::BMaj,
            "BMin" => SongKey::BMin,

            "CMaj" => SongKey::CMaj,
            "CMin" => SongKey::CMin,

            "DfMaj" => SongKey::DfMaj,
            "DfMin" => SongKey::DfMin,

            "DMaj" => SongKey::DMaj,
            "DMin" => SongKey::DMin,

            "EfMaj" => SongKey::EfMaj,
            "EfMin" => SongKey::EfMin,

            "EMaj" => SongKey::EMaj,
            "EMin" => SongKey::EMin,

            "FMaj" => SongKey::FMaj,
            "FMin" => SongKey::FMin,

            "GfMaj" => SongKey::GfMaj,
            "GfMin" => SongKey::GfMin,

            "GMaj" => SongKey::GMaj,
            "GMin" => SongKey::GMin,

            "AfMaj" => SongKey::AfMaj,
            "AfMin" => SongKey::AfMin,

            _ => SongKey::Unknown,
        }
    }
}

impl SongKey {
    // Converts a LibKeyFinder key_t into a SongKey
    pub fn from_key_t(i: i32) -> SongKey {
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

    // returns a list of compatible keys
    // TODO: fill this matrix
    pub fn compatible_keys(&self) -> Vec<SongKey> {
        match self {
            Self::AMaj => vec![],
            Self::AMin => vec![
                SongKey::CMaj,
                SongKey::AMin,
                SongKey::DMin,
                SongKey::FMaj,
                SongKey::EMin,
                SongKey::GMaj,
            ],

            Self::BfMaj => vec![],
            Self::BfMin => vec![],

            Self::BMaj => vec![],
            Self::BMin => vec![],

            Self::CMaj => vec![],
            Self::CMin => vec![],

            Self::DfMaj => vec![],
            Self::DfMin => vec![],

            Self::DMaj => vec![],
            Self::DMin => vec![],

            Self::EfMaj => vec![],
            Self::EfMin => vec![],

            Self::EMaj => vec![],
            Self::EMin => vec![],

            Self::FMaj => vec![],
            Self::FMin => vec![],

            Self::GfMaj => vec![],
            Self::GfMin => vec![],

            Self::GMaj => vec![],
            Self::GMin => vec![],

            Self::AfMaj => vec![],
            Self::AfMin => vec![],

            Self::Unknown => vec![],
        }
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
        }
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
                        unsafe {
                            kfwrapper__add_to_samples(
                                audio_data,
                                plane.as_ptr(),
                                plane.len().try_into().unwrap(),
                            )
                        };

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

// SEARCH THINGS
// -------------

// The reqwest client type used by the library
type ClientType = reqwest::blocking::Client;

// Searches for a song with a compatible key to the specified one
pub fn search_algolia_for_song_by_key( app_id: &str, api_key: &str, index_name: &str, key: SongKey, user_query: &str) -> Result<Vec<SongMeta>, String> {
    use url::form_urlencoded::{byte_serialize};

    // encode user data for URLs
    fn url_encode(s: &str) -> String {
        byte_serialize(s.as_bytes()).collect()
    }

    fn build_filter_value(keys: &Vec<SongKey>) -> String {
        let keys_strings: Vec<String> = keys
            .iter()
            .map(|key| format!("cof_key:\"{}\"", key.to_circle_of_fifths()))
            .collect();
        keys_strings.join(" OR ")
    }

    fn build_query_string(query: &str, keys: &Vec<SongKey>) -> String {
        let filter_str = format!("filter={}", url_encode(build_filter_value(keys).as_str()));

        match query {
            "" => filter_str,
            s => format!("query={}&{}", url_encode(s), filter_str),
        }
    }

    let client = ClientType::new();
    let mut have_more = true;
    let mut page = 0;
    let mut song_meta_vec: Vec<SongMeta> = vec![];

    while have_more {
        // build the URL
        let url = format!(
            "https://{}-dsn.algolia.net/1/indexes/{}?{}&page={}",
            app_id,
            index_name,
            build_query_string(user_query, &key.compatible_keys()),
            page.to_string()
        );

        print!("FETCHING ALGOLIA URL:{}\n", url);

        // send the request
        let res = client
            .get(url)
            .header("x-algolia-api-key", api_key)
            .header("x-algolia-application-id", app_id)
            .send();

        match res {
            Err(e) => return Err(format!("while fetching algolia data: {}", e.to_string())),
            Ok(response) => match response.bytes() {
                Err(e) => return Err(format!("while reading Algolia response: {}", e.to_string())),
                Ok(bytes) => match decode_search_response(&bytes) {
                    Err(e) => {
                        return Err(format!(
                            "while decoding Algolia response: {}",
                            e.to_string()
                        ))
                    }
                    Ok(search_response) => {
                        song_meta_vec.append(&mut search_response.get_song_meta_vec());
                        have_more = search_response.has_more_pages();
                        page += 1;
                    }
                },
            },
        };
    }

    Ok(song_meta_vec)
}

// The response for the songMeta type
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SongMetaResponse {
    pub path: String,
    pub artist: String,
    pub title: String,
    pub key: SongKey,

    #[serde(rename="objectID")]
    pub object_id: String,
}

impl std::convert::From<&SongMetaResponse> for SongMeta {
    fn from(s: &SongMetaResponse) -> Self {
        SongMeta {
            path: s.path.clone(),
            artist: s.artist.clone(),
            title: s.title.clone(),
            key: s.key,
            cof_key: s.key.to_circle_of_fifths(),
        }
    }
}

// impl SongMetaResponse {
//     pub fn to_song_meta(&self) -> SongMeta {
//         SongMeta {
//             path: self.path.clone(),
//             artist: self.artist.clone(),
//             title: self.title.clone(),
//             key: self.key,
//             cof_key: self.key.to_circle_of_fifths(),
//         }
//     }
// }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    hits: Vec<SongMetaResponse>,
    page: i32,

    nb_hits: i32,
    nb_pages: i32,

    hits_per_page: i32,
}

impl SearchResponse {
    pub fn get_song_meta_vec(&self) -> Vec<SongMeta> {
        self.hits.iter().map(|r| SongMeta::from(r)).collect()
    }

    pub fn has_more_pages(&self) -> bool {
        (self.page + 1) < self.nb_pages
    }
}

///
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    // file_name: Vec<String>,
    // algolia credentials
    #[arg(long)]
    app_id: String,
    #[arg(long)]
    api_key: String,

    // index to target
    #[arg(short, long)]
    index_name: String,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    Index {
        file_names: Vec<String>,
    },
    Search {
        query: String,
        #[arg(short, long, value_enum)]
        key: SongKey,
    },
}

const RESPONSE_EXAMPLE: &str = r#"
{
    "hits": [
        {
            "path": "/Users/gyulalaszlo/Music/Reaper Projects/set_preparation/dj-2022-09-08/songs/13-shelter97-156bpm.mp3",
            "artist": "Artist A",
            "title": "Title 1",
            "key": "AMin",
            "cof_key": "8A",

            "objectID": "2051967",
            "_highlightResult": {
                "name": {
                    "value": "<em>George</em> <em>Clo</em>oney",
                    "matchLevel": "full"
                }
            },
            "_snippetResult": {
                "bio": {
                    "value": "is the son of <em>George</em> <em>Clo</em>oney as was his father"
                }
            },
            "_rankingInfo": {
                "nbTypos": 0,
                "firstMatchedWord": 0,
                "proximityDistance": 1,
                "userScore": 5,
                "geoDistance": 0,
                "geoPrecision": 1,
                "nbExactWords": 0
            }
        },
        {
            "path": "/Users/gyulalaszlo/Music/Reaper Projects/set_preparation/dj-2022-09-08/songs/13-shelter97-156bpm.mp3",
            "artist": "Artist B",
            "title": "Title 2",
            "key": "CMaj",
            "cof_key": "8B",

            "objectID": "825416",
            "_highlightResult": {
                "name": {
                    "value": "<em>George</em> <em>Clo</em>oney's Irish Roots",
                    "matchLevel": "full"
                },
                "year": {
                    "value": "(2012 Documentary)",
                    "matchLevel": "none"
                }
            },
            "_rankingInfo": {
                "nbTypos": 0,
                "firstMatchedWord": 0,
                "proximityDistance": 1,
                "userScore": 4,
                "geoDistance": 0,
                "geoPrecision": 1,
                "nbExactWords": 0
            }
        }
    ],
    "page": 0,
    "nbHits": 38,
    "nbPages": 19,
    "hitsPerPage": 2,
    "processingTimeMS": 6,
    "query": "george clo",
    "parsed_query": "george clo",
    "params": "query=george%20clo&hitsPerPage=2&getRankingInfo=1"
}
"#;

fn decode_search_response(data: &[u8]) -> Result<SearchResponse, String> {
    match serde_json::from_slice(data) {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("while decoding response: {}", e.to_string())),
    }
}

fn print_search_results(results: &Vec<SongMeta>) {
    print!("Artist\tTitle\tKey\tPath\n");
    for result in results {
        print!(
            "{}\t{}\t{}\t{}\n",
            result.artist,
            result.title,
            result.key.to_circle_of_fifths(),
            result.path
        )
    }
}

fn run_search(app_id: &str, api_key: &str, index_name: &str, key: SongKey, query_string: &str) {
    match search_algolia_for_song_by_key(app_id, api_key, index_name, key, query_string) {
        Err(e) => {
            print!("ERROR: {}", e);
        }
        Ok(results) => {
            print!("---- RESULTS ----");
            print_search_results(&results);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    print!("ARGS: {:?}\n", args);

    match decode_search_response(RESPONSE_EXAMPLE.as_bytes()) {
        Err(e) => {
            print!("ERROR: {}", e);
        }
        Ok(results) => {
            print!(
                "Decoded: {:?}\nHas More: {:?}",
                results,
                results.has_more_pages()
            );
            print_search_results(&results.get_song_meta_vec());
        }
    }
    // panic!("Stop");

    match args.command {
        Commands::Index { file_names } => {
            /*
            // Create the sender from the credentials
            let mut sender = AlgoliaSender::new(args.app_id, args.api_key, args.index_name);

            for filename in file_names {
                let song_meta = process_mp3_file(&filename);
                print!("Song meta: {:?}\n ", song_meta);

                // add the metadata to the send objects list
                sender.add_item(song_meta);
            }

            // send the data
            sender.send_items();
            */
            Ok(())
        },
        Commands::Search { query, key } => {
            run_search(&args.app_id, &args.api_key, &args.index_name, key, &query);
            Ok(())
        }
    }

}
