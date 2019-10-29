const wasm = require("wasm-bindgen-test.js")
const assert = require("assert")

exports.create_biscuit_with_authority_fact_and_verify_should_fail_on_caveat = () => {
    let keypair = wasm.newKeypair()

    let builder = wasm.BiscuitBuilderBind.newWithDefaultSymbols()
    let fact = wasm.fact("right", [
        { Symbol: "authority" },
        { Str: "file1" },
        { Symbol: "read" }
    ])
    builder.addAuthorityFact(fact)

    fact = wasm.fact("right", [
        { Symbol: "authority" },
        { Str: "file2" },
        { Symbol: "read" }
    ])
    builder.addAuthorityFact(fact)

    fact = wasm.fact("right", [
        { Symbol: "authority" },
        { Str: "file1" },
        { Symbol: "write" }
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
        [{ Symbol: "right" }],
        [
        {
            name: "right",
            ids: [{ Symbol: "authority" }, { Str: "file2" }, { Symbol: "write" }]
        }
        ]
    )

    verifier.addAuthorityCaveat(rule)
    verifier.verify(biscuit2)
};

exports.create_block_with_authority_fact_and_verify = () => {
    let keypair = wasm.newKeypair()

    let authorityBlock = wasm.BlockBuilderBind.newWithDefaultSymbols();
    authorityBlock.addFact(wasm.fact("right", [ { Symbol: "authority" }, { Str: "file1" }, { Str: "read" } ] ))
    authorityBlock.addFact(wasm.fact("right", [ { Symbol: "authority" }, { Str: "file2" }, { Str: "read" } ] ))
    authorityBlock.addFact(wasm.fact("right", [ { Symbol: "authority" }, { Str: "file1" }, { Str: "write" } ] ))

    let biscuit1 = new wasm.BiscuitBinder(keypair, authorityBlock.build())

    let blockBuilder = biscuit1.createBlock()

    let rules = wasm.rule(
        "caveat1",
        [{ Variable: 0 }],
        [
            {
                name: "resource",
                ids: [{ Symbol: "ambient" }, { Variable: 0 }]
            },
            {
                name: "operation",
                ids: [{ Symbol: "ambient" }, { Symbol: "read" }]
            },
            {
                name: "right",
                ids: [{ Symbol: "authority" }, { Variable: 0 }, { Symbol: "read" }]
            }
        ]
    )

    blockBuilder.addCaveat(rules)
    let block2 = blockBuilder.build()

    let keypair2 = wasm.newKeypair()
    let biscuit2 = biscuit1.append(keypair2, block2)
    assert.ok(biscuit2 !== null && biscuit2 !== undefined)
};
