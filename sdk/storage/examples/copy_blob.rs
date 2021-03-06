use azure_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use chrono::{Duration, Utc};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    let source_account = std::env::var("SOURCE_STORAGE_ACCOUNT")
        .expect("Set env variable SOURCE_STORAGE_ACCOUNT first!");
    let source_master_key = std::env::var("SOURCE_STORAGE_MASTER_KEY")
        .expect("Set env variable SOURCE_STORAGE_MASTER_KEY first!");
    let destination_account = std::env::var("DESTINATION_STORAGE_ACCOUNT")
        .expect("Set env variable DESTINATION_STORAGE_ACCOUNT first!");
    let destination_master_key = std::env::var("DESTINATION_STORAGE_MASTER_KEY")
        .expect("Set env variable DESTINATION_STORAGE_MASTER_KEY first!");

    let source_container_name = std::env::args()
        .nth(1)
        .expect("please specify source container name as first command line parameter");
    let source_blob_name = std::env::args()
        .nth(2)
        .expect("please specify source blob name as second command line parameter");

    let destination_container_name = std::env::args()
        .nth(3)
        .expect("please specify destination container name as third command line parameter");
    let destination_blob_name = std::env::args()
        .nth(4)
        .expect("please specify destination blob name as fourth command line parameter");

    // let's get a SAS key for the source
    let sas_url = {
        let client = client::with_access_key(&source_account, &source_master_key);

        let now = Utc::now();
        let later = now + Duration::hours(1);
        let sas = client
            .shared_access_signature()
            .with_resource(SasResource::Blob)
            .with_resource_type(SasResourceType::Object)
            .with_start(now)
            .with_expiry(later)
            .with_permissions(SasPermissions::Read)
            .with_protocol(SasProtocol::HttpHttps)
            .finalize();
        println!("token: '{}'", sas.token());

        client
            .generate_signed_blob_url()
            .with_container_name(&source_container_name)
            .with_blob_name(&source_blob_name)
            .with_shared_access_signature(&sas)
            .finalize()
    };
    println!("read only SAS url: '{}'", sas_url);

    // now let's kick off the copy
    let client = client::with_access_key(&destination_account, &destination_master_key);

    let response = client
        .copy_blob()
        .with_source_url(&sas_url)
        .with_container_name(&destination_container_name)
        .with_blob_name(&destination_blob_name)
        .finalize()
        .await?;

    println!("copy response == {:#?}", response);

    Ok(())
}
