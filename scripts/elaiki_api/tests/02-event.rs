use elaiki_api::log::Logger;

struct TestLogger;

impl Logger for TestLogger {
    fn log(&self, level: elaiki_api::log::Level, content: &str) {
        print!("[{:5}] {}", level, content);
    }
}

mod test_events {
    use std::any::Any;

    use elaiki_api::events::*;

    use crate::TestLogger;

    const DEFAULT_I: i32 = 7;
    const REVISED_I: i32 = 2;

    #[event_base(name = "test_event")]
    struct TestEvent {
        i: i32,
    }

    struct TestHandler;

    impl Handler for TestHandler {
        fn handle<'a>(&self, event: &'a mut dyn Event) {
            assert_eq!(TestEvent::name(), event.event_name());
            assert_eq!(true, event.as_any().is::<TestEvent>());

            let mut event_test = event.as_any_mut().downcast_mut::<TestEvent>().unwrap();

            assert_eq!(DEFAULT_I, event_test.i);
            event_test.i = REVISED_I;
        }
    }

    #[test]
    fn test_events_dispatcher() {
        let logger = std::rc::Rc::new(TestLogger {});

        let handler = Box::new(TestHandler {});
        let mut dispatcher = Dispatcher::new(logger.clone());
        dispatcher.subscribe(TestEvent::name(), "test_handler", handler);

        let mut event = TestEvent { i: DEFAULT_I };
        assert_eq!(DEFAULT_I, event.i);

        dispatcher.publish(&mut event);

        assert_eq!(REVISED_I, event.i);
    }
}
