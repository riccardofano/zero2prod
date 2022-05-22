use actix_session::{Session, SessionExt};
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use std::future::{ready, Ready};
use uuid::Uuid;

pub struct TypedSession(Session);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn renew(&self) {
        self.0.renew();
    }

    pub fn insert_user_id(&self, user_id: Uuid) -> Result<(), serde_json::Error> {
        self.0.insert(Self::USER_ID_KEY, user_id)
    }

    pub fn get_user_id(&self) -> Result<Option<Uuid>, serde_json::Error> {
        self.0.get(Self::USER_ID_KEY)
    }

    pub fn log_out(&self) {
        self.0.purge()
    }
}

impl FromRequest for TypedSession {
    // return the same error as `FromRequest` returns for `Session`
    type Error = <Session as FromRequest>::Error;

    // Wrap `TypedSession` into `Ready` to convert it into a `Future` that
    // resolves to the wrapped value the first time it's polled by the executor.
    type Future = Ready<Result<TypedSession, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(Ok(TypedSession(req.get_session())))
    }
}
