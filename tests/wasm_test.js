const wasm = require("wasm-bindgen-test.js")
const assert = require("assert")

exports.create_biscuit_with_authority_fact_and_verify_should_fail_on_caveat = () => {
    let keypair = wasm.newKeypair()
    let public_key = wasm.publicKey(keypair)

    let builder = wasm.BiscuitBuilderBind.newWithDefaultSymbols()
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

    let keypair2 = wasm.newKeypair()
    let blockbuilder = biscuit.createBlock()
    let block = blockbuilder.build()

    let biscuit2 = biscuit.append(keypair2, block)

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
    let keypair = wasm.newKeypair()

    let authorityBlock = wasm.BlockBuilderBind.newWithDefaultSymbols();
    authorityBlock.addFact(wasm.fact("right", [ wasm.symbol("authority"), wasm.string("file1"), wasm.symbol("read") ] ))
    authorityBlock.addFact(wasm.fact("right", [ wasm.symbol("authority"), wasm.string("file2"), wasm.symbol("read") ] ))
    authorityBlock.addFact(wasm.fact("right", [ wasm.symbol("authority"), wasm.string("file1"), wasm.symbol("write") ] ))

    let biscuit1 = new wasm.BiscuitBinder(keypair, authorityBlock.build())

    let blockBuilder = biscuit1.createBlock()

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

    blockBuilder.addCaveat(rules)
    let block2 = blockBuilder.build()

    let keypair2 = wasm.newKeypair()
    let biscuit2 = biscuit1.append(keypair2, block2)
    assert.ok(biscuit2 !== null && biscuit2 !== undefined)
};
