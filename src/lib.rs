use up_core_api_wit::up_core_api::{Umessage, Uuri};
use up_core_api_wit::up_l1::UTransport;

struct UTransportMock;

impl UTransport for UTransportMock {
    fn register_listener(_: Uuri, _: Uuri) -> wit_bindgen_rt::async_support::FutureReader<String> {
        todo!()
    }
    fn unregister_listener(
        _: Uuri,
        _: Uuri,
    ) -> wit_bindgen_rt::async_support::FutureReader<String> {
        todo!()
    }
    fn send(_: Umessage) -> wit_bindgen_rt::async_support::FutureReader<String> {
        todo!()
    }
    fn receive(_: Uuri, _: Uuri) -> FutureReader<Umessage> {
        todo!()
    }
}
