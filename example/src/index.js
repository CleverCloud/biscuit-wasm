const fromHex = hexString =>
  new Uint8Array(hexString.match(/.{1,2}/g).map(byte => parseInt(byte, 16)));

const toHex = bytes =>
  bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '');

(async function () {

  const biscuit = await import('biscuit-wasm');
  console.log("hello world");

  const privkeyI = document.getElementById('private_key');
  const pubkeyI = document.getElementById('public_key');
  const genkeyB = document.getElementById('generate_keys');
  const newtokenB = document.getElementById('new_token');

  genkeyB.addEventListener("click", () => {
    let privkey = new Uint8Array(32);
    let pubkey = new Uint8Array(32);

    let keypair = new biscuit.KeyPair();
    let publicKey = keypair.publicKey();

    keypair.toBytes(privkey);
    publicKey.toBytes(pubkey);

    privkeyI.value = toHex(privkey);
    pubkeyI.value = toHex(pubkey);

  })
  let decoded = fromHex(privkeyI.value);
  let k = biscuit.KeyPair.fromBytes(decoded);
  console.log(k);

  let keypair = new biscuit.KeyPair()
  let public_key = keypair.publicKey()

  let builder = new biscuit.Biscuit()
  let fact = biscuit.fact("right", [
    biscuit.symbol("authority"),
    biscuit.string("file1"),
    biscuit.symbol("read")
  ])
  builder.addAuthorityFact(fact)

  fact = biscuit.fact("right", [
    { symbol: "authority" },
    { string: "file2" },
    { symbol: "read" }
  ])
  builder.addAuthorityFact(fact)

  fact = biscuit.fact("right", [
    { symbol: "authority" },
    { string: "file1" },
    { symbol: "write" }
  ])
  builder.addAuthorityFact(fact)

  let token = builder.build(keypair)
  console.log(token.print())

  let keypair2 = new biscuit.KeyPair()
  let block = token.createBlock()

  let token2 = token.append(keypair2, block)
  console.log(token2.print())

  let verifier = new biscuit.Verifier()
  let rule = biscuit.rule(
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

  verifier.verify(public_key, token2)

/*
	const input = document.getElementById('input');
	const output = document.getElementById('output');

	const calculate = () => {

		const number = parseInt(input.value);
		const result = module.factorial(number);
		output.innerText = `${result}`;
	};

	// Calculate on load

	calculate();

	// Calculate on input

	input.addEventListener('input', calculate);
*/

})();


