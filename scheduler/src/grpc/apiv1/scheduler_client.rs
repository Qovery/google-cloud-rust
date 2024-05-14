use google_cloud_gax::conn::Channel;
use google_cloud_longrunning::autogen::operations_client::OperationsClient;
use google_cloud_longrunning::longrunning::Operation;
use std::time::Duration;

use google_cloud_gax::create_request;
use google_cloud_gax::grpc::{Code, Status};
use google_cloud_gax::retry::{invoke, invoke_fn, RetrySetting};
use google_cloud_googleapis::cloud::scheduler::v1::cloud_scheduler_client::CloudSchedulerClient;
use google_cloud_googleapis::cloud::scheduler::v1::{
    CreateJobRequest, DeleteJobRequest, GetJobRequest, Job, ListJobsRequest, ListJobsResponse, PauseJobRequest,
    ResumeJobRequest, RunJobRequest, UpdateJobRequest,
};

fn default_setting() -> RetrySetting {
    RetrySetting {
        from_millis: 50,
        max_delay: Some(Duration::from_secs(60)),
        factor: 1u64,
        take: 20,
        codes: vec![Code::Unavailable, Code::Unknown],
    }
}

#[derive(Clone)]
pub struct Client {
    inner: CloudSchedulerClient<Channel>,
    lro_client: OperationsClient,
}

impl Client {
    pub fn new(inner: CloudSchedulerClient<Channel>, lro_client: OperationsClient) -> Self {
        Self {
            inner: inner.max_decoding_message_size(i32::MAX as usize),
            lro_client,
        }
    }

    /// Create job
    ///
    /// https://cloud.google.com/scheduler/docs/reference/rpc/google.cloud.scheduler.v1#google.cloud.scheduler.v1.CloudScheduler.CreateJob
    ///
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn create_job(&mut self, req: CreateJobRequest, retry: Option<RetrySetting>) -> Result<Job, Status> {
        invoke_fn(
            Some(retry.unwrap_or_else(default_setting)),
            |client| async {
                let request = create_request(format!("parent={}", req.parent), req.clone());
                client
                    .create_job(request)
                    .await
                    .map(|o| o.into_inner())
                    .map_err(|e| (e, client))
            },
            &mut self.inner,
        )
        .await
    }

    /// Delete job
    ///
    /// https://cloud.google.com/scheduler/docs/reference/rpc/google.cloud.scheduler.v1#google.cloud.scheduler.v1.CloudScheduler.DeleteJob
    ///
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn delete_job(&mut self, req: DeleteJobRequest, retry: Option<RetrySetting>) -> Result<(), Status> {
        invoke_fn(
            Some(retry.unwrap_or_else(default_setting)),
            |client| async {
                let request = create_request("".to_string(), req.clone());
                client
                    .delete_job(request)
                    .await
                    .map(|o| o.into_inner())
                    .map_err(|e| (e, client))
            },
            &mut self.inner,
        )
        .await
    }

    /// Get job
    ///
    /// https://cloud.google.com/scheduler/docs/reference/rpc/google.cloud.scheduler.v1#google.cloud.scheduler.v1.CloudScheduler.GetJob
    ///
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn get_job(&mut self, req: GetJobRequest, retry: Option<RetrySetting>) -> Result<Job, Status> {
        invoke_fn(
            Some(retry.unwrap_or_else(default_setting)),
            |client| async {
                let request = create_request("".to_string(), req.clone());
                client
                    .get_job(request)
                    .await
                    .map(|o| o.into_inner())
                    .map_err(|e| (e, client))
            },
            &mut self.inner,
        )
        .await
    }

    /// List jobs
    ///
    /// https://cloud.google.com/scheduler/docs/reference/rpc/google.cloud.scheduler.v1#google.cloud.scheduler.v1.CloudScheduler.ListJobs
    ///
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn list_jobs(
        &mut self,
        req: ListJobsRequest,
        retry: Option<RetrySetting>,
    ) -> Result<ListJobsResponse, Status> {
        invoke_fn(
            Some(retry.unwrap_or_else(default_setting)),
            |client| async {
                let request = create_request("".to_string(), req.clone());
                client
                    .list_jobs(request)
                    .await
                    .map(|o| o.into_inner())
                    .map_err(|e| (e, client))
            },
            &mut self.inner,
        )
        .await
    }

    /// Pause job
    ///
    /// https://cloud.google.com/scheduler/docs/reference/rpc/google.cloud.scheduler.v1#google.cloud.scheduler.v1.CloudScheduler.PauseJob
    ///
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn pause_job(&mut self, req: PauseJobRequest, retry: Option<RetrySetting>) -> Result<Job, Status> {
        invoke_fn(
            Some(retry.unwrap_or_else(default_setting)),
            |client| async {
                let request = create_request("".to_string(), req.clone());
                client
                    .pause_job(request)
                    .await
                    .map(|o| o.into_inner())
                    .map_err(|e| (e, client))
            },
            &mut self.inner,
        )
        .await
    }

    /// Resume job
    ///
    /// https://cloud.google.com/scheduler/docs/reference/rpc/google.cloud.scheduler.v1#google.cloud.scheduler.v1.CloudScheduler.ResumeJob
    ///
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn resume_job(&mut self, req: ResumeJobRequest, retry: Option<RetrySetting>) -> Result<Job, Status> {
        invoke_fn(
            Some(retry.unwrap_or_else(default_setting)),
            |client| async {
                let request = create_request("".to_string(), req.clone());
                client
                    .resume_job(request)
                    .await
                    .map(|o| o.into_inner())
                    .map_err(|e| (e, client))
            },
            &mut self.inner,
        )
        .await
    }

    /// Run job
    ///
    /// https://cloud.google.com/scheduler/docs/reference/rpc/google.cloud.scheduler.v1#google.cloud.scheduler.v1.CloudScheduler.RunJob
    ///
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn run_job(&mut self, req: RunJobRequest, retry: Option<RetrySetting>) -> Result<Job, Status> {
        invoke_fn(
            Some(retry.unwrap_or_else(default_setting)),
            |client| async {
                let request = create_request("".to_string(), req.clone());
                client
                    .run_job(request)
                    .await
                    .map(|o| o.into_inner())
                    .map_err(|e| (e, client))
            },
            &mut self.inner,
        )
        .await
    }

    /// Update job
    ///
    /// https://cloud.google.com/scheduler/docs/reference/rpc/google.cloud.scheduler.v1#google.cloud.scheduler.v1.CloudScheduler.UpdateJob
    ///
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn update_job(&mut self, req: UpdateJobRequest, retry: Option<RetrySetting>) -> Result<Job, Status> {
        invoke_fn(
            Some(retry.unwrap_or_else(default_setting)),
            |client| async {
                let request = create_request("".to_string(), req.clone());
                client
                    .update_job(request)
                    .await
                    .map(|o| o.into_inner())
                    .map_err(|e| (e, client))
            },
            &mut self.inner,
        )
        .await
    }
}
