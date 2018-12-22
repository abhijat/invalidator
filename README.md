# invalidator

invalidator is a bloom filter sitting behind a rest api.
It stores information about the presence (and absence) of strings in its backing store. 

Using the rest API one can:

1. feed lists of strings to the filter, it will store information about their "presence" in its backing store.
2. query the filter using lists of strings, it will respond with the strings missing from its backing store.

The major advantage here is that the backing store is very small, since we do not actually store the strings in memory, 
we just store information related to their presence in a bit vector. 

A bloom filter is guaranteed to return
a correct negative (element not in set) response. If it returns a positive response, then there is a small
chance that the response may be incorrect and the element may actually be in the set.

This property makes a bloom filter a good tool to pare down large suspect data sets from invalid data. For example, it can be placed
in front of a database to remove invalid entries before hitting the database with a potentially expensive query.


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


### Bloom filter design:

The bloom filter uses a bit vector ([bit-vec](http://contain-rs.github.io/bit-vec/bit_vec/)) as its backing store.

The filter is sized as a power of two because this way we can simplify the mod operation (which is very slow) to a bitwise AND.

[metro hash](https://github.com/flier/rust-fasthash) is used for computing the first two hashes of the supplied key.
These two hashes are used to compute subsequent positions for a key.
