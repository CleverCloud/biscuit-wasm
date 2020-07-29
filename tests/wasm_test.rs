use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

use biscuit_wasm::error::FailedCaveat::Verifier;
use biscuit_wasm::*;

#[wasm_bindgen(module = "tests/wasm_test.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn create_biscuit_with_authority_fact_and_verify_should_fail_on_caveat() -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    fn create_block_with_authority_fact_and_verify() -> Result<(), JsValue>;
}

#[wasm_bindgen_test]
fn wasm_create_biscuit_with_authority_fact_only_and_verify_should_fail_on_caveat() {
    let res = create_biscuit_with_authority_fact_and_verify_should_fail_on_caveat();

    if let Err(e) = res {
        let res = e.into_serde();

        if let Err(_) = res {
            panic!("{:#?}", e)
        } else {
            assert_eq!(
                error::Error::FailedLogic(error::Logic::FailedCaveats(vec![Verifier(
                    error::FailedVerifierCaveat {
                        caveat_id: 0,
                        rule: "*right(#right) <- right(#authority, \"file2\", #write)"
                            .to_string()
                    }
                )])),
                res.unwrap()
            );
        }
    } else {
        panic!("Should return a failed caveats error");
    }
}

#[wasm_bindgen_test]
fn wasm_create_block_with_authority_fact_only_and_verify() {
    let res = create_block_with_authority_fact_and_verify();

    if let Err(e) = res {
        panic!("{:#?}", e)
    }

    assert!(res.is_ok())
}
