use logistics_account_erasure::{ErasureRequest, InfraiClient, ProofOfDelivery, ShipmentEvent};
use std::future::Future;

fn block_on<T>(future: impl Future<Output = T>) -> T {
    use std::pin::Pin;
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    fn noop(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker { RawWaker::new(std::ptr::null(), &VTABLE) }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);
    loop { if let Poll::Ready(value) = Pin::new(&mut future).poll(&mut context) { return value; } }
}

fn main() {
    let client = match InfraiClient::from_env() { Ok(value) => value, Err(error) => { eprintln!("{error}"); return; } };
    let request = ErasureRequest {
        user_id: "user-42".into(), identity_id: "identity-7".into(),
        session_ids: vec!["session-a".into()], account_key_id: "temporary-key-9".into(),
        events: vec![ShipmentEvent { shipment_id: "S-1".into(), event: "delivered".into() }],
        proofs: vec![ProofOfDelivery { shipment_id: "S-1".into(), file_id: "pod-1".into() }],
        exception: None,
    };
    match block_on(client.erase_account(request)) { Ok(result) => println!("erased: {result:?}"), Err(error) => eprintln!("{error}") }
}
