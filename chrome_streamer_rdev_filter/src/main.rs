
#![feature(async_closure)]
use std::any::{Any, TypeId};
use std::collections::{HashMap, LinkedList};
use std::future::poll_fn;
use std::ops::Deref;
use std::ptr::slice_from_raw_parts;
use std::marker::PhantomData;
use std::pin::pin;
use lazy_static::lazy_static;
use knockoff_helper::project_directory;
use kafka_data_subscriber::data_publisher::{AggregatedKafkaErrors, KafkaPublishResult, KafkaSenderHandle, MessageSource};

use kafka_data_subscriber::data_publisher::KafkaDataPublisher;
use rdkafka::util::TokioRuntime;
use kafka_data_subscriber::EventReceiver;

use std::sync::Arc;

use knockoff_logging::*;
use kafka_data_subscriber::NetworkEvent;

import_logger_root!("main.rs", concat!(project_directory!(), "/log_out/chrome_streamer_rdev_filter.log"));


include!(concat!(env!("OUT_DIR"), "/spring-knockoff.rs"));

use module_macro::module_attr;

#[module_attr]
#[cfg(springknockoff)]
pub mod chrome_streamer_rdev_filter {

}


pub fn main() {

}