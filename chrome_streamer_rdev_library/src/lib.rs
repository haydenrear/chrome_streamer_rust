use enum_fields::EnumFields;
use kafka_data_subscriber::NetworkEvent;
use rdev::{Event, EventType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum InputMonitoringEvent {
    ButtonPress { event: EventType },
    ButtonRelease { event: EventType },
    MouseMove { event: EventType },
    MouseWheel { event: EventType },
    KeyPress {event: EventType},
    KeyRelease {event: EventType},
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
    pub(crate) fn new_event(event: Event) -> Self {
        match event.event_type {
            EventType::KeyPress(k) => InputMonitoringEvent::KeyPress {event: EventType::KeyPress(k)},
            EventType::KeyRelease(k) => InputMonitoringEvent::KeyPress {event: EventType::KeyRelease(k)},
            EventType::ButtonPress(k) => InputMonitoringEvent::ButtonPress {event: EventType::ButtonPress(k)},
            EventType::ButtonRelease(k) => InputMonitoringEvent::ButtonPress {event: EventType::ButtonRelease(k)},
            EventType::MouseMove { x, y } => InputMonitoringEvent::ButtonPress {event: EventType::MouseMove{x, y}},
            EventType::Wheel { delta_x, delta_y } => InputMonitoringEvent::ButtonPress {event: EventType::Wheel{delta_x, delta_y}},
        }
    }

}

impl Default for InputMonitoringEvent {
    fn default() -> Self {
        InputMonitoringEvent::KeyPress {event: EventType::KeyPress(rdev::Key::KeyA)}
    }
}
