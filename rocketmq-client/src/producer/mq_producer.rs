/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::any::Any;

use rocketmq_common::common::message::message_queue::MessageQueue;
use rocketmq_common::common::message::message_single::Message;

use crate::producer::message_queue_selector::MessageQueueSelector;
use crate::producer::request_callback::RequestCallback;
use crate::producer::send_callback::SendCallback;
use crate::producer::send_result::SendResult;
use crate::producer::transaction_send_result::TransactionSendResult;
use crate::Result;

#[trait_variant::make(MQProducer: Send)]
pub trait MQProducerLocal: Any {
    /// Starts the MQ producer.
    ///
    /// This method initializes and starts the MQ producer, preparing it for sending messages.
    ///
    /// # Returns
    /// A `Result` indicating success or failure.
    async fn start(&mut self) -> Result<()>;

    /// Shuts down the MQ producer.
    ///
    /// This method gracefully shuts down the MQ producer, releasing any resources held.
    async fn shutdown(&self);

    /// Fetches the list of message queues for a given topic.
    ///
    /// This method retrieves the list of message queues available for the specified topic.
    ///
    /// # Arguments
    /// * `topic` - The topic for which to fetch the message queues.
    ///
    /// # Returns
    /// A `Result` containing a vector of `MessageQueue` objects, or an error.
    async fn fetch_publish_message_queues(&self, topic: &str) -> Result<Vec<MessageQueue>>;

    /// Sends a message.
    ///
    /// This method sends the specified message to the MQ.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    ///
    /// # Returns
    /// A `Result` containing the `SendResult`, or an error.
    async fn send(&self, msg: &Message) -> Result<SendResult>;

    /// Sends a message with a timeout.
    ///
    /// This method sends the specified message to the MQ, with a specified timeout.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `timeout` - The timeout duration in milliseconds.
    ///
    /// # Returns
    /// A `Result` containing the `SendResult`, or an error.
    async fn send_with_timeout(&self, msg: &Message, timeout: u64) -> Result<SendResult>;

    /// Sends a message with a callback.
    ///
    /// This method sends the specified message to the MQ and invokes the provided callback
    /// with the result of the send operation.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `send_callback` - A callback function to be invoked with the result of the send operation.
    async fn send_with_callback(&self, msg: &Message, send_callback: impl SendCallback + Send);

    /// Sends a message with a callback and a timeout.
    ///
    /// This method sends the specified message to the MQ, with a specified timeout, and invokes
    /// the provided callback with the result of the send operation.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `send_callback` - A callback function to be invoked with the result of the send operation.
    /// * `timeout` - The timeout duration in milliseconds.
    async fn send_with_callback_timeout(
        &self,
        msg: &Message,
        send_callback: impl SendCallback + Send,
        timeout: u64,
    );

    /// Sends a message without waiting for a response.
    ///
    /// This method sends the specified message to the MQ without waiting for a response.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    ///
    /// # Returns
    /// A `Result` indicating success or failure.
    async fn send_oneway(&self, msg: &Message) -> Result<()>;

    /// Sends a message to a specific queue.
    ///
    /// This method sends the specified message to the given message queue.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `mq` - A reference to the `MessageQueue` where the message should be sent.
    ///
    /// # Returns
    /// A `Result` containing the `SendResult`, or an error.
    async fn send_to_queue(&self, msg: &Message, mq: &MessageQueue) -> Result<SendResult>;

    /// Sends a message to a specific queue with a timeout.
    ///
    /// This method sends the specified message to the given message queue, with a specified
    /// timeout.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `mq` - A reference to the `MessageQueue` where the message should be sent.
    /// * `timeout` - The timeout duration in milliseconds.
    ///
    /// # Returns
    /// A `Result` containing the `SendResult`, or an error.
    async fn send_to_queue_with_timeout(
        &self,
        msg: &Message,
        mq: &MessageQueue,
        timeout: u64,
    ) -> Result<SendResult>;

