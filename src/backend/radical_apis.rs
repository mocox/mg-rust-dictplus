use dioxus::prelude::*;
use crate::models::radical::Radical;


#[get("/api/radicals")]
pub async fn get_radicals() -> Result<Vec<Radical>, ServerFnError> {
    crate::backend::radicals::get_all_radicals().map_err(|error| {
        eprintln!("Failed to load radicals: {error}");
        ServerFnError::ServerError {
            message: "Failed to load radicals from the database".to_string(),
            code: 500,
            details: None,
        }
    })
}

#[get("/api/radicals/by-strokes/:strokes")]
pub async fn get_radicals_by_strokes(strokes: i32) -> Result<Vec<Radical>, ServerFnError> {
    crate::backend::radicals::get_radicals_by_strokes(strokes).map_err(|error| {
        eprintln!("Failed to load radicals: {error}");
        ServerFnError::ServerError {
            message: "Failed to load radicals from the database".to_string(),
            code: 500,
            details: None,
        }
    })
}

#[cfg(all(test, feature = "server"))]
mod tests {
    use std::future::Future;
    use std::task::{Context, Poll, Waker};

    #[test]
    fn endpoint_returns_database_radicals() {
        let mut future = std::pin::pin!(super::get_radicals());
        let mut context = Context::from_waker(Waker::noop());
        match future.as_mut().poll(&mut context) {
            Poll::Ready(Ok(radicals)) => {
                assert!(!radicals.is_empty());
                assert_eq!(radicals[0].traditional, "一");
            }
            result => panic!("radicals endpoint failed: {result:?}"),
        }
    }
}