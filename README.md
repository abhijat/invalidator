# invalidator

invalidator is a bloom filter sitting behind a rest api.
You can feed lists of strings to it and query it with lists of strings to see which ones it contains 
and which ones it does not.

### Prerequisites

To build this project you need the rust toolchain installed, follow the instructions at https://rustup.rs/ 
for your operating system. 

### Building and running

Clone this repository and run cargo build (cargo is part of the rust toolchain). 
Then run the binary generated:

```
./target/debug/invalidator
```

Or simply
```
cargo run
```

### Rest API

There are two endpoints:

* /push
* /check

Both accept a json payload as POST which should contain a single field: `keys`

This field should be a list of strings.

**/push** takes the supplied keys and adds them to the internal bloom filter. 
Later queries can be run against these keys.

**/check** takes the supplied keys and checks them against the filter. The response payload contains two fields: 
 
 
 `missing_entries`: the keys which were not in the filter
 
 
 `matched_entries`: the keys which were in the filter 


###### Bloom filter design:

The bloom filter uses a bit vector ([bit-vec](http://contain-rs.github.io/bit-vec/bit_vec/)) as its backing store.

The filter is sized as a power of two because this way we can simplify the mod operation (which is very slow) to a bitwise AND.

metro hash is used for computing the initial hashes of the supplied key.
