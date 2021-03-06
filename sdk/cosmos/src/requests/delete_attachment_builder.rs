use crate::prelude::*;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteAttachmentBuilder<'a, 'b> {
    attachment_client: &'a AttachmentClient,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> DeleteAttachmentBuilder<'a, 'b> {
    pub(crate) fn new(attachment_client: &'a AttachmentClient) -> Self {
        Self {
            attachment_client,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b> AttachmentClientRequired<'a> for DeleteAttachmentBuilder<'a, 'b> {
    fn attachment_client(&self) -> &'a AttachmentClient {
        self.attachment_client
    }
}

impl<'a, 'b> IfMatchConditionOption<'b> for DeleteAttachmentBuilder<'a, 'b> {
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b> UserAgentOption<'b> for DeleteAttachmentBuilder<'a, 'b> {
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b> ActivityIdOption<'b> for DeleteAttachmentBuilder<'a, 'b> {
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b> ConsistencyLevelOption<'b> for DeleteAttachmentBuilder<'a, 'b> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b> IfMatchConditionSupport<'b> for DeleteAttachmentBuilder<'a, 'b> {
    type O = Self;

    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        Self {
            if_match_condition: Some(if_match_condition),
            ..self
        }
    }
}

impl<'a, 'b> UserAgentSupport<'b> for DeleteAttachmentBuilder<'a, 'b> {
    type O = Self;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a, 'b> ActivityIdSupport<'b> for DeleteAttachmentBuilder<'a, 'b> {
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a, 'b> ConsistencyLevelSupport<'b> for DeleteAttachmentBuilder<'a, 'b> {
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> DeleteAttachmentBuilder<'a, 'b> {
    pub async fn execute(&self) -> Result<crate::responses::DeleteAttachmentResponse, CosmosError> {
        let mut req = self
            .attachment_client
            .prepare_request_with_attachment_name(http::Method::DELETE);

        // add trait headers
        req = IfMatchConditionOption::add_header(self, req);
        req = UserAgentOption::add_header(self, req);
        req = ActivityIdOption::add_header(self, req);
        req = ConsistencyLevelOption::add_header(self, req);

        req = crate::add_partition_keys_header(
            self.attachment_client.document_client().partition_keys(),
            req,
        );

        let req = req.body(EMPTY_BODY.as_ref())?;

        debug!("req == {:#?}", req);

        Ok(self
            .attachment_client
            .http_client()
            .execute_request_check_status(req, StatusCode::NO_CONTENT)
            .await?
            .try_into()?)
    }
}
