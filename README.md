## Overview

Let's set the stage:

- our hero is an aspiring DJ

- who has a large amount of music and samples at hand

- and wants to build an indexed database that can suggests tracks to mix in

- this database should be accessible from both the command-line and the web-browser


### Musical keys


To understand how we plan to have meaningful suggestions for mixing two songs, we need to take a very-very quick look at what a "key of a song" is:


> In music theory, the key of a piece is the group of pitches, or scale, that forms the basis of a musical composition in classical, Western art, and Western pop music.

[Wikipedia: Musical Key](https://en.wikipedia.org/wiki/Key_(music))


Most western music uses the notes available on a piano -- while there are many individual black and white keys on a piano, when you look at it you can see that it's just a pattern of twelve (the octave) repeating itself.

C - D flat - D - E flat - E - F - G flat - G - A flat - A - B flat - B


These twelve notes are your full spectrum of individual colours.

Just like when painting a house, most of the time you don't want to use EVERY colour in EVERY room -- you'd rather pick a smaller subset of colours to have a palette that fits the house . This smaller subset of notes is the "scale" of the song (think "pastel earth-tones" vs. "80s Miami neon"). And just like with houses, one colour is generally dominant in the final palette -- that will be the Key of our song ("peachy pastel earth-tones" vs "bluey pastel earth-tones").



### The user interface


#### Indexer UI

The indexer will be a command-line application written in Rust that can be invoked to process Wave files either by the user or by automation -- this indexer is intended to demostrate how a high performance native indexer can be written for custom content to use with Algolia.

#### Query UI

We want to create a single centralized database using Algolia with multiple user interfaces:

- a web-based UI for browser-based access for user visibility
- a command-line interface that demonstrates accessing the search and recommendations from native applications (desktop or mobile)



## Creating the music indexer

While this looks like a fairly large undertaking on its own, thankfully we can split it up to small chunks and have libraries ready for most of the daunting tasks:

- to open and read metadata and PCM wave data from audio files we'll use the [Symphonia](https://github.com/pdeljanov/Symphonia) Rust library
- the heaviest of heavy lifting (detecting the key of a song) will be done by the [libkeyfinder](https://github.com/mixxxdj/libkeyfinder) C++ library
- we'll use the previously created Rust HTTP submitter for uploading the data to Algolia


### Setting up some basics

Before anything complicated happens lets sketch the data types for what we want to do:



We'll need to return a `SongKey` that encapsulates the key of the song:

```rust

#[derive(Serialize, Debug, Copy, Clone)]
pub enum SongKey {
    CMaj,
    DfMaj,
    // ....
    AMin,
    BfMin,
    BMin,

    Unknown,

}

```

We add `Unknown` because sometimes we won't be able to figure out the key of the song. We'll also need the wrapped song metadata (which we keep simple for now):


```rust

#[derive(Serialize, Debug)]
pub struct SongMeta {
    path: String,
    artist: String,
    title: String,
    key: SongKey,
}

```

This data is what we'll be sending to Algolia for search and recommendations. We've added the Serialize trait derive to easily serialize into JSON via the serde package (`Cargo.toml`):

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = { version = "1.0" }
```


### Reading audio files


We want a function that takes an mp3 file path and returns a SongMeta if possible:

```rust
fn process_audio_file(path: &str) -> Option<SongMeta> {
```

We'll be targeting mp3 files for this iteration of the indexer, and converting the [Getting started with Symphonia example](https://github.com/pdeljanov/Symphonia/blob/master/GETTING_STARTED.md) code to our purposes:

```rust

fn process_mp3_file(path: &str) -> SongMeta {
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
            Err(err) => {
                // A unrecoverable error occured, halt decoding.
                //
                // end of stream is one such error, but our work is generally over at this point
                return song_meta;
            }
        };

        // Consume any new metadata that has been read since the last packet.
        while !format.metadata().is_latest() {
            // Pop the old head of the metadata queue.
            format.metadata().pop();

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

                        for plane in planes.planes() {
                            // TODO: We have the block of samples here one channel, send to libKeyfinder here
                            for &sample in plane.iter() {

                                // Do something with `sample`.
                            }
                        }

                        // TODO: update the song key from the libkeyfinder instance
                        song_meta.key = SongKey::Unknown;
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
```

Most of the code is a straight copy from the Symphonia example, a number of changes have been made:

When encountering errors getting the next packet we return the current song metadata -- this is because end of stream is such an event, but generally this should signal the end of work for now

```rust
    // Get the next packet from the media format.
    let packet = match format.next_packet() {
        Ok(packet) => packet,
        // ....
        Err(err) => {
            // A unrecoverable error occured, halt decoding.
            //
            // end of stream is one such error, but our work is generally over at this point
            return song_meta;
        }
    };
```

After the packet has been successfully decoded, we need the the inidividual channels (`planes` in Symphinia parlance):

```rust
match decoder.decode(&packet) {
    Ok(_decoded) => {
        // Consume the decoded audio samples.
        use symphonia::core::audio::{AudioBufferRef, Signal};
        match _decoded {
            AudioBufferRef::F32(buf) => {
                // split to channels
                let planes = buf.planes();

                for plane in planes.planes() {
                    // TODO: We have the block of samples here one channel, send to libKeyfinder here

                    for &_sample in plane.iter() {
                        // Do something with `sample`.
                    }
                }

                // TODO: update the song key from the libkeyfinder instance
                song_meta.key = SongKey::Unknown;
            }
            _ => {
                // Repeat for the different sample formats.
                unimplemented!()
            }
        }
    }
    ///...
}
```


We also would like to extract the metadata from the mp3 file whenever we encounter metadatad frames. Symphonia documentation recommends to access the metadata from the format reader (using code like the following):

```rust
// Consume any new metadata that has been read since the last packet.
while !format.metadata().is_latest() {
    // Pop the old head of the metadata queue.
    format.metadata().pop();

    // Consume the new metadata at the head of the metadata queue.
    if let Some(rev) = format.metadata().current() {
        // Consume the new metadata at the head of the metadata queue.

        // TODO: get metadata from tags (but they don't seem to work for now)
        print!("\nTags: {:?}\n", rev.tags());
    }
}

```

At the time of writing this I was unable to successfully extract metadata from mp3 files I've created, and every other ID3v2 reader was able to read.

TODO: further investigation necessary


### Wrapping libkeyfinder

LibKeyfinder is a C++ library


We want to mimic the basic LibKeyFinder usage example:

```cpp
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
```

Since `C++ <-> Rust` interop is not really safe, we'll have to interject a little C that wraps all these C++ calls.

We need a function that can take a memory block of audio data and return a key for it. To keep the code smaller we'll make it single-channel only.

To keep the exaple shorter we'll also be using the ugly and unsafe `void*` to pass the Keyfinder data around instead of using an opaque struct.


```cpp

// This is a shared instance that contains functions & data used by all instances of
// a keyfinder object is used read-only
static KeyFinder::KeyFinder kfwrapper_shared_keyfinder;

// creates a new instance of the keyfinder state
extern "C"
void* kfwrapper__init_audio_data(uint32_t frame_rate) {
    const auto a = new KeyFinder::AudioData();
    a.setFrameRate(frame_rate);
    a.setChannels(1);
    return a;
}

// destroys and cleans up after the keyfinder.
extern "C"
void kfwrapper__destroy_audio_data(void* key_finder) {
    // TODO: check if it is an AudioData for real
    delete ((KeyFinder::AudioData*)key_finder);
}


// The main processing function: takes a bunch of samples and adds it to the keyfinder audiodata
extern "C"
void kfwrapper__add_to_samples(void* audio_data,const float* data, uint64_t data_size) {
    auto a = (KeyFinder::AudioData*)audio_data;
    a.addToSampleCount(data_size);

    // Copy your audio into the object
    for (int i = 0; i < data_size; i++) {
        a.setSample(i, data[i]);
    }
}

// After all samples are added we can use this function to get the key from LibKeyFinder
extern "C"
int32_t kfwrapper__key_of_audio(void* audio_data) {
    auto a = (KeyFinder::AudioData*)audio_data;

    KeyFinder::key_t key = kfwrapper_shared_keyfinder.keyOfAudio(a);
    return (int32_t)key;
}

```

Now that we have the C functions themselves we need to wrap them on the rust side



```rust
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

```


The KeyFinder `constants.h` gives us the list of key values so we can parse them:

```cpp
   enum key_t {
     A_MAJOR = 0,
     A_MINOR,
     B_FLAT_MAJOR,
     G_MINOR,
     // ....
     A_FLAT_MAJOR,
     A_FLAT_MINOR,
     SILENCE = 24
   };
```

With a simple converter function:

```rust
impl SongKey {
    // Converts a LibKeyFinder key_t into a SongKey
    pub fn from_key_t(i:i32) -> SongKey {
        match i {
            0 => SongKey::AMaj,
            1 => SongKey::AMin,

            2 => SongKey::BfMaj,
            3 => SongKey::BfMin,

            // ....

            22 => SongKey::AfMaj,
            23 => SongKey::AfMin,

            _ => SongKey::Unknown,
        }
    }
}

```

We can now inject the LibKeyFinder calls into our `process_mp3_file()` function:

- initialize a new audiodata at the start
- for each packet of samples: add the


First let's create the audio data -- we'll need to find the sample rate to do this, which comes from the `track` object we've gotten after opening the audio file.

To ensure that the audio data is properly disposed of on scope exit we'll use the Go-style `defer!` from the `scopeguard` crate.


```toml
[dependencies]
scopeguard = "1.1.0"
```

```rust
// after we have the `track` from the audio file  figure out the sample rate
let track = ...;

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
```

To add the individual samples we'll add to the decoding loop:

```rust
// the decoding loop starts here
let planes = buf.planes();

// check if we have audio channels
if planes.planes().len() == 0 {
    print!("No audio channles available");
    return None;
}

// use the first channel only (as we are mono)
let plane = planes.planes()[0];

// add the data from the channel to the keyfinder audio data
unsafe {
    kfwrapper__add_to_samples(
        audio_data,
        plane.as_ptr(),
        plane.len().try_into().unwrap()
    )
};

// update the song key
let int_song_key = unsafe { kfwrapper__key_of_audio(audio_data) };
let song_key = SongKey::from_key_t(int_song_key);

// update the song key in the returned meta instance
song_meta.key = song_key;
```


This completes our `process_mp3_file()` function.



### Package up the data and send it to Algolia

Now that we have the song metadata and the key, we can send it to Algolia for searching and recommendations, but before that, we want to take a further step in adding a field to our data: a ["Camelot Key"](https://www.google.com/search?q=camelot+key)-comaptible notation of the song key.

This will give us back a number-letter combination, like `8A` for `A Minor`, which makes selecting a compatible track easy:

- tracks in the same key always work (`8A`)
- tracks with the same number, but other letter always work (`8B`)
- tracks with the same letter and one lower or higher will work (`7A` and `9A`)


```rust

impl SongKey {
    pub fn to_circle_of_fifths(&self) -> String {
        String::from(match self {
            Self::AMaj => "11B",
            Self::AMin => "8A",

            Self::BfMaj => "6B",
            Self::BfMin => "3A",

            // ...

            Self::Unknown => "Unknown",
        })
    }
}
```

Next we'll need to modify the `SongMeta` struct (which is sent to Algolia) to add this key:

```rust
pub struct SongMeta {
    // ...

    // The circle-of-fifths key
    pub cof_key: String,
}

```


And update it along the per-packet update in the audio data reader function:

```rust
// we assign the song key here
song_meta.key = song_key;
// ADD the new circle-of-fifths key
song_meta.cof_key = song_key.to_circle_of_fifths();
```


Then we can send the full results to Algolia using the previously created Rust library:

```rust

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The credentials data
    const APP_ID: &str = "";
    const INDEX_NAME: &str = "";
    const API_KEY: &str = "";

    // Create the sender from the credentials
    let mut sender = AlgoliaSender::new(APP_ID, API_KEY, INDEX_NAME);

    // find the name of the file to process from the command-line arguments
    let args: Vec<String> = std::env::args().collect();
    let song_meta = process_mp3_file(args[1]);

    // add the metadata to the send objects list
    sender.add_item(song_meta);

    // send the data
    sender.send_items();

    return Ok(());
}


```


And now we can test our indexer using:

```bash
cargo run -- "Sonny Coca-Crooks.mp3"
```


## Creating a CLI client


### Indexing

To implement a simple command-line tool for indexing and searching we'll use the [clap](https://docs.rs/clap) library to parse the command line.

```toml
[dependencies]
clap = { version = "4.0.24", features = ["derive"] }
```

This library requires us to create a struct that describes our expected arguments. For our initial version we'll process some mp3 files and upload the processing results to Algolia. To do this we'll require the following arguments:

- one or more filenames for the mp3 files
- Algolia credentials (`APP_ID` and `API_KEY`)
- the Algolia index to push the data to

The initial implementation of this is fairly straightforward: we define the structure and loop through the files adding the song metadata one-by-one then sending the results

```rust
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // a list of filenames
    file_name: Vec<String>,
    // algolia credentials
    #[arg(long)]
    app_id: String,
    #[arg(long)]
    api_key: String,

    // index to target
    #[arg(short, long)]
    index_name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse the arguments
    let args = Args::parse();

    // Create the sender from the credentials
    let mut sender = AlgoliaSender::new(args.app_id, args.api_key, args.index_name);

    for filename in args.file_name {
        let song_meta = process_mp3_file(&filename);
        print!("Song meta: {:?}\n ", song_meta)

        // add the metadata to the send objects list
        sender.add_item(song_meta);
    }


    // send the data
    sender.send_items();

    return Ok(());
}

```

### Searching

TODO: implement wrapper around search API (with filtering for compatible keys)


## Creating a web UI for the same database


TODO: implement WebUI