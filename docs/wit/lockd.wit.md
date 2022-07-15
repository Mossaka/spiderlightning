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

#### <a href="#lockd_open" name="lockd_open"></a> `lockd::open` 

##### Results

- <a href="#lockd_open.result" name="lockd_open.result"></a> `result`: expected<handle<lockd>, [`error`](#error)>

----

#### <a href="#lockd_lock" name="lockd_lock"></a> `lockd::lock` 

##### Params

- <a href="#lockd_lock.self" name="lockd_lock.self"></a> `self`: handle<lockd>
- <a href="#lockd_lock.lock_name" name="lockd_lock.lock_name"></a> `lock-name`: [`payload`](#payload)
##### Results

- <a href="#lockd_lock.result" name="lockd_lock.result"></a> `result`: expected<[`payload`](#payload), [`error`](#error)>

----

#### <a href="#lockd_lock_with_time_to_live" name="lockd_lock_with_time_to_live"></a> `lockd::lock-with-time-to-live` 

##### Params

- <a href="#lockd_lock_with_time_to_live.self" name="lockd_lock_with_time_to_live.self"></a> `self`: handle<lockd>
- <a href="#lockd_lock_with_time_to_live.lock_name" name="lockd_lock_with_time_to_live.lock_name"></a> `lock-name`: [`payload`](#payload)
- <a href="#lockd_lock_with_time_to_live.time_to_live_in_secs" name="lockd_lock_with_time_to_live.time_to_live_in_secs"></a> `time-to-live-in-secs`: `s64`
##### Results

- <a href="#lockd_lock_with_time_to_live.result" name="lockd_lock_with_time_to_live.result"></a> `result`: expected<[`payload`](#payload), [`error`](#error)>

----

#### <a href="#lockd_unlock" name="lockd_unlock"></a> `lockd::unlock` 

##### Params

- <a href="#lockd_unlock.self" name="lockd_unlock.self"></a> `self`: handle<lockd>
- <a href="#lockd_unlock.lock_key" name="lockd_unlock.lock_key"></a> `lock-key`: [`payload`](#payload)
##### Results

- <a href="#lockd_unlock.result" name="lockd_unlock.result"></a> `result`: expected<`unit`, [`error`](#error)>

