use async_trait::async_trait;
use derivative::Derivative;
use openvm_circuit::{
    arch::Streams,
    system::memory::{tree::public_values::UserPublicValuesProof, CHUNK},
};
use openvm_stark_backend::{
    config::{Com, StarkGenericConfig, Val},
    proof::Proof,
};
use serde::{Deserialize, Serialize};

pub mod local;
pub mod types;

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Clone(bound = "Com<SC>: Clone"))]
#[serde(bound(
    serialize = "Com<SC>: Serialize",
    deserialize = "Com<SC>: Deserialize<'de>"
))]
pub struct ContinuationVmProof<SC: StarkGenericConfig> {
    pub per_segment: Vec<Proof<SC>>,
    pub user_public_values: UserPublicValuesProof<{ CHUNK }, Val<SC>>,
}

/// Prover for a specific exe in a specific continuation VM using a specific Stark config.
pub trait ContinuationVmProver<SC: StarkGenericConfig> {
    fn prove(&self, input: impl Into<Streams<Val<SC>>>) -> ContinuationVmProof<SC>;
}

/// Async prover for a specific exe in a specific continuation VM using a specific Stark config.
#[async_trait]
pub trait AsyncContinuationVmProver<SC: StarkGenericConfig> {
    async fn prove(
        &self,
        input: impl Into<Streams<Val<SC>>> + Send + Sync,
    ) -> ContinuationVmProof<SC>;
}

/// Prover for a specific exe in a specific single-segment VM using a specific Stark config.
pub trait SingleSegmentVmProver<SC: StarkGenericConfig> {
    fn prove(&self, input: impl Into<Streams<Val<SC>>>) -> Proof<SC>;
}

/// Async prover for a specific exe in a specific single-segment VM using a specific Stark config.
#[async_trait]
pub trait AsyncSingleSegmentVmProver<SC: StarkGenericConfig> {
    async fn prove(&self, input: impl Into<Streams<Val<SC>>> + Send + Sync) -> Proof<SC>;
}
