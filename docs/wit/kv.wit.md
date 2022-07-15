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

#### <a href="#kv_open" name="kv_open"></a> `kv::open` 

##### Params

- <a href="#kv_open.name" name="kv_open.name"></a> `name`: `string`
##### Results

- <a href="#kv_open.result" name="kv_open.result"></a> `result`: expected<handle<kv>, [`error`](#error)>

----

#### <a href="#kv_get" name="kv_get"></a> `kv::get` 

##### Params

- <a href="#kv_get.self" name="kv_get.self"></a> `self`: handle<kv>
- <a href="#kv_get.key" name="kv_get.key"></a> `key`: `string`
##### Results

- <a href="#kv_get.result" name="kv_get.result"></a> `result`: expected<[`payload`](#payload), [`error`](#error)>

----

#### <a href="#kv_set" name="kv_set"></a> `kv::set` 

##### Params

- <a href="#kv_set.self" name="kv_set.self"></a> `self`: handle<kv>
- <a href="#kv_set.key" name="kv_set.key"></a> `key`: `string`
- <a href="#kv_set.value" name="kv_set.value"></a> `value`: [`payload`](#payload)
##### Results

- <a href="#kv_set.result" name="kv_set.result"></a> `result`: expected<`unit`, [`error`](#error)>

----

#### <a href="#kv_delete" name="kv_delete"></a> `kv::delete` 

##### Params

- <a href="#kv_delete.self" name="kv_delete.self"></a> `self`: handle<kv>
- <a href="#kv_delete.key" name="kv_delete.key"></a> `key`: `string`
##### Results

- <a href="#kv_delete.result" name="kv_delete.result"></a> `result`: expected<`unit`, [`error`](#error)>

----

#### <a href="#kv_watch" name="kv_watch"></a> `kv::watch` 

##### Params

- <a href="#kv_watch.self" name="kv_watch.self"></a> `self`: handle<kv>
- <a href="#kv_watch.key" name="kv_watch.key"></a> `key`: `string`
##### Results

- <a href="#kv_watch.result" name="kv_watch.result"></a> `result`: expected<[`observable`](#observable), [`error`](#error)>

