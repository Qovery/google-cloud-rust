use std::ops::{Deref, DerefMut};
use std::sync::Arc;

#[cfg(feature = "auth")]
pub use google_cloud_auth;
use google_cloud_gax::conn::{ConnectionOptions, Environment, Error};
use google_cloud_googleapis::cloud::scheduler::v1::cloud_scheduler_client::CloudSchedulerClient;
use google_cloud_googleapis::longrunning::operations_client::OperationsClient;

use crate::grpc::apiv1::{AUDIENCE, SCHEDULER_ENDPOINT, SCOPES};
use google_cloud_token::{NopeTokenSourceProvider, TokenSourceProvider};

use crate::grpc::apiv1::scheduler_client::Client as SchedulerGrpcClient;

#[derive(Debug)]
pub struct ClientConfig {
    pub endpoint: String,
    pub token_source_provider: Box<dyn TokenSourceProvider>,
    pub pool_size: Option<usize>,
    pub connection_option: ConnectionOptions,
}

#[cfg(feature = "auth")]
impl ClientConfig {
    pub async fn with_auth(self) -> Result<Self, google_cloud_auth::error::Error> {
        let ts = google_cloud_auth::token::DefaultTokenSourceProvider::new(Self::auth_config()).await?;
        Ok(self.with_token_source(ts).await)
    }

    pub async fn with_credentials(
        self,
        credentials: google_cloud_auth::credentials::CredentialsFile,
    ) -> Result<Self, google_cloud_auth::error::Error> {
        let ts = google_cloud_auth::token::DefaultTokenSourceProvider::new_with_credentials(
            Self::auth_config(),
            Box::new(credentials),
        )
        .await?;
        Ok(self.with_token_source(ts).await)
    }

    async fn with_token_source(mut self, ts: google_cloud_auth::token::DefaultTokenSourceProvider) -> Self {
        self.token_source_provider = Box::new(ts);
        self
    }

    fn auth_config() -> google_cloud_auth::project::Config<'static> {
        google_cloud_auth::project::Config {
            audience: None,
            scopes: Some(&SCOPES),
            sub: None,
        }
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            endpoint: SCHEDULER_ENDPOINT.to_string(),
            token_source_provider: Box::new(NopeTokenSourceProvider {}),
            pool_size: Some(1),
            connection_option: ConnectionOptions::default(),
        }
    }
}

#[derive(Clone)]
pub struct Client {
    scheduler_client: SchedulerGrpcClient,
}

impl Client {
    pub async fn new(config: ClientConfig) -> Result<Self, Error> {
        let conn_pool = google_cloud_gax::conn::ConnectionManager::new(
            1,
            config.endpoint,
            AUDIENCE,
            &Environment::GoogleCloud(config.token_source_provider),
            &config.connection_option,
        )
        .await?;
        let conn = conn_pool.conn();
        let lro_client = google_cloud_longrunning::autogen::operations_client::OperationsClient::new(conn_pool.conn())
            .await
            .unwrap();

        Ok(Self {
            scheduler_client: SchedulerGrpcClient::new(CloudSchedulerClient::new(conn), lro_client),
        })
    }
}

impl Deref for Client {
    type Target = SchedulerGrpcClient;

    fn deref(&self) -> &Self::Target {
        &self.scheduler_client
    }
}

impl DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.scheduler_client
    }
}

#[cfg(test)]
mod tests {

    use crate::client::{Client, ClientConfig};
    use serial_test::serial;

    async fn new_client() -> (Client, String) {
        let cred = google_cloud_auth::credentials::CredentialsFile::new().await.unwrap();
        let project = cred.project_id.clone().unwrap();
        let config = ClientConfig::default().with_credentials(cred).await.unwrap();
        (Client::new(config).await.unwrap(), project)
    }

    #[ctor::ctor]
    fn init() {
        let _ = tracing_subscriber::fmt().try_init();
    }
}
