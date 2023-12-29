## Server

```
start listen on: 127.0.0.1:8989
client: 127.0.0.1:53005 connected
read 13 bytes
decode cmd: CommondRequest { request_data: Some(Get(Get { key: "test-key\n" })) }
client: 127.0.0.1:53022 connected
read 22 bytes
decode cmd: CommondRequest { request_data: Some(Set(Set { kv: Some(Kv { key: "test-key", value: "value\n" }) })) }
client: 127.0.0.1:53035 connected
read 14 bytes
decode cmd: CommondRequest { request_data: Some(Hget(HGet { key: "key", field: "name\n" })) }
client: 127.0.0.1:53064 connected
read 36 bytes
decode cmd: CommondRequest { request_data: Some(Hset(HSet { data: Some(HMap { key: "key", field_values: [Kv { key: "name", value: "alice" }, Kv { key: "age", value: "30\n" }] }) })) }
```

## Client

```
>>>Get test-key
done write cmd: [10, 11, 10, 9, 116, 101, 115, 116, 45, 107, 101, 121, 10]
read response: Ok("Received data")
>>>Set test-key value
done write cmd: [18, 20, 10, 18, 10, 8, 116, 101, 115, 116, 45, 107, 101, 121, 18, 6, 118, 97, 108, 117, 101, 10]
read response: Ok("Received data")
>>>HGet key name
done write cmd: [26, 12, 10, 3, 107, 101, 121, 18, 5, 110, 97, 109, 101, 10]
read response: Ok("Received data")
>>>HSet key name alice age 30
done write cmd: [34, 34, 10, 32, 10, 3, 107, 101, 121, 18, 13, 10, 4, 110, 97, 109, 101, 18, 5, 97, 108, 105, 99, 101, 18, 10, 10, 3, 97, 103, 101, 18, 3, 51, 48, 10]
read response: Ok("Received data")
```
