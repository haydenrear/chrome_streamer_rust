use std::sync::mpsc::Receiver;
use kafka_data_subscriber::config::{KafkaClientProvider, MessageClientProvider};
use kafka_data_subscriber::data_subscriber::{DataSubscriber, KafkaMessageSubscriber};
use kafka_data_subscriber::EventReceiver;
use rdev::Event;
use rdkafka::util::TokioRuntime;
use chrome_streamer_rdev_library::InputMonitoringEvent;
use crate::chrome_streamer_rdev_filter::video_filter::VideoFrameFilter;

type TokioKafkaMessageSubscriber = KafkaMessageSubscriber<InputMonitoringEvent, EventReceiver<InputMonitoringEvent>, RdevMessageClientProvider, TokioRuntime>;

pub struct KafkaActionsConsumer {
    kafka_actions_event_receiver: EventReceiver<InputMonitoringEvent>,
    video_frame_filter: VideoFrameFilter
}

pub struct RdevMessageClientProvider {
    kafka_client_provider: Option<KafkaClientProvider>,
    kafka_message_subscriber: TokioKafkaMessageSubscriber
}

impl Default for RdevMessageClientProvider {
    fn default() -> Self {
        Self {
            kafka_client_provider: KafkaClientProvider::default().into(),
            kafka_message_subscriber: KafkaMessageSubscriber {
                phantom: Default::default(),
                phantom_1: Default::default(),
                phantom_2: Default::default(),
                phantom_3: Default::default(),
            }
        }
    }
}

impl MessageClientProvider<KafkaClientProvider> for RdevMessageClientProvider {
    fn create_get_client<'a>(&'a mut self) -> &'a mut Option<KafkaClientProvider> where Self: 'a {
        &mut self.kafka_client_provider
    }

    fn create_client(&mut self) -> KafkaClientProvider {
        KafkaClientProvider::default()
    }

    fn set_client(&mut self, client: KafkaClientProvider) {
        self.kafka_client_provider = Some(client);
    }
}

impl KafkaActionsConsumer {
    pub async fn start() {
        let mut message_client_provider = RdevMessageClientProvider::default();
        // let mut new_value = Self {
        //     kafka_actions_event_receiver: EventReceiver::default(),
        //     video_frame_filter: (),
        // };
        // let mut event_receiver
        //     = new_value.kafka_actions_event_receiver;
        // TokioKafkaMessageSubscriber::subscribe(
        //     &mut message_client_provider,
        //     &mut event_receiver,
        //     TokioRuntime{}
        // );
        // assert!(event_receiver.receiver.is_some());
    }

    pub async fn process_messages(
        event_receiver: Receiver<InputMonitoringEvent>,
        video_frame_filter: VideoFrameFilter
    ) {
        loop {
            if let Ok(e) = event_receiver.recv() {
                let time = *e.timestamp();
            }
        }
    }
}