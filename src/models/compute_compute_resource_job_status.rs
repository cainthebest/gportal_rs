#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeComputeResourceJobStatus {
    #[serde(rename = "jobStartedAt", skip_serializing_if = "Option::is_none")]
    pub job_started_at: Option<String>,
    #[serde(
        rename = "jobEstimatedFinishAt",
        skip_serializing_if = "Option::is_none"
    )]
    pub job_estimated_finish_at: Option<String>,
}

impl ComputeComputeResourceJobStatus {
    pub fn new() -> ComputeComputeResourceJobStatus {
        ComputeComputeResourceJobStatus {
            job_started_at: None,
            job_estimated_finish_at: None,
        }
    }
}
