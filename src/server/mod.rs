use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use tracing::error;
use validator::{Validate, ValidationErrors};

use crate::{header, types::*};

#[allow(unused_imports)]
use crate::{apis, models};


/// Setup API Server.
pub fn new<I, A>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::default::Default + 'static,
{
    // build our application with a route
    Router::new()
        .route("/items",
            get(items_get::<I, A>).post(items_post::<I, A>)
        )
        .route("/items/:id",
            delete(items_id_delete::<I, A>).get(items_id_get::<I, A>).put(items_id_put::<I, A>)
        )
        .with_state(api_impl)
}


#[tracing::instrument(skip_all)]
fn items_get_validation(
) -> std::result::Result<(
), ValidationErrors>
{

Ok((
))
}
/// ItemsGet - GET /items
#[tracing::instrument(skip_all)]
async fn items_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::default::Default,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    items_get_validation(
    )
  ).await.unwrap();

  let Ok((
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().items_get(
      method,
      host,
      cookies,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::default::ItemsGetResponse::Status200_AListOfItems
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn items_id_delete_validation(
  path_params: models::ItemsIdDeletePathParams,
) -> std::result::Result<(
  models::ItemsIdDeletePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// ItemsIdDelete - DELETE /items/{id}
#[tracing::instrument(skip_all)]
async fn items_id_delete<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::ItemsIdDeletePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::default::Default,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    items_id_delete_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().items_id_delete(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::default::ItemsIdDeleteResponse::Status204_TheItemHasBeenSuccessfullyDeleted
                                                => {
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::default::ItemsIdDeleteResponse::Status404_ItemNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn items_id_get_validation(
  path_params: models::ItemsIdGetPathParams,
) -> std::result::Result<(
  models::ItemsIdGetPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// ItemsIdGet - GET /items/{id}
#[tracing::instrument(skip_all)]
async fn items_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::ItemsIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::default::Default,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    items_id_get_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().items_id_get(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::default::ItemsIdGetResponse::Status200_TheItemDetails
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::default::ItemsIdGetResponse::Status404_ItemNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct ItemsIdPutBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::Item,
    }


#[tracing::instrument(skip_all)]
fn items_id_put_validation(
  path_params: models::ItemsIdPutPathParams,
        body: models::Item,
) -> std::result::Result<(
  models::ItemsIdPutPathParams,
        models::Item,
), ValidationErrors>
{
  path_params.validate()?;
              let b = ItemsIdPutBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
    body,
))
}
/// ItemsIdPut - PUT /items/{id}
#[tracing::instrument(skip_all)]
async fn items_id_put<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::ItemsIdPutPathParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::Item>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::default::Default,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    items_id_put_validation(
        path_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().items_id_put(
      method,
      host,
      cookies,
        path_params,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::default::ItemsIdPutResponse::Status200_TheItemHasBeenUpdated
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::default::ItemsIdPutResponse::Status404_ItemNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct ItemsPostBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::Item,
    }


#[tracing::instrument(skip_all)]
fn items_post_validation(
        body: models::Item,
) -> std::result::Result<(
        models::Item,
), ValidationErrors>
{
              let b = ItemsPostBodyValidator { body: &body };
              b.validate()?;

Ok((
    body,
))
}
/// ItemsPost - POST /items
#[tracing::instrument(skip_all)]
async fn items_post<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<models::Item>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::default::Default,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    items_post_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().items_post(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::default::ItemsPostResponse::Status201_TheNewItemHasBeenCreated
                                                    (body)
                                                => {
                                                  let mut response = response.status(201);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

