const fromHex = hexString =>
  new Uint8Array(hexString.match(/.{1,2}/g).map(byte => parseInt(byte, 16)));

const toHex = bytes =>
  bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '');

const newBlock = index => {
  var li = document.createElement("li");
  li.innerHTML = "<li><div id=\"block_"+index+"\">" +
    "Block n°"+index+
    "<div id=\"block_"+index+"_serialized\" width=\"100\" height=\"300\"></div>" +

    "<input type=\"button\" id=\"block_"+index+"_build\" value=\"Build\"></input>" +
    "<div id=\"block_"+index+"_buttons\"></div>" +
    "</div></li>";
  return li;
}

(async function () {

  const biscuit = await import('biscuit-wasm');
  console.log("hello world");

  const privkeyI = document.getElementById('private_key');
  const pubkeyI = document.getElementById('public_key');
  const serializedI = document.getElementById('serialized_token');
  const genkeyB = document.getElementById('generate_keys');
  const newtokenB = document.getElementById('new_token');
  const verifyB = document.getElementById('verify');
  const workD = document.getElementById('work');
  const blocksU = document.getElementById('blocks');

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

  newtokenB.addEventListener("click", () => {
    let builder = new biscuit.Biscuit()
    let firstBlock = newBlock(0);
    blocksU.appendChild(firstBlock);
    const block0Build = document.getElementById('block_0_build');
    const tokenContent = document.getElementById('token_content');

    block0Build.addEventListener("click", () => {
      let decoded = fromHex(privkeyI.value);
      let keypair = biscuit.KeyPair.fromBytes(decoded);

      let token = builder.build(keypair);
      console.log(token);
      let serialized = token.toVec();
      console.log(serialized);
      const block0Serialized = document.getElementById('block_0_serialized');
      let b64 = btoa(String.fromCharCode(...serialized));
      block0Serialized.innerText = "Serialized ("+serialized.length+" bytes -> "+b64.length+" in base64): "+b64;
      console.log("b64 length"+b64.length);
      serializedI.value = b64;

      tokenContent.innerText = token.print();
      console.log("hello")
    });

    let decoded = fromHex(privkeyI.value);
    let keypair = biscuit.KeyPair.fromBytes(decoded);
    tokenContent.innerText = builder.print(keypair);


    console.log("done");
  });

  verifyB.addEventListener("click", () => {
    let data = new Uint8Array(atob(serializedI.value).split("").map(function(c) {
          return c.charCodeAt(0); }));
    let token = biscuit.Biscuit.from(data);

    const resourceI = document.getElementById('resource');
    const operationI = document.getElementById('operation');

    let verifier = new biscuit.Verifier()
    verifier.addResource(resourceI.value);
    verifier.addOperation(operationI.value);

    let decoded = fromHex(privkeyI.value);
    let k = biscuit.KeyPair.fromBytes(decoded);

    const resI = document.getElementById('verification_result');
    try {
      let result = verifier.verify(k.publicKey(), token);
      resI.innerText = "OK";
    } catch(error) {
      resI.innerText = error;
    }
  });

  let decoded = fromHex(privkeyI.value);
  let k = biscuit.KeyPair.fromBytes(decoded);
  console.log(k);

/*
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
*/

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


