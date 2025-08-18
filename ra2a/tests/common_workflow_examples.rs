#[cfg(test)]
#[cfg(feature = "agent")]
mod basic_execution {
    use indoc::indoc;
    use ra2a::agent::{A2AAgentError, AgentBuilder, AgentHandler};
    use ra2a::client::A2AClient;
    use ra2a::core::A2A;
    use ra2a::core::artifact::Artifact;
    use ra2a::core::message::{Message, SendMessageRequest, SendMessageResponsePayload};
    use ra2a::core::part::{Part, PartBase};
    use ra2a::core::task::{Task, TaskState, TaskStatus};
    use ra2a::core::util::Object;
    use std::net::SocketAddr;

    #[derive(Debug, Default)]
    struct TestHandler;

    #[async_trait::async_trait]
    impl AgentHandler for TestHandler {
        async fn handle_message(
            &self,
            mut message: Message,
            _metadata: Option<Object>,
            mut task: Task,
        ) -> Result<SendMessageResponsePayload, A2AAgentError> {
            message.message_id = "9229e770-767c-417b-a0b0-f0741243c589".to_string();
            message.task_id = Some("363422be-b0f9-4692-a24d-278670e7c7f1".to_string());
            message.context_id = Some("c295ea44-7543-4f78-b524-7a38915ad6e4".to_string());

            task.id = "363422be-b0f9-4692-a24d-278670e7c7f1".to_string();
            task.context_id = "c295ea44-7543-4f78-b524-7a38915ad6e4".to_string();
            task.artifacts.push(Artifact {
                artifact_id: "9b6934dd-37e3-4eb1-8766-962efaab63a1".to_string(),
                name: Some("joke".to_string()),
                description: None,
                parts: vec![Part {
                    part: Some(PartBase::Text(String::from(
                        "Why did the chicken cross the road? To get to the other side!",
                    ))),
                }],
                metadata: None,
                extensions: vec![],
            });
            task.history.push(message);
            task.status = Some(TaskStatus {
                state: TaskState::Completed.into(),
                message: None,
                timestamp: None,
            });
            task.metadata = Some(Object::default());

            Ok(SendMessageResponsePayload::Task(task))
        }
    }

    #[tokio::test]
    async fn should_create_task_and_respond() {
        let agent_builder = AgentBuilder::new(TestHandler)
            .with_name("test")
            .with_json_rpc_server("[::]:0".parse::<SocketAddr>().unwrap());
        #[cfg(feature = "grpc")]
        let agent_builder = agent_builder.with_grpc_server("[::]:0".parse::<SocketAddr>().unwrap());
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
            let message = indoc! {r#"
          {
            "role": "user",
            "parts": [
              {
                "kind": "text",
                "text": "tell me a joke"
              }
            ],
            "messageId": "9229e770-767c-417b-a0b0-f0741243c589"
          }
        "#};
            let message = serde_json::from_str::<Message>(message).unwrap();
            let res = client
                .send_message(SendMessageRequest {
                    message: Some(message),
                    configuration: None,
                    metadata: None,
                })
                .await
                .unwrap();

            let expected = indoc! {r#"
          {
             "id": "363422be-b0f9-4692-a24d-278670e7c7f1",
             "contextId": "c295ea44-7543-4f78-b524-7a38915ad6e4",
             "status": {
               "state": "completed"
             },
             "artifacts": [
               {
                 "artifactId": "9b6934dd-37e3-4eb1-8766-962efaab63a1",
                 "name": "joke",
                 "parts": [
                   {
                     "kind": "text",
                     "text": "Why did the chicken cross the road? To get to the other side!"
                   }
                 ]
               }
             ],
             "history": [
               {
                 "role": "user",
                 "parts": [
                   {
                     "kind": "text",
                     "text": "tell me a joke"
                   }
                 ],
                 "messageId": "9229e770-767c-417b-a0b0-f0741243c589",
                 "taskId": "363422be-b0f9-4692-a24d-278670e7c7f1",
                 "contextId": "c295ea44-7543-4f78-b524-7a38915ad6e4"
               }
             ],
             "kind": "task",
             "metadata": {}
          }
        "#};
            let expected = serde_json::from_str::<SendMessageResponsePayload>(expected).unwrap();
            assert_eq!(res.payload.unwrap(), expected);
        }
        handle.shutdown().await.unwrap()
    }
}
