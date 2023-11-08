use std::error::Error;
use std::fmt::Debug;
use std::future::Future;
use std::marker::PhantomData;
use std::ops::DerefMut;
use std::pin::{pin, Pin};
use kafka_data_subscriber::data_publisher::{AggregatedKafkaErrors, DataPublisher, KafkaDataPublisher, KafkaPublishResult, KafkaSenderHandle, MessageSource, MessageSourceImpl, TokioDataPublisher};
use kafka_data_subscriber::{ConsumerSink, EventReceiver, EventSender, JoinableConsumerSink};
use kafka_data_subscriber::receiver::ReceiverHandler;
use kafka_data_subscriber::sender::SenderHandle;
use rdkafka::util::TokioRuntime;

use spring_knockoff_boot_macro::prototype;

use chrome_streamer_rdev_library::InputMonitoringEvent;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::SendError;
use knockoff_logging::*;
use rdev::{Event, ListenError};
use crate::logger_lazy;

import_logger!("rdev_listener.rs");

use spring_knockoff_boot_macro::{knockoff_ignore, service};

#[derive(Default, Debug)]
pub struct RdevMessageSource {
    event_sender: Option<EventSender<InputMonitoringEvent>>
}

#[knockoff_ignore]
impl MessageSource<InputMonitoringEvent> for RdevMessageSource {
    fn new() -> Self {
        RdevMessageSource::default()
    }

    fn set_sender(&mut self, event_sender: EventSender<InputMonitoringEvent>) {
        self.event_sender = Some(event_sender);
    }

    fn sender(&mut self) -> &mut EventSender<InputMonitoringEvent> {
        self.event_sender.as_mut().unwrap()
    }
}


#[service(RdevMessageSource)]
pub fn rdev_message_source() -> RdevMessageSource {
    let mut rdev_message: RdevMessageSource = RdevMessageSource::default();
    rdev_message
}


#[derive(Debug)]
#[knockoff_ignore]
pub struct RdevListener<
    D: TokioDataPublisher<
        InputMonitoringEvent, SenderHandlerT,
        EventReceiverHandlerT, SenderResultT,
        SenderResultErrorT
    >,
    SenderHandlerT,
    EventReceiverHandlerT,
    SenderResultT,
    SenderResultErrorT,

>
    where
        SenderHandlerT: SenderHandle<InputMonitoringEvent, SenderResultT, SenderResultErrorT, TokioRuntime> + Send + Sync + 'static,
        EventReceiverHandlerT: ReceiverHandler<InputMonitoringEvent> + Send + Sync + 'static,
        SenderResultErrorT: Error + Debug + Send + Sync,
        SenderResultT: Send + Sync + 'static
{
    pub(crate) sender_handle: PhantomData<SenderHandlerT>,
    pub(crate) event_receiver_handle: PhantomData<EventReceiverHandlerT>,
    pub(crate) data_publisher: PhantomData<D>,
    pub(crate) result: PhantomData<SenderResultT>,
    pub(crate) error: PhantomData<SenderResultErrorT>
}

#[knockoff_ignore]
impl Default for  RdevListener<
    KafkaDataPublisher<InputMonitoringEvent>,
    KafkaSenderHandle<InputMonitoringEvent, TokioRuntime>,
    EventReceiver<InputMonitoringEvent>,
    KafkaPublishResult,
    AggregatedKafkaErrors
> {
    fn default() -> Self {
        Self {
            sender_handle: Default::default(),
            event_receiver_handle: Default::default(),
            data_publisher: Default::default(),
            result: Default::default(),
            error: Default::default(),
        }
    }
}

#[knockoff_ignore]
impl<
    D: TokioDataPublisher<
        InputMonitoringEvent, SenderHandlerT,
        EventReceiverHandlerT, SenderResultT,
        SenderResultErrorT
    >,
    SenderHandlerT,
    EventReceiverHandlerT,
    SenderResultT,
    SenderResultErrorT
> RdevListener<
    D,
    SenderHandlerT,
    EventReceiverHandlerT,
    SenderResultT,
    SenderResultErrorT
>
    where
        SenderHandlerT: SenderHandle<InputMonitoringEvent, SenderResultT, SenderResultErrorT, TokioRuntime> + Send + Sync + 'static,
        EventReceiverHandlerT: ReceiverHandler<InputMonitoringEvent> + Send + Sync + 'static,
        SenderResultErrorT: Error + Debug + Send + Sync,
        SenderResultT: Send + Sync + 'static {
    pub async fn listen(
        mut sender_handler: SenderHandlerT,
        mut receiver_handler: EventReceiverHandlerT,
        mut message_source: RdevMessageSource,
    ) {
        D::publish(sender_handler, TokioRuntime {},
                   TokioRuntime {}, receiver_handler);
        let _ = rdev::listen(move |e| {
            let _ = futures::executor::block_on(async  {
                let sent = message_source.send(InputMonitoringEvent::new_event(e));
                let fut = async {
                    let sent = sent.await;
                    sent.map_err(|e| {
                        error!("Error sending InputMonitor event: {:?}", e.to_string().as_str());
                        Ok::<(), SendError<InputMonitoringEvent>>(())
                    })
                };
                let _ = fut.await;
            });
        }).map_err(|err| {
            error!("Error when listening on rdev: {:?}", &err);
            Ok::<(), ListenError>(())
        });
    }
}

#[knockoff_ignore]
impl RdevListener<
    KafkaDataPublisher<InputMonitoringEvent>,
    KafkaSenderHandle<InputMonitoringEvent, TokioRuntime>,
    EventReceiver<InputMonitoringEvent>,
    KafkaPublishResult,
    AggregatedKafkaErrors
> {
    pub async fn listen_kafka(
        mut rdev_listener_container: RdevListenerContainer
    ) {
        let event_receiver = rdev_listener_container.message_source.initialize();
        let sender_handler = KafkaSenderHandle::default_value();
        RdevListener::<
            KafkaDataPublisher<InputMonitoringEvent>,
            KafkaSenderHandle<InputMonitoringEvent, TokioRuntime>,
            EventReceiver<InputMonitoringEvent>,
            KafkaPublishResult,
            AggregatedKafkaErrors
        >::listen(sender_handler, event_receiver, rdev_listener_container.message_source).await;
    }
}

pub type RdevKafkaListener = RdevListener<
    KafkaDataPublisher<InputMonitoringEvent>,
    KafkaSenderHandle<InputMonitoringEvent, TokioRuntime>,
    EventReceiver<InputMonitoringEvent>,
    KafkaPublishResult,
    AggregatedKafkaErrors
>;

#[service(RdevListenerContainer)]
pub struct RdevListenerContainer {
    #[autowired]
    #[prototype]
    pub message_source: RdevMessageSource,
    #[autowired]
    #[prototype]
    pub event_receiver: EventReceiver<InputMonitoringEvent>
}

impl RdevListenerContainer {
    pub async fn start_container()  {
        use module_macro_lib::module_macro_lib::knockoff_context::AbstractListableFactory;

        use crate::ListableBeanFactory;
        use crate::DefaultProfile;
        use crate::PrototypeBeanContainer;

        let listable: ListableBeanFactory = AbstractListableFactory::<DefaultProfile>::new();
        let mut created_enum_one: RdevListenerContainer = PrototypeBeanContainer::<RdevListenerContainer>::fetch_bean(&listable);

        RdevListener::listen_kafka(created_enum_one).await;
    }
}

#[prototype(EventReceiver)]
pub fn rdev_listener() -> EventReceiver<InputMonitoringEvent> {
    EventReceiver::default()
}