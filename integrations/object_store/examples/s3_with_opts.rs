use bytes::Bytes;
use object_store::path::Path;
use object_store::{GetOptions, ObjectStore, PutOptions};
use object_store_opendal::OpendalStore;
use opendal::services::S3Config;
use opendal::Operator;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Configure S3
    let mut cfg = S3Config::default();
    cfg.access_key_id = Some("my_access_key".to_string());
    cfg.secret_access_key = Some("my_secret_key".to_string());
    cfg.endpoint = Some("my_endpoint".to_string());
    cfg.region = Some("my_region".to_string());
    cfg.bucket = "my_bucket".to_string();

    // Create a new operator
    let operator = Operator::from_config(cfg).unwrap().finish();

    // Create a new object store
    let object_store: Arc<dyn ObjectStore> = Arc::new(OpendalStore::new(operator));

    let path = Path::from("data/test.txt");
    let bytes = Bytes::from_static(b"hello, world!");

    // Put object with options
    let put_options = PutOptions::default();
    object_store
        .put_opts(&path, bytes.clone().into(), put_options)
        .await
        .unwrap();

    let meta = object_store.head(&path).await.unwrap();

    // Get object with options
    let t = meta.last_modified - Duration::from_secs(10);
    let get_options = GetOptions {
        if_modified_since: Some(t),
        ..Default::default()
    };
    let content = object_store
        .get_opts(&path, get_options)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    assert_eq!(content, bytes);
}
