use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use enum_fields::EnumFields;
use kafka_data_subscriber::NetworkEvent;
use rdev::{Event, EventType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, EnumFields)]
pub enum InputMonitoringEvent {
    ButtonPress {
        event: EventType,
        timestamp: u128
    },
    ButtonRelease {
        event: EventType,
        timestamp: u128
    },
    MouseMove {
        event: EventType ,
        timestamp: u128
    },
    MouseWheel {
        event: EventType,
        timestamp: u128
    },
    KeyPress {
        event: EventType,
        timestamp: u128
    },
    KeyRelease {
        event: EventType,
        timestamp: u128
    },
}

type StreamId = String;

#[derive(Serialize, Deserialize, Debug)]
pub enum StreamChangeEvent {
    StreamFinished{

    }, StreamAdded{
        stream_id: StreamId
    }
}

impl NetworkEvent for InputMonitoringEvent {
    fn topic_matcher() -> &'static str {
        "input_monitoring*"
    }

    fn publish_topics() -> Vec<&'static str> {
        vec!{ "input_monitoring_events" }
    }
}

impl InputMonitoringEvent {
    pub fn new_event(event: Event) -> Self {
        let mut now = SystemTime::now();
        let epoch_milli = now.duration_since(UNIX_EPOCH).unwrap().as_millis();
        match event.event_type {
            EventType::KeyPress(k) => InputMonitoringEvent::KeyPress {
                event: EventType::KeyPress(k),
                timestamp: epoch_milli
            },
            EventType::KeyRelease(k) => InputMonitoringEvent::KeyPress {
                event: EventType::KeyRelease(k),
                timestamp: epoch_milli
            },
            EventType::ButtonPress(k) => InputMonitoringEvent::ButtonPress {
                event: EventType::ButtonPress(k),
                timestamp: epoch_milli
            },
            EventType::ButtonRelease(k) => InputMonitoringEvent::ButtonPress {
                event: EventType::ButtonRelease(k),
                timestamp: epoch_milli
            },
            EventType::MouseMove { x, y } => InputMonitoringEvent::ButtonPress
            {event: EventType::MouseMove{x, y},
                timestamp: epoch_milli
            },
            EventType::Wheel { delta_x, delta_y } => InputMonitoringEvent::ButtonPress {
                event: EventType::Wheel{delta_x, delta_y},
                timestamp: epoch_milli
            },
        }
    }

}

impl Default for InputMonitoringEvent {
    fn default() -> Self {
        InputMonitoringEvent::KeyPress {event: EventType::KeyPress(rdev::Key::KeyA), timestamp: 0 }
    }
}
