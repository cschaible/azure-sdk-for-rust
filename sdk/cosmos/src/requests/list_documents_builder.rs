use crate::prelude::*;
use crate::responses::ListDocumentsResponse;
use crate::ResourceType;
use azure_core::prelude::*;
use futures::stream::{unfold, Stream};
use http::StatusCode;
use serde::de::DeserializeOwned;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListDocumentsBuilder<'a, 'b> {
    collection_client: &'a CollectionClient,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel>,
    continuation: Option<&'b str>,
    max_item_count: i32,
    a_im: bool,
    partition_range_id: Option<&'b str>,
}

impl<'a, 'b> ListDocumentsBuilder<'a, 'b> {
    pub(crate) fn new(collection_client: &'a CollectionClient) -> Self {
        Self {
            collection_client,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: -1,
            a_im: false,
            partition_range_id: None,
        }
    }
}

impl<'a, 'b> CollectionClientRequired<'a> for ListDocumentsBuilder<'a, 'b> {
    fn collection_client(&self) -> &'a CollectionClient {
        self.collection_client
    }
}

impl<'a, 'b> IfMatchConditionOption<'b> for ListDocumentsBuilder<'a, 'b> {
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b> UserAgentOption<'b> for ListDocumentsBuilder<'a, 'b> {
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b> ActivityIdOption<'b> for ListDocumentsBuilder<'a, 'b> {
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b> ConsistencyLevelOption<'b> for ListDocumentsBuilder<'a, 'b> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b> ContinuationOption<'b> for ListDocumentsBuilder<'a, 'b> {
    fn continuation(&self) -> Option<&'b str> {
        self.continuation
    }
}

impl<'a, 'b> MaxItemCountOption for ListDocumentsBuilder<'a, 'b> {
    fn max_item_count(&self) -> i32 {
        self.max_item_count
    }
}

impl<'a, 'b> AIMOption for ListDocumentsBuilder<'a, 'b> {
    fn a_im(&self) -> bool {
        self.a_im
    }
}

impl<'a, 'b> PartitionRangeIdOption<'b> for ListDocumentsBuilder<'a, 'b> {
    fn partition_range_id(&self) -> Option<&'b str> {
        self.partition_range_id
    }
}

impl<'a, 'b> IfMatchConditionSupport<'b> for ListDocumentsBuilder<'a, 'b> {
    type O = Self;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: Some(if_match_condition),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b> UserAgentSupport<'b> for ListDocumentsBuilder<'a, 'b> {
    type O = Self;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a, 'b> ActivityIdSupport<'b> for ListDocumentsBuilder<'a, 'b> {
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a, 'b> ConsistencyLevelSupport<'b> for ListDocumentsBuilder<'a, 'b> {
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

impl<'a, 'b> ContinuationSupport<'b> for ListDocumentsBuilder<'a, 'b> {
    type O = Self;

    fn with_continuation(self, continuation: &'b str) -> Self::O {
        Self {
            continuation: Some(continuation),
            ..self
        }
    }
}

impl<'a, 'b> MaxItemCountSupport for ListDocumentsBuilder<'a, 'b> {
    type O = Self;

    fn with_max_item_count(self, max_item_count: i32) -> Self::O {
        Self {
            max_item_count,
            ..self
        }
    }
}

impl<'a, 'b> AIMSupport for ListDocumentsBuilder<'a, 'b> {
    type O = Self;

    fn with_a_im(self, a_im: bool) -> Self::O {
        Self { a_im, ..self }
    }
}

impl<'a, 'b> PartitionRangeIdSupport<'b> for ListDocumentsBuilder<'a, 'b> {
    type O = Self;

    fn with_partition_range_id(self, partition_range_id: &'b str) -> Self::O {
        Self {
            partition_range_id: Some(partition_range_id),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> ListDocumentsBuilder<'a, 'b> {
    pub async fn execute<T>(&self) -> Result<ListDocumentsResponse<T>, CosmosError>
    where
        T: DeserializeOwned,
    {
        let req = self.collection_client.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs",
                self.collection_client.database_client().database_name(),
                self.collection_client.collection_name()
            ),
            http::Method::GET,
            ResourceType::Documents,
        );

        // add trait headers
        let req = IfMatchConditionOption::add_header(self, req);
        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);
        let req = ContinuationOption::add_header(self, req);
        let req = MaxItemCountOption::add_header(self, req);
        let req = AIMOption::add_header(self, req);
        let req = PartitionRangeIdOption::add_header(self, req);

        let req = req.body(EMPTY_BODY.as_ref())?;

        Ok(self
            .collection_client
            .http_client()
            .execute_request_check_status(req, StatusCode::OK)
            .await?
            .try_into()?)
    }

    pub fn stream<T>(
        &self,
    ) -> impl Stream<Item = Result<ListDocumentsResponse<T>, CosmosError>> + '_
    where
        T: DeserializeOwned,
    {
        #[derive(Debug, Clone, PartialEq)]
        enum States {
            Init,
            Continuation(String),
        };

        unfold(
            Some(States::Init),
            move |continuation_token: Option<States>| {
                async move {
                    debug!("continuation_token == {:?}", &continuation_token);
                    let response = match continuation_token {
                        Some(States::Init) => self.execute().await,
                        Some(States::Continuation(continuation_token)) => {
                            self.clone()
                                .with_continuation(&continuation_token)
                                .execute()
                                .await
                        }
                        None => return None,
                    };

                    // the ? operator does not work in async move (yet?)
                    // so we have to resort to this boilerplate
                    let response = match response {
                        Ok(response) => response,
                        Err(err) => return Some((Err(err), None)),
                    };

                    let continuation_token = match &response.continuation_token {
                        Some(ct) => Some(States::Continuation(ct.to_owned())),
                        None => None,
                    };

                    Some((Ok(response), continuation_token))
                }
            },
        )
    }
}
