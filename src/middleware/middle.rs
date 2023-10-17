use actix_web::{dev::ServiceRequest, body::MessageBody, dev::ServiceResponse, Error};
use actix_web_lab::middleware::Next;

pub async fn middle(req : ServiceRequest,next : Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let response = next.call(req).await?;
    Ok(response)
}