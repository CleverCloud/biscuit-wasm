# Biscuit authentication and authorization token WASM API

The project support the `wasm32-unknown-unknown` compilation target. We export the biscuit API in WebAssembly in order to allow other languages to use Biscuit.
The wasm binding has been done with [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen), a facilitating high-level interactions between wasm modules and JavaScript.

## Compile for WebAssembly

We use the tool [wasm-pack](https://github.com/rustwasm/wasm-pack). It helps to build a rust-generated WebAssembly package that could be publish to the npm registry, or otherwise use alongside any javascript packages in workflows e.g.: webpack.

`wasm-pack build`

## Usage

In this example we will see how we can create a token, add some caveats, serialize and deserialize a token, append more caveats, and validate those caveats in the context of a request:

```js
// let's generate the root key pair. The root public key will be necessary
// to verify the token
let keypair = wasm.newKeypair()

// the root public key will be used for the verification phase
let public_key = keypair.publicKey()

// creating a first token
// the first block of the token is the authority block. It contains global
// information like which operation types are available
let builder = new wasm.Biscuit()

// let's define some access rights
// every fact added to the authority block must have the authority fact
let fact = wasm.fact("right", [
    { symbol: "authority" },
    { string: "file1" },
    { symbol: "read" }
])

builder.addAuthorityFact(fact)

fact = wasm.fact("right", [
    { symbol: "authority" },
    { string: "file2" },
    { symbol: "read" }
])
builder.addAuthorityFact(fact)

fact = wasm.fact("right", [
    { symbol: "authority" },
    { string: "file1" },
    { symbol: "write" }
])

builder.addAuthorityFact(fact)

// we can now create the token
let biscuit = builder.build(keypair)

let keypair2 = wasm.newKeypair()
let block = biscuit.createBlock()

block.addFact(wasm.fact("revocation_id", { integer: 1234 }))

let biscuit2 = biscuit.append(keypair2, block)

// let's define a verifier:
// for /a/file2.txt and a read operation
let verifier = new wasm.Verifier()

let rule = wasm.rule(
    "right",
    [{ symbol: "right" }],
    [
        {
            name: "right",
            ids: [{ symbol: "authority" }, { string: "file2" }, { symbol: "write" }]
        }
    ]
)

// we will check that the token has the corresponding right
verifier.addAuthorityCaveat(rule)
verifier.verify(public_key, biscuit2)
```

## Run the test

Run the tests with:
`wasm-pack test --node`

By default, the tests are generated to target Node.js, but you can configure tests to run inside headless browsers as well:

`wasm-pack test --firefox --headless`


