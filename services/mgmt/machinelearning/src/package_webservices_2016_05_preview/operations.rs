#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub mod web_services {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn get(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        web_service_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<WebService, get::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearning/webServices/{}",
            &operation_config.base_path, subscription_id, resource_group_name, web_service_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(get::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(get::BuildRequestError)?;
        let rsp = client.execute(req).await.context(get::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: WebService = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                get::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn create_or_update(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        web_service_name: &str,
        create_or_update_payload: &WebService,
        subscription_id: &str,
    ) -> std::result::Result<create_or_update::Response, create_or_update::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearning/webServices/{}",
            &operation_config.base_path, subscription_id, resource_group_name, web_service_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(create_or_update::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(create_or_update_payload);
        let req = req_builder.build().context(create_or_update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create_or_update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: WebService = serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Ok200(rsp_value))
            }
            StatusCode::CREATED => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: WebService = serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Created201(rsp_value))
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                create_or_update::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod create_or_update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(WebService),
            Created201(WebService),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn patch(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        web_service_name: &str,
        patch_payload: &WebService,
        subscription_id: &str,
    ) -> std::result::Result<WebService, patch::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearning/webServices/{}",
            &operation_config.base_path, subscription_id, resource_group_name, web_service_name
        );
        let mut req_builder = client.patch(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(patch::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(patch_payload);
        let req = req_builder.build().context(patch::BuildRequestError)?;
        let rsp = client.execute(req).await.context(patch::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(patch::ResponseBytesError)?;
                let rsp_value: WebService = serde_json::from_slice(&body).context(patch::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(patch::ResponseBytesError)?;
                patch::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod patch {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn remove(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        web_service_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<remove::Response, remove::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearning/webServices/{}",
            &operation_config.base_path, subscription_id, resource_group_name, web_service_name
        );
        let mut req_builder = client.delete(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(remove::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(remove::BuildRequestError)?;
        let rsp = client.execute(req).await.context(remove::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::ACCEPTED => Ok(remove::Response::Accepted202),
            StatusCode::NO_CONTENT => Ok(remove::Response::NoContent204),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(remove::ResponseBytesError)?;
                remove::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod remove {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list_keys(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        web_service_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<WebServiceKeys, list_keys::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearning/webServices/{}/listKeys",
            &operation_config.base_path, subscription_id, resource_group_name, web_service_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(list_keys::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list_keys::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_keys::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_keys::ResponseBytesError)?;
                let rsp_value: WebServiceKeys = serde_json::from_slice(&body).context(list_keys::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_keys::ResponseBytesError)?;
                list_keys::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list_keys {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list_by_resource_group(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        skiptoken: Option<&str>,
        subscription_id: &str,
    ) -> std::result::Result<PaginatedWebServicesList, list_by_resource_group::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.MachineLearning/webServices",
            &operation_config.base_path, subscription_id, resource_group_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(list_by_resource_group::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        if let Some(skiptoken) = skiptoken {
            req_builder = req_builder.query(&[("$skiptoken", skiptoken)]);
        }
        let req = req_builder.build().context(list_by_resource_group::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_by_resource_group::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_resource_group::ResponseBytesError)?;
                let rsp_value: PaginatedWebServicesList =
                    serde_json::from_slice(&body).context(list_by_resource_group::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_resource_group::ResponseBytesError)?;
                list_by_resource_group::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list_by_resource_group {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list(
        operation_config: &crate::OperationConfig,
        skiptoken: Option<&str>,
        subscription_id: &str,
    ) -> std::result::Result<PaginatedWebServicesList, list::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.MachineLearning/webServices",
            &operation_config.base_path, subscription_id
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(list::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        if let Some(skiptoken) = skiptoken {
            req_builder = req_builder.query(&[("$skiptoken", skiptoken)]);
        }
        let req = req_builder.build().context(list::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: PaginatedWebServicesList = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                list::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
}
