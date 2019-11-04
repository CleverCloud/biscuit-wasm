const wasm = require("wasm-bindgen-test.js")
const assert = require("assert")

exports.create_biscuit_with_authority_fact_and_verify_should_fail_on_caveat = () => {
    let keypair = new wasm.KeyPair()
    let public_key = keypair.publicKey()

    let builder = new wasm.Biscuit()
    let fact = wasm.fact("right", [
        wasm.symbol("authority"),
        wasm.string("file1"),
        wasm.symbol("read")
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

    let biscuit = builder.build(keypair)
    console.log(biscuit.print())

    let keypair2 = new wasm.KeyPair()
    let block = biscuit.createBlock()

    let biscuit2 = biscuit.append(keypair2, block)
    console.log(biscuit2.print())

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

    verifier.addAuthorityCaveat(rule)

    verifier.verify(public_key, biscuit2)
};

exports.create_block_with_authority_fact_and_verify = () => {
    let keypair = new wasm.KeyPair()
    //let builder = wasm.BiscuitBuilder.newWithDefaultSymbols()
    //let builder = new wasm.Biscuit(["abc"])
    let builder = new wasm.Biscuit()

    builder.addAuthorityFact(wasm.fact("right", [ wasm.symbol("authority"), wasm.string("file1"), wasm.symbol("read") ] ))
    builder.addAuthorityFact(wasm.fact("right", [ wasm.symbol("authority"), wasm.string("file2"), wasm.symbol("read") ] ))
    builder.addAuthorityFact(wasm.fact("right", [ wasm.symbol("authority"), wasm.string("file1"), wasm.symbol("write") ] ))

    let biscuit1 = builder.build(keypair)


    let block2 = biscuit1.createBlock()

    let rules = wasm.rule(
        "caveat1",
        [{ variable: 0 }],
        [
            {
                name: "resource",
                ids: [{ symbol: "ambient" }, { variable: 0 }]
            },
            {
                name: "operation",
                ids: [{ symbol: "ambient" }, { symbol: "read" }]
            },
            {
                name: "right",
                ids: [{ symbol: "authority" }, { variable: 0 }, { symbol: "read" }]
            }
        ]
    )

    block2.addCaveat(rules)

    let keypair2 = new wasm.KeyPair()
    let biscuit2 = biscuit1.append(keypair2, block2)
    assert.ok(biscuit2 !== null && biscuit2 !== undefined)

    /*
    let f = wasm.constraint_test()
    console.log(f)
    console.table(f)
    */

    // test creating a rule with constraints
    let rule = wasm.constrained_rule(
      // name
      "revocation_check",
      // head ids
      [{ variable: 0 }],
      // predicates
      [{ name: "revocation_id", ids: [{ variable: 0 }] }],
      // constraints
      [{ id: 0, kind: "integer", operation: "in", data: [ 2, 1 ] }])

    let block3 = biscuit2.createBlock()
    console.table(block3)
    block3.addCaveat(rule)
    let keypair3 = new wasm.KeyPair()
    let biscuit3 = biscuit2.append(keypair3, block3)

    console.log(rule)
    console.log(biscuit3.print())
};
