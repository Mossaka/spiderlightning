# Types

## <a href="#payload" name="payload"></a> `payload`: list<`u8`>


Size: 8, Alignment: 4

## <a href="#message" name="message"></a> `message`: record


Size: 24, Alignment: 4

### Record Fields

- <a href="message.key" name="message.key"></a> [`key`](#message.key): option<[`payload`](#payload)>
  
  
- <a href="message.value" name="message.value"></a> [`value`](#message.value): option<[`payload`](#payload)>
  
  
## <a href="#error" name="error"></a> `error`: variant


Size: 12, Alignment: 4

### Variant Cases

- <a href="error.error_with_description" name="error.error_with_description"></a> [`error-with-description`](#error.error_with_description): `string`
  
  
## <a href="#observable" name="observable"></a> `observable`: record


Size: 16, Alignment: 4

### Record Fields

- <a href="observable.rd" name="observable.rd"></a> [`rd`](#observable.rd): `string`
  
  
- <a href="observable.key" name="observable.key"></a> [`key`](#observable.key): `string`
  
  
# Functions

----

#### <a href="#pubsub_open" name="pubsub_open"></a> `pubsub::open` 

##### Results

- <a href="#pubsub_open.result" name="pubsub_open.result"></a> `result`: expected<handle<pubsub>, [`error`](#error)>

----

#### <a href="#pubsub_send_message_to_topic" name="pubsub_send_message_to_topic"></a> `pubsub::send-message-to-topic` 

##### Params

- <a href="#pubsub_send_message_to_topic.self" name="pubsub_send_message_to_topic.self"></a> `self`: handle<pubsub>
- <a href="#pubsub_send_message_to_topic.msg_key" name="pubsub_send_message_to_topic.msg_key"></a> `msg-key`: [`payload`](#payload)
- <a href="#pubsub_send_message_to_topic.msg_value" name="pubsub_send_message_to_topic.msg_value"></a> `msg-value`: [`payload`](#payload)
- <a href="#pubsub_send_message_to_topic.topic" name="pubsub_send_message_to_topic.topic"></a> `topic`: `string`
##### Results

- <a href="#pubsub_send_message_to_topic.result" name="pubsub_send_message_to_topic.result"></a> `result`: expected<`unit`, [`error`](#error)>

----

#### <a href="#pubsub_subscribe_to_topic" name="pubsub_subscribe_to_topic"></a> `pubsub::subscribe-to-topic` 

##### Params

- <a href="#pubsub_subscribe_to_topic.self" name="pubsub_subscribe_to_topic.self"></a> `self`: handle<pubsub>
- <a href="#pubsub_subscribe_to_topic.topic" name="pubsub_subscribe_to_topic.topic"></a> `topic`: list<`string`>
##### Results

- <a href="#pubsub_subscribe_to_topic.result" name="pubsub_subscribe_to_topic.result"></a> `result`: expected<`unit`, [`error`](#error)>

----

#### <a href="#pubsub_poll_for_message" name="pubsub_poll_for_message"></a> `pubsub::poll-for-message` 

##### Params

- <a href="#pubsub_poll_for_message.self" name="pubsub_poll_for_message.self"></a> `self`: handle<pubsub>
- <a href="#pubsub_poll_for_message.timeout_in_secs" name="pubsub_poll_for_message.timeout_in_secs"></a> `timeout-in-secs`: `u64`
##### Results

- <a href="#pubsub_poll_for_message.result" name="pubsub_poll_for_message.result"></a> `result`: expected<[`message`](#message), [`error`](#error)>

