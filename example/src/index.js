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
  const basictokenB = document.getElementById('basic_rights');
  const alltokenB = document.getElementById('all_rights');
  const verifyB = document.getElementById('verify');
  const workD = document.getElementById('work');
  const blocksU = document.getElementById('blocks');
	const tokenContent = document.getElementById('token_content');

  const loadKeys = () => {
    let decoded = fromHex(privkeyI.value);
    return biscuit.KeyPair.fromBytes(decoded);
  }

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

  const printToken = token => {
    let serialized = token.toVec();
    let b64 = btoa(String.fromCharCode(...serialized));
    serializedI.value = b64;

    tokenContent.innerText = token.print();
    const sizeS = document.getElementById('token_size');
    sizeS.innerText = "("+b64.length+" bytes in base64)";
  };

  /*
  newtokenB.addEventListener("click", () => {
    let builder = new biscuit.Biscuit()
    let firstBlock = newBlock(0);
    blocksU.appendChild(firstBlock);
    const block0Build = document.getElementById('block_0_build');

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
      printToken(token);
    });


    let decoded = fromHex(privkeyI.value);
    let keypair = biscuit.KeyPair.fromBytes(decoded);
    tokenContent.innerText = builder.print(keypair);

    console.log("done");
  });
  */

  verifyB.addEventListener("click", () => {
    let data = new Uint8Array(atob(serializedI.value).split("").map(function(c) {
          return c.charCodeAt(0); }));
    let token = biscuit.Biscuit.from(data);

    const resourceI = document.getElementById('resource');
    const operationI = document.getElementById('operation');

    let verifier = new biscuit.Verifier()
    verifier.addResource(resourceI.value);
    verifier.addOperation(operationI.value);
    let rule = biscuit.rule(
      "check_right",
      [
        { variable: 0 },
        { variable: 1 }
      ],
      [
        {
          name: "resource",
          ids: [{ symbol: "ambient" }, { variable: 0 }]
        },
        {
          name: "operation",
          ids: [{ symbol: "ambient" }, { variable: 1 }]
        },
        {
          name: "right",
          ids: [{ symbol: "authority" }, { variable: 0 }, { variable: 1 }]
        }
      ]
    );

    verifier.addAuthorityCaveat(rule)
    //verifier.addBlockCaveat(rule)

    let decoded = fromHex(privkeyI.value);
    let k = biscuit.KeyPair.fromBytes(decoded);

    const resI = document.getElementById('verification_result');
    try {
      let result = verifier.verify(k.publicKey(), token);
      resI.innerText = "OK";
    } catch(error) {
      resI.innerText = JSON.stringify(error);
    }
  });

  basictokenB.addEventListener("click", () => {

    let builder = new biscuit.Biscuit()
    let fact = biscuit.fact("right", [
      biscuit.symbol("authority"),
      biscuit.string("/a/file1.txt"),
      biscuit.symbol("read")
    ])
    builder.addAuthorityFact(fact)

    fact = biscuit.fact("right", [
      { symbol: "authority" },
      { string: "/b/file2.txt" },
      { symbol: "read" }
    ])
    builder.addAuthorityFact(fact)

    fact = biscuit.fact("right", [
      { symbol: "authority" },
      { string: "/a/file1.txt" },
      { symbol: "write" }
    ])
    builder.addAuthorityFact(fact)

    let token = builder.build(loadKeys())

    printToken(token);

  });
/*
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
*/

})();


