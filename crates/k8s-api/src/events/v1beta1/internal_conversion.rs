use super::*;

impl InternalConversion for Event {
    type Internal = crate::events::internal::Event;
}

impl InternalConversion for EventSeries {
    type Internal = crate::events::internal::EventSeries;
}

impl InternalConversion for EventList {
    type Internal = crate::events::internal::EventList;
}
