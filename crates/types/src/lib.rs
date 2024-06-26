mod bindings;
pub use bindings::{RollupPassage, Zenith};

mod block;
pub use block::{Coder, ZenithBlock, ZenithTransaction};

mod req;
pub use req::SignRequest;

mod resp;
pub use resp::SignResponse;

/// A [`RequestSigner`] signs [`SignRequest`]s by delegating to an
/// [`alloy_signer::Signer`].
pub trait RequestSigner {
    fn sign_request(
        &self,
        request: &SignRequest,
    ) -> impl std::future::Future<Output = Result<alloy_primitives::Signature, alloy_signer::Error>> + Send;
}

impl<T> RequestSigner for T
where
    T: alloy_signer::Signer + Send + Sync,
{
    async fn sign_request(
        &self,
        request: &SignRequest,
    ) -> Result<alloy_primitives::Signature, alloy_signer::Error> {
        let hash = request.signing_hash();
        self.sign_hash(&hash).await
    }
}