    /// Sends a message to a specific queue with a callback.
    ///
    /// This method sends the specified message to the given message queue and invokes the provided
    /// callback with the result of the send operation.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `mq` - A reference to the `MessageQueue` where the message should be sent.
    /// * `send_callback` - A callback function to be invoked with the result of the send operation.
    async fn send_to_queue_with_callback(
        &self,
        msg: &Message,
        mq: &MessageQueue,
        send_callback: impl SendCallback + Send,
    );

    /// Sends a message to a specific queue with a callback and a timeout.
    ///
    /// This method sends the specified message to the given message queue, with a specified
    /// timeout, and invokes the provided callback with the result of the send operation.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `mq` - A reference to the `MessageQueue` where the message should be sent.
    /// * `send_callback` - A callback function to be invoked with the result of the send operation.
    /// * `timeout` - The timeout duration in milliseconds.
    async fn send_to_queue_with_callback_timeout(
        &self,
        msg: &Message,
        mq: &MessageQueue,
        send_callback: impl SendCallback + Send,
        timeout: u64,
    );

    /// Sends a message to a specific queue without waiting for a response.
    ///
    /// This method sends the specified message to the given message queue without waiting for a
    /// response.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `mq` - A reference to the `MessageQueue` where the message should be sent.
    ///
    /// # Returns
    /// A `Result` indicating success or failure.
    async fn send_oneway_to_queue(&self, msg: &Message, mq: &MessageQueue) -> Result<()>;

    /// Sends a message with a selector.
    ///
    /// This method sends the specified message to the MQ using the provided message queue selector.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `selector` - A message queue selector to determine the target queue.
    /// * `arg` - An argument to be used by the selector.
    ///
    /// # Returns
    /// A `Result` containing the `SendResult`, or an error.
    async fn send_with_selector(
        &self,
        msg: &Message,
        selector: impl MessageQueueSelector,
        arg: &str,
    ) -> Result<SendResult>;

    /// Sends a message with a selector and a timeout.
    ///
    /// This method sends the specified message to the MQ using the provided message queue selector,
    /// with a specified timeout.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `selector` - A message queue selector to determine the target queue.
    /// * `arg` - An argument to be used by the selector.
    /// * `timeout` - The timeout duration in milliseconds.
    ///
    /// # Returns
    /// A `Result` containing the `SendResult`, or an error.
    async fn send_with_selector_timeout(
        &self,
        msg: &Message,
        selector: impl MessageQueueSelector,
        arg: &str,
        timeout: u64,
    ) -> Result<SendResult>;

    /// Sends a message with a selector and a callback.
    ///
    /// This method sends the specified message to the MQ using the provided message queue selector
    /// and invokes the provided callback with the result of the send operation.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `selector` - A message queue selector to determine the target queue.
    /// * `arg` - An argument to be used by the selector.
    /// * `send_callback` - A callback function to be invoked with the result of the send operation.
    async fn send_with_selector_callback(
        &self,
        msg: &Message,
        selector: impl MessageQueueSelector,
        arg: &str,
        send_callback: impl SendCallback + Send,
    );

    /// Sends a message with a selector, a callback, and a timeout.
    ///
    /// This method sends the specified message to the MQ using the provided message queue selector,
    /// with a specified timeout, and invokes the provided callback with the result of the send
    /// operation.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `selector` - A message queue selector to determine the target queue.
    /// * `arg` - An argument to be used by the selector.
    /// * `send_callback` - A callback function to be invoked with the result of the send operation.
    /// * `timeout` - The timeout duration in milliseconds.
    async fn send_with_selector_callback_timeout(
        &self,
        msg: &Message,
        selector: impl MessageQueueSelector,
        arg: &str,
        send_callback: impl SendCallback + Send,
        timeout: u64,
    );

    /// Sends a message with a selector without waiting for a response.
    ///
    /// This method sends the specified message to the MQ using the provided message queue selector
    /// without waiting for a response.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `selector` - A message queue selector to determine the target queue.
    /// * `arg` - An argument to be used by the selector.
    ///
    /// # Returns
    /// A `Result` indicating success or failure.
    async fn send_oneway_with_selector(
        &self,
        msg: &Message,
        selector: impl MessageQueueSelector,
        arg: &str,
    ) -> Result<()>;

