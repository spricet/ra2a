#[cfg(test)]
#[cfg(feature = "agent")]
mod task_polling {
    use async_trait::async_trait;
    use ra2a::agent::{A2AAgentError, AgentBuilder, AgentHandler};
    use ra2a::client::A2AClient;
    use ra2a::core::message::{Message, SendMessageRequest, SendMessageResponsePayload};
    use ra2a::core::task::{GetTaskRequest, Task, TaskState, TaskStatus};
    use ra2a::core::util::Object;
    use ra2a::core::{A2A, A2AError, A2AErrorCode, A2AProtocolError};

    #[derive(Debug, Default)]
    struct TestHandler;

    #[async_trait]
    impl AgentHandler for TestHandler {
        async fn handle_message(
            &self,
            message: Message,
            _metadata: Option<Object>,
            mut task: Task,
        ) -> Result<SendMessageResponsePayload, A2AAgentError> {
            task.history.push(message);
            task.status = Some(TaskStatus::default_submitted());
            Ok(SendMessageResponsePayload::Task(task))
        }
    }

    #[tokio::test]
    async fn should_have_existing_task() {
        let agent_builder = AgentBuilder::new(TestHandler)
            .with_name("test")
            .with_json_rpc_server("[::]:0".parse().unwrap());
        #[cfg(feature = "grpc")]
        let agent_builder = agent_builder.with_grpc_server("[::]:0".parse().unwrap());
        let agent = agent_builder.build().expect("failed to build agent");

        let handle = agent.start_server().await.expect("failed to start server");

        for transport in agent.supported_transports().into_iter() {
            let client = A2AClient::new(
                transport,
                format!(
                    "http://localhost:{}",
                    handle.local_addr(transport).unwrap().port()
                ),
            )
            .await
            .unwrap();
            let res = client
                .send_message(SendMessageRequest {
                    message: Some(Message::new_simple("hello there!")),
                    configuration: None,
                    metadata: None,
                })
                .await
                .unwrap();
            let task = match res.payload.unwrap() {
                SendMessageResponsePayload::Task(task) => task,
                _ => panic!("expected task"),
            };
            assert_eq!(
                task.status.as_ref().unwrap().state,
                TaskState::Submitted.into_i32()
            );

            let got_task = client
                .get_task(GetTaskRequest {
                    id: task.id.clone(),
                    history_length: None,
                    metadata: None,
                })
                .await
                .unwrap();
            assert_eq!(got_task, task);
        }

        handle.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn should_report_task_not_found() {
        let agent_builder = AgentBuilder::new(TestHandler)
            .with_name("test")
            .with_json_rpc_server("[::]:0".parse().unwrap());
        #[cfg(feature = "grpc")]
        let agent_builder = agent_builder.with_grpc_server("[::]:0".parse().unwrap());
        let agent = agent_builder.build().expect("failed to build agent");

        let handle = agent.start_server().await.expect("failed to start server");

        for transport in agent.supported_transports().into_iter() {
            let client = A2AClient::new(
                transport,
                format!(
                    "http://localhost:{}",
                    handle.local_addr(transport).unwrap().port()
                ),
            )
            .await
            .unwrap();

            let res = client
                .get_task(GetTaskRequest {
                    id: "bogus".to_string(),
                    history_length: None,
                    metadata: None,
                })
                .await;
            let err = res.unwrap_err();
            if let A2AError::Protocol(A2AProtocolError::TaskNotFound { id, code }) = err {
                assert_eq!(id, "bogus");
                assert_eq!(code, A2AErrorCode::TaskNotFound);
            } else {
                panic!("expected task not found error, got {err:?}");
            }
        }

        handle.shutdown().await.unwrap();
    }
}
