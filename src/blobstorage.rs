use std::env;

use rusoto_core::{HttpClient, credential::StaticProvider, Region};
use rusoto_s3::{
    S3, S3Client,
    HeadBucketRequest, 
    CreateBucketRequest, CreateBucketOutput,
    PutObjectRequest, PutObjectOutput,
    GetObjectRequest
};
use anyhow::Result;
use tokio::io::AsyncReadExt;

struct BlobStorageConfiguration {
    endpoint: String,
    access_key: String,
    access_key_secret: String,
    region: String
}

#[derive(Clone)]
pub struct Blobstorage {
    client: S3Client
}

impl Blobstorage {
    pub fn new() -> Result<Blobstorage> {
        let configuration = Self::collect_configuration()?;

        let http_client = HttpClient::new()?;
        let credentials = StaticProvider::new(configuration.access_key, configuration.access_key_secret, None, None);
        let region = Region::Custom{name: configuration.region, endpoint: configuration.endpoint};
    
        let client = S3Client::new_with(http_client, credentials, region);

        Ok(Blobstorage{client})
    }

    pub async fn bucket_exists(&self, bucket: String) -> bool { 
        let head_bucket_request = HeadBucketRequest{bucket, ..Default::default()}; 
        let request_result = self.client.head_bucket(head_bucket_request).await;
        request_result.is_err()
    }
    
    pub async fn create_bucket(&self, bucket: String) -> Result<CreateBucketOutput> {
        let create_bucket_request = CreateBucketRequest{bucket, ..Default::default()};
        Ok(self.client.create_bucket(create_bucket_request).await?)
    }

    pub async fn push(self, bucket: String, key: String, bytes: Vec<u8>) -> Result<PutObjectOutput> {
        let put_object_request = PutObjectRequest{bucket, key, body: Some(bytes.into()), ..Default::default()};
        Ok(self.client.put_object(put_object_request).await?)
    }

    pub async fn pull(self, bucket: String, key: String) -> Result<Option<Vec<u8>>> {
        let get_object_request = GetObjectRequest{bucket, key, ..Default::default()};
        let request_result = self.client.get_object(get_object_request).await?;
        let result = if let Some(bytestream) = request_result.body {
            let mut data = Vec::new();
            let mut stream  = bytestream.into_async_read();
            stream.read_to_end(&mut data).await?;
            Some(data)
        } else {
            None
        };
        Ok(result)
    }

    fn collect_configuration() -> Result<BlobStorageConfiguration> {
        let configuration = BlobStorageConfiguration {
            endpoint: env::var("BLOB_STORAGE_ENDPOINT")?,
            access_key: env::var("BLOB_STORAGE_ACCESS_KEY")?,
            access_key_secret: env::var("BLOB_STORAGE_ACCESS_KEY_SECRET")?,
            region: env::var("BLOB_STORAGE_REGION")?
        };

        Ok(configuration)
    }
}