    /// Sends a message in a transaction.
    ///
    /// This method sends the specified message to the MQ as part of a transaction.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `arg` - An argument to be used in the transaction.
    ///
    /// # Returns
    /// A `Result` containing the `TransactionSendResult`, or an error.
    async fn send_message_in_transaction(
        &self,
        msg: &Message,
        arg: &str,
    ) -> Result<TransactionSendResult>;

    /// Sends a batch of messages.
    ///
    /// This method sends the specified batch of messages to the MQ.
    ///
    /// # Arguments
    /// * `msgs` - A slice of `Message` references to be sent.
    ///
    /// # Returns
    /// A `Result` containing the `SendResult`, or an error.
    async fn send_batch(&self, msgs: &[Message]) -> Result<SendResult>;

    /// Sends a batch of messages with a timeout.
    ///
    /// This method sends the specified batch of messages to the MQ, with a specified timeout.
    ///
    /// # Arguments
    /// * `msgs` - A slice of `Message` references to be sent.
    /// * `timeout` - The timeout duration in milliseconds.
    ///
    /// # Returns
    /// A `Result` containing the `SendResult`, or an error.
    async fn send_batch_with_timeout(&self, msgs: &[Message], timeout: u64) -> Result<SendResult>;

    /// Sends a batch of messages to a specific queue.
    ///
    /// This method sends the specified batch of messages to the given message queue.
    ///
    /// # Arguments
    /// * `msgs` - A slice of `Message` references to be sent.
    /// * `mq` - A reference to the `MessageQueue` where the messages should be sent.
    ///
    /// # Returns
    /// A `Result` containing the `SendResult`, or an error.
    async fn send_batch_to_queue(&self, msgs: &[Message], mq: &MessageQueue) -> Result<SendResult>;

    /// Sends a batch of messages to a specific queue with a timeout.
    ///
    /// This method sends the specified batch of messages to the given message queue, with a
    /// specified timeout.
    ///
    /// # Arguments
    /// * `msgs` - A slice of `Message` references to be sent.
    /// * `mq` - A reference to the `MessageQueue` where the messages should be sent.
    /// * `timeout` - The timeout duration in milliseconds.
    ///
    /// # Returns
    /// A `Result` containing the `SendResult`, or an error.
    async fn send_batch_to_queue_with_timeout(
        &self,
        msgs: &[Message],
        mq: &MessageQueue,
        timeout: u64,
    ) -> Result<SendResult>;

    /// Sends a batch of messages with a callback.
    ///
    /// This method sends the specified batch of messages to the MQ and invokes the provided
    /// callback with the result of the send operation.
    ///
    /// # Arguments
    /// * `msgs` - A slice of `Message` references to be sent.
    /// * `send_callback` - A callback function to be invoked with the result of the send operation.
    async fn send_batch_with_callback(
        &self,
        msgs: &[Message],
        send_callback: impl SendCallback + Send,
    );

    /// Sends a batch of messages with a callback and a timeout.
    ///
    /// This method sends the specified batch of messages to the MQ, with a specified timeout, and
    /// invokes the provided callback with the result of the send operation.
    ///
    /// # Arguments
    /// * `msgs` - A slice of `Message` references to be sent.
    /// * `send_callback` - A callback function to be invoked with the result of the send operation.
    /// * `timeout` - The timeout duration in milliseconds.
    async fn send_batch_with_callback_timeout(
        &self,
        msgs: &[Message],
        send_callback: impl SendCallback + Send,
        timeout: u64,
    );

    /// Sends a batch of messages to a specific queue with a callback.
    ///
    /// This method sends the specified batch of messages to the given message queue and invokes the
    /// provided callback with the result of the send operation.
    ///
    /// # Arguments
    /// * `msgs` - A slice of `Message` references to be sent.
    /// * `mq` - A reference to the `MessageQueue` where the messages should be sent.
    /// * `send_callback` - A callback function to be invoked with the result of the send operation.
    async fn send_batch_to_queue_with_callback(
        &self,
        msgs: &[Message],
        mq: &MessageQueue,
        send_callback: impl SendCallback + Send,
    );

