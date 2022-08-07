mod test_entity_attribute_macro {
    use elaiki_api::entities::{Entity, entity_base};

    #[entity_base]
    pub struct TestEntityAttributeMacro {}

    #[test]
    fn test_entity_base() {
        let mut e = TestEntityAttributeMacro { __id: 0 };
        assert_eq!(0, e.id());
        e.set_id(10);
        assert_eq!(10, e.id());
    }
}

mod test_event_attribute_macro {
    use elaiki_api::events::{Event, event_base};

    #[event_base(name = "test_event_attribute_macro")]
    pub struct TestEventAttributeMacro {}

    #[test]
    fn test_macro_entity() {
        let e = TestEventAttributeMacro {};
        assert_eq!("test_event_attribute_macro", e.event_name());
        assert_eq!(
            "test_event_attribute_macro",
            TestEventAttributeMacro::name()
        );
    }
}

mod test_event_revocable_attribute_macro {
    use elaiki_api::events::{event_base, event_revocable_base, EventRevocable};

    #[event_base(name = "test_event_revocable_attribute_macro")]
    #[event_revocable_base]
    pub struct TestEventRevocableAttributeMacro {}

    #[test]
    fn test_macro_entity() {
        let mut e = TestEventRevocableAttributeMacro { __is_revoke: false };
        assert_eq!(false, e.is_revoke());
        e.revoke();
        assert_eq!(true, e.is_revoke());
        e.set_revoke(false);
        assert_eq!(false, e.is_revoke());
    }
}
