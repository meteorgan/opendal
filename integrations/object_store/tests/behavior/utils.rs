use libtest_mimic::{Failed, Trial};
use object_store_opendal::OpendalStore;
use opendal::raw::tests::TEST_RUNTIME;
use opendal::raw::MaybeSend;
use std::future::Future;

pub fn build_trail<F, Fut>(name: &str, store: &OpendalStore, f: F) -> Trial
where
    F: FnOnce(OpendalStore) -> Fut + MaybeSend + 'static,
    Fut: Future<Output = anyhow::Result<()>>,
{
    let handle = TEST_RUNTIME.handle().clone();

    let store = store.clone();

    Trial::test(format!("behavior::{name}"), move || {
        handle
            .block_on(f(store))
            .map_err(|err| Failed::from(err.to_string()))
    })
}

pub fn new_file_path(dir: &str) -> String {
    format!("{}/{}", dir, uuid::Uuid::new_v4().to_string())
}
