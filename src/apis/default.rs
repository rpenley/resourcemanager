use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ItemsGetResponse {
    /// A list of items.
    Status200_AListOfItems
    (Vec<models::Item>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ItemsIdDeleteResponse {
    /// The item has been successfully deleted.
    Status204_TheItemHasBeenSuccessfullyDeleted
    ,
    /// Item not found.
    Status404_ItemNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ItemsIdGetResponse {
    /// The item details.
    Status200_TheItemDetails
    (models::Item)
    ,
    /// Item not found.
    Status404_ItemNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ItemsIdPutResponse {
    /// The item has been updated.
    Status200_TheItemHasBeenUpdated
    (models::Item)
    ,
    /// Item not found.
    Status404_ItemNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ItemsPostResponse {
    /// The new item has been created.
    Status201_TheNewItemHasBeenCreated
    (models::Item)
}


/// Default
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Default {
    /// Retrieve a list of all items in the inventory..
    ///
    /// ItemsGet - GET /items
    async fn items_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
    ) -> Result<ItemsGetResponse, String>;

    /// Delete an item from the inventory..
    ///
    /// ItemsIdDelete - DELETE /items/{id}
    async fn items_id_delete(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::ItemsIdDeletePathParams,
    ) -> Result<ItemsIdDeleteResponse, String>;

    /// Retrieve a specific item by its ID..
    ///
    /// ItemsIdGet - GET /items/{id}
    async fn items_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::ItemsIdGetPathParams,
    ) -> Result<ItemsIdGetResponse, String>;

    /// Update an existing item..
    ///
    /// ItemsIdPut - PUT /items/{id}
    async fn items_id_put(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::ItemsIdPutPathParams,
            body: models::Item,
    ) -> Result<ItemsIdPutResponse, String>;

    /// Create a new item in the inventory..
    ///
    /// ItemsPost - POST /items
    async fn items_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::Item,
    ) -> Result<ItemsPostResponse, String>;
}