    /// Sends a batch of messages to a specific queue with a callback and a timeout.
    ///
    /// This method sends the specified batch of messages to the given message queue, with a
    /// specified timeout, and invokes the provided callback with the result of the send
    /// operation.
    ///
    /// # Arguments
    /// * `msgs` - A slice of `Message` references to be sent.
    /// * `mq` - A reference to the `MessageQueue` where the messages should be sent.
    /// * `send_callback` - A callback function to be invoked with the result of the send operation.
    /// * `timeout` - The timeout duration in milliseconds.
    async fn send_batch_to_queue_with_callback_timeout(
        &self,
        msgs: &[Message],
        mq: &MessageQueue,
        send_callback: impl SendCallback + Send,
        timeout: u64,
    );

    /// Sends a request message.
    ///
    /// This method sends the specified request message to the MQ and waits for a response.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `timeout` - The timeout duration in milliseconds.
    ///
    /// # Returns
    /// A `Result` containing the response `Message`, or an error.
    async fn request(&self, msg: &Message, timeout: u64) -> Result<Message>;

    /// Sends a request message with a callback.
    ///
    /// This method sends the specified request message to the MQ and invokes the provided callback
    /// with the response message.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `request_callback` - A callback function to be invoked with the response message.
    /// * `timeout` - The timeout duration in milliseconds.
    async fn request_with_callback(
        &self,
        msg: &Message,
        request_callback: impl RequestCallback + Send,
        timeout: u64,
    );

    /// Sends a request message with a selector.
    ///
    /// This method sends the specified request message to the MQ using the provided message queue
    /// selector and waits for a response.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `selector` - A message queue selector to determine the target queue.
    /// * `arg` - An argument to be used by the selector.
    /// * `timeout` - The timeout duration in milliseconds.
    ///
    /// # Returns
    /// A `Result` containing the response `Message`, or an error.
    async fn request_with_selector(
        &self,
        msg: &Message,
        selector: impl MessageQueueSelector,
        arg: &str,
        timeout: u64,
    ) -> Result<Message>;

    /// Sends a request message with a selector and a callback.
    ///
    /// This method sends the specified request message to the MQ using the provided message queue
    /// selector and invokes the provided callback with the response message.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `selector` - A message queue selector to determine the target queue.
    /// * `arg` - An argument to be used by the selector.
    /// * `request_callback` - A callback function to be invoked with the response message.
    /// * `timeout` - The timeout duration in milliseconds.
    async fn request_with_selector_callback(
        &self,
        msg: &Message,
        selector: impl MessageQueueSelector,
        arg: &str,
        request_callback: impl FnOnce(Result<Message>) + Send + Sync,
        timeout: u64,
    );

    /// Sends a request message to a specific queue.
    ///
    /// This method sends the specified request message to the given message queue and waits for a
    /// response.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `mq` - A reference to the `MessageQueue` where the message should be sent.
    /// * `timeout` - The timeout duration in milliseconds.
    ///
    /// # Returns
    /// A `Result` containing the response `Message`, or an error.
    async fn request_to_queue(
        &self,
        msg: &Message,
        mq: &MessageQueue,
        timeout: u64,
    ) -> Result<Message>;

    /// Sends a request message to a specific queue with a callback.
    ///
    /// This method sends the specified request message to the given message queue and invokes the
    /// provided callback with the response message.
    ///
    /// # Arguments
    /// * `msg` - A reference to the `Message` to be sent.
    /// * `mq` - A reference to the `MessageQueue` where the message should be sent.
    /// * `request_callback` - A callback function to be invoked with the response message.
    /// * `timeout` - The timeout duration in milliseconds.
    async fn request_to_queue_with_callback(
        &self,
        msg: &Message,
        mq: &MessageQueue,
        request_callback: impl FnOnce(Result<Message>) + Send + Sync,
        timeout: u64,
    );

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}
