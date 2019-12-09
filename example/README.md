## requirements

- install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- in the root folder, run `wasm-pack build --release`
- run `npm install`
- run `npm run-script serve`
- open http://127.0.0.1:8080/

## testing

- the `Basic rights token` button generates a token with read and write rights on `/apps/123`, read on `/apps/456`
- the `All rights token` button generates a token with all rights on all resources starting with `/`
- the `serialized token` text input can be used to copy and load any token for attenuation or verification
- the attenuation parts can (for now) add two restrictions: fix the operation type (read, write, etc) or set a prefix on the resource. Adding a restriction immediately generate a new token
- the verification part lets you define a resource name and an operation type. If it works, the text "OK" appears, otherwise the error(s) we saw in the verification
