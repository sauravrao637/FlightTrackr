# Flight Trackr

**_Saurav Rao_**

## Development

##### Crates used

- [warp](https://crates.io/crates/warp) :- web server framework
- [tokio](https://crates.io/crates/tokio) :- used for asynchronous programming
- [serde](https://crates.io/crates/serde) :- for serialization/deserialization
- [serde_json](https://crates.io/crates/serde_json) :- used for transforming rust data structures to json and vice versa
- [log](https://crates.io/crates/log) :- used for logging
- [log4rs](https://crates.io/crates/log4rs) :- multiple output for logging

##### Libraries created

- handle-errors :- used to handle error throughout the program

## Run

- Go to root folder of the project
- use command `cargo run`
- to terminate press **Ctrl+C**

## API Documentation

To use Postman API use [this](https://www.getpostman.com/collections/bc0be6931987ef93d27a) json link and import the collection.

##### POST calculateFlightPath

- This endpoint is used to calculate final start and end point of all the flights a person took
- URL = localhost:8080/calculate
- Body [raw (json)]

```
[
  [
    "SFO",
    "ATL"
  ],
  [
    "ATL",
    "GSO"
  ]
]
```

- Output

```
["SFO", "GSO"]
```

- [cURL](https://reqbin.com/c-nkeg9yma)

```
curl --location --request POST 'localhost:8080/calculate' --header 'Content-Type: application/json' --data-raw '[["SFO", "ATL"], ["ATL","GSO"]]'
```

##### GET ping

- URL = localhost:8080/ping
- Output

```
Api v0.0.1
```

- [cURL](https://reqbin.com/c-1zsan3ex)

```
curl --location --request GET 'localhost:8080/ping'
```
