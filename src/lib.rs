use up_core_api_wit::up_core_api::{Umessage, Uuri, Ustatus};
use up_core_api_wit::up_l1::UTransport;

use wit_bindgen_rt::async_support::FutureReader;

struct UTransportMock;

impl UTransport for UTransportMock {
    fn register_listener(_: Uuri, _: Uuri) -> FutureReader<Ustatus> {
        todo!()
    }
    fn unregister_listener(
        _: Uuri,
        _: Uuri,
    ) -> FutureReader<Ustatus> {
        todo!()
    }
    fn send(_: Umessage) -> FutureReader<Ustatus> {
        todo!()
    }
    fn receive(_: Uuri, _: Uuri) -> FutureReader<Umessage> {
        todo!()
    }
}
