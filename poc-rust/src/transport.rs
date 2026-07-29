use tonic::transport::Channel;

use crate::proto::{
    blind_signature_authority_client::BlindSignatureAuthorityClient,
    vote_broadcast_queue_client::VoteBroadcastQueueClient, RequestBlindSignatureRequest,
    RequestBlindSignatureResponse, SubmitVoteRequest, SubmitVoteResponse,
};

pub struct FreenetStub {
    bsa: BlindSignatureAuthorityClient<Channel>,
    queue: VoteBroadcastQueueClient<Channel>,
}

impl FreenetStub {
    pub async fn connect(endpoint: &str) -> anyhow::Result<Self> {
        Ok(Self {
            bsa: BlindSignatureAuthorityClient::connect(endpoint.to_string()).await?,
            queue: VoteBroadcastQueueClient::connect(endpoint.to_string()).await?,
        })
    }

    pub async fn request_blind_signature(
        &mut self,
        request: RequestBlindSignatureRequest,
    ) -> anyhow::Result<RequestBlindSignatureResponse> {
        tracing::info!("[Freenet stub] delivering blind signature request");
        Ok(self.bsa.request_blind_signature(request).await?.into_inner())
    }

    pub async fn submit_vote(
        &mut self,
        request: SubmitVoteRequest,
    ) -> anyhow::Result<SubmitVoteResponse> {
        tracing::info!("[Freenet stub] delivering vote submission");
        Ok(self.queue.submit_vote(request).await?.into_inner())
    }
}
