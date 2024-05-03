mod abi;

use substreams::hex;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::rpc::RpcBatch;

const TELLERV2_TRACKED_CONTRACT: [u8; 20] = hex!("00182fdb0b880ee24d428e3cc39383717677c37e");

#[substreams::handlers::map]
pub fn map_block(blk: Block) -> Option<()> {
    substreams::log::info!("block number: {}", blk.number);

    let mut batch = RpcBatch::new();
    let mut call_keys: Vec<String> = vec![];

    let commitments = vec![
        (
            "id",
            // WETH ERC20
            hex!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"),
            // Random address found on Etherscan, for balanceOf
            hex!("9def7cde171841a9f0724124ca0b01a622d749e4"),
        ),
        (
            "id",
            // USDC
            hex!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"),
            // Random address found on Etherscan, for balanceOf
            hex!("51ce8dbc24bfce2998b642375e38edabc17cd6c8"),
        ),
        (
            "id",
            // USDC
            hex!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"),
            // Random address found on Etherscan, for balanceOf
            hex!("51ce8dbc24bfce2998b642375e38edabc17cd6c8"),
        ),
        (
            "id",
            // GRT
            hex!("c944E90C64B2c07662A292be6244BDf05Cda44a7"),
            // Random address found on Etherscan, for balanceOf
            hex!("b8aa083dcf72901455bbac29468543c906df1c99"),
        ),
    ];

    for (i, (id, principal_token_address, lender)) in commitments.iter().enumerate() {
        let commitment_id = format!("{}:{}", id, i);

        call_keys.push(format!("{}:balance", &commitment_id));
        call_keys.push(format!("{}:allowance", &commitment_id));

        batch = batch
            .add(
                abi::erc20::functions::BalanceOf {
                    param0: lender.to_vec(),
                },
                principal_token_address.to_vec(),
            )
            .add(
                abi::erc20::functions::Allowance {
                    param0: lender.to_vec(),
                    param1: TELLERV2_TRACKED_CONTRACT.into(),
                },
                principal_token_address.to_vec(),
            );
    }

    let batch_response = batch.execute().expect("failed to execute batch");

    substreams::log::info!("batch_response: {:?}", batch_response.responses.len());

    Some(())
}
