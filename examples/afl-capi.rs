extern crate mp4parse;

use mp4parse::*;

#[cfg(feature = "fuzz")]
#[macro_use]
extern crate abort_on_panic;

use std::io::Read;

fn doit() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();
    let context = mp4parse_new();
    unsafe {
        let rv = mp4parse_read(context, input.as_ptr(), input.len());
        if rv == mp4parse_error::MP4PARSE_OK {
            for track in 0..mp4parse_get_track_count(context) {
                let mut info = mp4parse_track_info {
                    track_type: mp4parse_track_type::MP4PARSE_TRACK_TYPE_VIDEO,
                    track_id: 0,
                    duration: 0,
                    media_time: 0,
                };
                let rv = mp4parse_get_track_info(context, track, &mut info);
                if rv == mp4parse_error::MP4PARSE_OK {
                    println!("track {}: id={} duration={} media_time={}",
                             track, info.track_id, info.duration, info.media_time);
                    match info.track_type {
                        mp4parse_track_type::MP4PARSE_TRACK_TYPE_VIDEO => {
                            let mut video = mp4parse_track_video_info {
                                display_width: 0,
                                display_height: 0,
                                image_width: 0,
                                image_height: 0,
                            };
                            let rv = mp4parse_get_track_video_info(context, track, &mut video);
                            if rv == mp4parse_error::MP4PARSE_OK {
                                println!("  video: display={}x{} image={}x{}",
                                         video.display_width, video.display_height,
                                         video.image_width, video.image_height);
                            }
                        }
                        mp4parse_track_type::MP4PARSE_TRACK_TYPE_AUDIO => {
                            let mut audio = mp4parse_track_audio_info {
                                channels: 0,
                                bit_depth: 0,
                                sample_rate: 0,
                            };
                            let rv = mp4parse_get_track_audio_info(context, track, &mut audio);
                            if rv == mp4parse_error::MP4PARSE_OK {
                                println!("  audio: channels={} bit_depth={} sample_rate={}",
                                         audio.channels, audio.bit_depth, audio.sample_rate);
                            }
                        }
                    }
                }
            }
        } else if rv == mp4parse_error::MP4PARSE_ERROR_ASSERT {
            panic!("wrapper thread caught panic");
        }
        mp4parse_free(context);
    }
}

#[cfg(feature = "fuzz")]
fn main() {
    abort_on_panic!({
        doit();
    });
}

#[cfg(not(feature = "fuzz"))]
fn main() {
    doit();
}
