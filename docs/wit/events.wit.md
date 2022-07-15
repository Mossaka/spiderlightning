# Types

## <a href="#observable" name="observable"></a> `observable`: record


Size: 16, Alignment: 4

### Record Fields

- <a href="observable.rd" name="observable.rd"></a> [`rd`](#observable.rd): `string`
  
  
- <a href="observable.key" name="observable.key"></a> [`key`](#observable.key): `string`
  
  
## <a href="#error" name="error"></a> `error`: variant


Size: 12, Alignment: 4

### Variant Cases

- <a href="error.error_with_description" name="error.error_with_description"></a> [`error-with-description`](#error.error_with_description): `string`
  
  
# Functions

----

#### <a href="#events_get" name="events_get"></a> `events::get` 

##### Results

- <a href="#events_get.result" name="events_get.result"></a> `result`: expected<handle<events>, [`error`](#error)>

----

#### <a href="#events_listen" name="events_listen"></a> `events::listen` 

##### Params

- <a href="#events_listen.self" name="events_listen.self"></a> `self`: handle<events>
- <a href="#events_listen.ob" name="events_listen.ob"></a> `ob`: [`observable`](#observable)
##### Results

- <a href="#events_listen.result" name="events_listen.result"></a> `result`: expected<handle<events>, [`error`](#error)>

----

#### <a href="#events_exec" name="events_exec"></a> `events::exec` 

##### Params

- <a href="#events_exec.self" name="events_exec.self"></a> `self`: handle<events>
- <a href="#events_exec.duration" name="events_exec.duration"></a> `duration`: `u64`
##### Results

- <a href="#events_exec.result" name="events_exec.result"></a> `result`: expected<`unit`, [`error`](#error)>

