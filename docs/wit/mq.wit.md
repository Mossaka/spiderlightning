# Types

## <a href="#error" name="error"></a> `error`: variant


Size: 12, Alignment: 4

### Variant Cases

- <a href="error.error_with_description" name="error.error_with_description"></a> [`error-with-description`](#error.error_with_description): `string`
  
  
## <a href="#payload" name="payload"></a> `payload`: list<`u8`>


Size: 8, Alignment: 4

## <a href="#observable" name="observable"></a> `observable`: record


Size: 16, Alignment: 4

### Record Fields

- <a href="observable.rd" name="observable.rd"></a> [`rd`](#observable.rd): `string`
  
  
- <a href="observable.key" name="observable.key"></a> [`key`](#observable.key): `string`
  
  
# Functions

----

#### <a href="#mq_open" name="mq_open"></a> `mq::open` 

##### Params

- <a href="#mq_open.name" name="mq_open.name"></a> `name`: `string`
##### Results

- <a href="#mq_open.result" name="mq_open.result"></a> `result`: expected<handle<mq>, [`error`](#error)>

----

#### <a href="#mq_send" name="mq_send"></a> `mq::send` 

##### Params

- <a href="#mq_send.self" name="mq_send.self"></a> `self`: handle<mq>
- <a href="#mq_send.msg" name="mq_send.msg"></a> `msg`: [`payload`](#payload)
##### Results

- <a href="#mq_send.result" name="mq_send.result"></a> `result`: expected<`unit`, [`error`](#error)>

----

#### <a href="#mq_receive" name="mq_receive"></a> `mq::receive` 

##### Params

- <a href="#mq_receive.self" name="mq_receive.self"></a> `self`: handle<mq>
##### Results

- <a href="#mq_receive.result" name="mq_receive.result"></a> `result`: expected<[`payload`](#payload), [`error`](#error)>

