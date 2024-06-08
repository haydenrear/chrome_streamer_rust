use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};

use std::sync::{Arc, Mutex};
use std::sync::mpsc::SendError;
use knockoff_helper::project_directory;
use knockoff_logging::*;
use rdev::{Event, ListenError};

use knockoff_logging::*;
use kafka_data_subscriber::NetworkEvent;
use lazy_static::lazy_static;
use video_rs::{Decoder, Encoder, Error, Frame, Location, Options, Reader, Time};
use video_rs::frame::PixelFormat;
use video_rs::encode::Settings;

import_logger_root!("test.rs", concat!(project_directory!(), "/log_out/video_filter_test.log"));

pub struct VideoFrameFilterConfigurationProperties {
    frame_window_size: usize,
    filetype: String,
}

pub struct VideoFrameFilter {
    video_frame_filter_config: VideoFrameFilterConfigurationProperties,
}

impl VideoFrameFilter {
    pub fn new(video_frame_filter_config: VideoFrameFilterConfigurationProperties) -> Self {
        Self { video_frame_filter_config }
    }

    pub fn filter_video(&self, timestamps: Vec<i64>, location: PathBuf) {
        let out_file = self.get_out_file(&location);
        Decoder::new(location.clone())
            .map_err(|e| {
                error!("Error decoding video {:?} with error {:?}", &location, &e);
            })
            .ok()
            .map(|mut decoded| {
                let (w, h) = decoded.size();
                let mut encoder = Encoder::new(
                    &out_file,
                    Settings::preset_h264_custom(w as usize, h as usize,
                                                 PixelFormat::YUV420P,
                                                 Options::from(
                                                     HashMap::from([
                                                         ("c:v".to_string(), "libvpx-vp9".to_string()),
                                                         ("b:v".to_string(), "1000000".to_string()),
                                                         ("crf".to_string(), "20".to_string())
                                                     ])),
                    ))
                    .map_err(|e| {
                        error!("Error creating encoder: {:?}.", &e);
                    })
                    .ok();

                if encoder.is_none() {
                    panic!("Could not create encoder!");
                }

                let mut encoder = encoder.unwrap();
                let mut milliseconds = timestamps
                    .windows(self.video_frame_filter_config.frame_window_size)
                    .flat_map(|w| w.into_iter().map(|&i| i))
                    .collect::<Vec<i64>>();

                milliseconds.sort();

                let mut first = milliseconds.first();
                if first.is_none() {
                    error!("No encoding milliseconds specified.");
                    return;
                }

                for item in decoded.decode_iter() {
                    if item.is_ok() {
                        if !Self::encode_frame(&location, &mut encoder, first, item.unwrap()) {
                            info!("Doing next: {:?}", &first);
                            if milliseconds.len() == 0 {
                                error!("Finished encoding.");
                                break;
                            }
                        } else {
                            milliseconds.remove(0);
                            first = milliseconds.first();
                        }
                    } else if let Err(Error::ReadExhausted) = item {
                        error!("Reader exhausted with {} milliseconds left.", milliseconds.len());
                        break;
                    }
                }
            });
    }

    fn encode_frame(location: &PathBuf,
                    mut encoder: &mut Encoder,
                    mut first: Option<&i64>,
                    item: (Time, Frame)) -> bool {
        let (ref t, f) = item;
        let time_float = item.0.as_secs_f64().clone();
        let next_millis = (time_float * 1000.0);
        /// Skip millis in between millis specified.
        if first.filter(|first_value| (**first_value as f64) <= next_millis).is_some() {
            if let Ok(_) = encoder.encode(&f, *t)
                .map_err(|e| {
                    error!("Error writing output frame for {:?}, timestamp {:?}: {:?}", &location, &item.0, &e);
                }) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn get_out_file(&self, location: &PathBuf) -> Location {
        let new_filename = self.get_filename(&location);
        let out_file = location.as_path().parent()
            .map(|out| out.join(new_filename))
            .or_else(|| {
                error!("Could not parse filename. Using old filename for {:?}", &location);
                Some(location.clone())
            })
            .unwrap();
        Location::from(out_file)
    }

    fn get_filename(&self, location: &PathBuf) -> String {
        let out_file = location.as_path().file_name().into_iter()
            .flat_map(|f| f.to_str().into_iter())
            .map(|filename| filename.replace(&format!(".{}", &self.video_frame_filter_config.filetype), ""))
            .map(|filename| format!("{}_processed.{}", &filename, &self.video_frame_filter_config.filetype).to_string())
            .next()
            .or_else(|| {
                error!("Could not parse filename {:?}", location);
                location.as_path().file_name().map(|f| f.to_str().map(|s| s.to_string())).flatten()
            })
            .unwrap();
        out_file
    }
}

#[test]
fn test_get_frames() {
    video_rs::init().unwrap();
    let web_video = knockoff_helper::get_project_path("chrome_streamer_rdev_filter")
        .join("test_resources")
        .join("input.webm");

    assert!(web_video.exists(), "{:?} did not exist", &web_video);

    let video_frame_processor = VideoFrameFilter::new(VideoFrameFilterConfigurationProperties {
        frame_window_size: 10,
        filetype: "webm".to_string(),
    });

    let mut out = vec![];

    for i in 0..100 {
        if i % 5 == 0 {
            out.push((i * 1000));
        }
    }
    video_frame_processor.filter_video(out, web_video);

    let web_video_processed = knockoff_helper::get_project_path("chrome_streamer_rdev_filter")
        .join("test_resources")
        .join("input_processed.webm");

    assert!(web_video_processed.exists(), "Processed video did not exist");

    let webm_file = File::open(&web_video_processed).unwrap();
    assert_ne!(webm_file.metadata().as_ref().unwrap().len(), 0);

    let e = std::fs::remove_file(&web_video_processed)
        .map_err(|m| {
            error!("{}", m);
        });
}