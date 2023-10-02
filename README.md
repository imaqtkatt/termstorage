# Termstorage

\# _Termstorage - The best worse minimal database._

Termstorage is a server that can store **terms**, it has a simple protocol and term encoding/serialization.

## Term encoding

First, let's take a look of how to encode terms.

We have the following terms:

```rs
enum Term {
  Bool(bool),
  Number(f64),
  String(String),
  Tuple(Box<Term>, Box<Term>),
}
```

The only way to compose data structures is using the recursive Tuple.

To encode terms into a sequence of bytes, we can use a **byte tag** to know which term we need to write/read.

So, we have the following tags:

```rs
const TAG_BOOL: u8   = 20;
const TAG_NUMBER: u8 = 21;
const TAG_STRING: u8 = 22;
const TAG_TUPLE: u8  = 23;
```

The byte representation of '`Number(255.0)`' is the same as:

```rs
let num255_0 = vec![21, 64, 111, 224, 0, 0, 0, 0, 0];
```

> Notice that the byte endianness is big-endian/network-endian.

We have the following byte representation of all terms:

|  term  | tag | length? |      rest      |
|--------|-----|---------|----------------|
| Bool   | 20  | No      | 1 byte         |
| Number | 21  | No      | 8 bytes        |
| String | 22  | 8 bytes | [ length ]     |
| Tuple  | 23  | No      | { term, term } |

## Protocol

As you may know, you are using the HTTP protocol to read this page on GitHub.

A protocol models data to be transferred across the network.

Termstorage's protocol has 3 **operations**:

```rs
enum Protocol {
  Fetch(/* Omit */),
  Set(/* Omit */),
  Delete(/* Omit */),
}
```

and 5 **response** types:

```rs
enum Response {
  Ok(Term),
  Processed,
  NotFound,
  Unprocessed,
  ServerError,
}
```

Same as term encoding, we use a byte tag to represent the request/response type in bytes.

```rs
// Requests
const TAG_FETCH: u8 = 10;
const TAG_SET: u8 = 11;
const TAG_DELETE: u8 = 12;

// Responses
const TAG_OK: u8 = 50;
const TAG_PROCESSED: u8 = 51;
const TAG_NOT_FOUND: u8 = 52;
const TAG_UNPROCESSED: u8 = 53;
const TAG_SERVER_ERROR: u8 = 54;
```

We have the following byte representation of the requests:

| type   | tag | name    | rest?                          |
|--------|-----|---------|--------------------------------|
| Fetch  | 10  | 8 bytes | No                             |
| Set    | 11  | 8 bytes | payload = 8 bytes, [ payload ] |
| Delete | 12  | 8 bytes | No                             |

and for responses:

| type        | tag | rest?                          |
|-------------|-----|--------------------------------|
| Ok          | 50  | payload = 8 bytes, [ payload ] |
| Processed   | 51  | No                             |
| NotFound    | 52  | No                             |
| Unprocessed | 53  | No                             |
| ServerError | 54  | No                             |
