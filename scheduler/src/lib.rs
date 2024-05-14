//! # google-cloud-scheduler
//!
//! Google Cloud Platform Key Management Service Client library.
//!
//! ## Quickstart
//!
//! ### Authentication
//! There are two ways to create a client that is authenticated against the google cloud.
//!
//! #### Automatically
//!
//! The function `with_auth()` will try and read the credentials from a file specified in the environment variable `GOOGLE_APPLICATION_CREDENTIALS`, `GOOGLE_APPLICATION_CREDENTIALS_JSON` or
//! from a metadata server.
//!
//! This is also described in [google-cloud-auth](https://github.com/yoshidan/google-cloud-rust/blob/main/foundation/auth/README.md)
//!
//! ```rust
//! use google_cloud_scheduler::client::{Client, ClientConfig};
//!
//! async fn run() {
//!     let config = ClientConfig::default().with_auth().await.unwrap();
//!     let client = Client::new(config);
//! }
//! ```
//!
//! #### Manually
//!
//! When you can't use the `gcloud` authentication but you have a different way to get your credentials (e.g a different environment variable)
//! you can parse your own version of the 'credentials-file' and use it like that:
//!
//! ```rust
//! use google_cloud_auth::credentials::CredentialsFile;
//! // or google_cloud_scheduler::client::google_cloud_auth::credentials::CredentialsFile
//! use google_cloud_scheduler::client::{Client, ClientConfig};
//!
//! async fn run(cred: CredentialsFile) {
//!    let config = ClientConfig::default().with_credentials(cred).await.unwrap();
//!    let client = Client::new(config);
//! }
//! ```
//!
//! ### Usage
pub mod client;
pub mod grpc;
