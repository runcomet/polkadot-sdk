// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
// Generated, do not edit!
// See ethereum client README.md for instructions to generate

use hex_literal::hex;
use snowbridge_beacon_primitives::{
	types::deneb, AncestryProof, BeaconHeader, ExecutionProof, VersionedExecutionPayloadHeader,
};
use snowbridge_inbound_queue_primitives::{EventProof, InboundQueueFixture, Log, Proof};
use sp_core::U256;
use sp_std::vec;

pub fn make_send_token_to_penpal_message() -> InboundQueueFixture {
	InboundQueueFixture {
        event: EventProof {
            event_log: 	Log {
                address: hex!("eda338e4dc46038493b885327842fd3e301cab39").into(),
                topics: vec![
                    hex!("7153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84f").into(),
                    hex!("c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539").into(),
                    hex!("be323bced46a1a49c8da2ab62ad5e974fd50f1dabaeed70b23ca5bcf14bfe4aa").into(),
                ],
                data: hex!("00000000000000000000000000000000000000000000000000000000000000030000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000007300a736aa00000000000187d1f7fdfee7f651fabc8bfcb6e086c278b77a7d01d00700001cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c00286bee000000000000000000000000000064a7b3b6e00d000000000000000000e40b5402000000000000000000000000000000000000000000000000").into(),
            },
            proof: Proof {
                receipt_proof: (vec![
                    hex!("106f1eaeac04e469da0020ad5c8a72af66323638bd3f561a3c8236063202c120").to_vec(),
                ], vec![
                    hex!("f90471822080b9046b02f904670183017d9cb9010000800000000000008000000000000000000000000000004000000000000000000400000000004000000000001000000010000000000000000000001008000000000000000000000001000008000040000000000000000000000000008000080000000000200000000000000000000000000100000000000000000010000000000000020000000000000000000000000000003000000000080018000000000000000000040004000021000000002000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000200820000000000f9035cf89b9487d1f7fdfee7f651fabc8bfcb6e086c278b77a7df863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa000000000000000000000000090a987b944cb1dcce5564e5fdecd7a54d3de27fea000000000000000000000000057a2d4ff0c3866d96556884bf09fecdd7ccd530ca00000000000000000000000000000000000000000000000000de0b6b3a7640000f9015d94eda338e4dc46038493b885327842fd3e301cab39f884a024c5d2de620c6e25186ae16f6919eba93b6e2c1a33857cc419d9f3a00d6967e9a000000000000000000000000087d1f7fdfee7f651fabc8bfcb6e086c278b77a7da000000000000000000000000090a987b944cb1dcce5564e5fdecd7a54d3de27fea000000000000000000000000000000000000000000000000000000000000007d0b8c000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000de0b6b3a76400000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000201cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07cf9015c94eda338e4dc46038493b885327842fd3e301cab39f863a07153f9357c8ea496bba60bf82e67143e27b64462b49041f8e689e1b05728f84fa0c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539a0be323bced46a1a49c8da2ab62ad5e974fd50f1dabaeed70b23ca5bcf14bfe4aab8e000000000000000000000000000000000000000000000000000000000000000030000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000007300a736aa00000000000187d1f7fdfee7f651fabc8bfcb6e086c278b77a7d01d00700001cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c00286bee000000000000000000000000000064a7b3b6e00d000000000000000000e40b5402000000000000000000000000000000000000000000000000").to_vec(),
                ]),
                execution_proof: ExecutionProof {
                    header: BeaconHeader {
                        slot: 4235,
                        proposer_index: 4,
                        parent_root: hex!("1b31e6264c19bcad120e434e0aede892e7d7c8ed80ab505cb593d9a4a16bc566").into(),
                        state_root: hex!("725f51771a0ecf72c647a283ab814ca088f998eb8c203181496b0b8e01f624fa").into(),
                        body_root: hex!("6f1c326d192e7e97e21e27b16fd7f000b8fa09b435ff028849927e382302b0ce").into(),
                    },
                        ancestry_proof: Some(AncestryProof {
                        header_branch: vec![
                            hex!("1b31e6264c19bcad120e434e0aede892e7d7c8ed80ab505cb593d9a4a16bc566").into(),
                            hex!("335eb186c077fa7053ec96dcc5d34502c997713d2d5bc4eb74842118d8cd5a64").into(),
                            hex!("326607faf2a7dfc9cfc4b6895f8f3d92a659552deb2c8fd1e892ec00c86c734c").into(),
                            hex!("4e20002125d7b6504df7c774f3f48e018e1e6762d03489149670a8335bba1425").into(),
                            hex!("e76af5cd61aade5aec8282b6f1df9046efa756b0466bba5e49032410f7739a1b").into(),
                            hex!("ee4dcd9527712116380cddafd120484a3bedf867225bbb86850b84decf6da730").into(),
                            hex!("e4687a07421d3150439a2cd2f09f3b468145d75b359a2e5fa88dfbec51725b15").into(),
                            hex!("38eaa78978e95759aa9b6f8504a8dbe36151f20ae41907e6a1ea165700ceefcd").into(),
                            hex!("1c1b071ec6f13e15c47d07d1bfbcc9135d6a6c819e68e7e6078a2007418c1a23").into(),
                            hex!("0b3ad7ad193c691c8c4ba1606ad2a90482cd1d033c7db58cfe739d0e20431e9e").into(),
                            hex!("ffff0ad7e659772f9534c195c815efc4014ef1e1daed4404c06385d11192e92b").into(),
                            hex!("6cf04127db05441cd833107a52be852868890e4317e6a02ab47683aa75964220").into(),
                            hex!("b2ffec5f2c14640305dd941330f09216c53b99d198e93735a400a6d3a4de191f").into(),
                        ],
                        finalized_block_root: hex!("08be7a59e947f08cd95c4ef470758730bf9e3b0db0824cb663ea541c39b0e65c").into(),
                        }),
                    execution_header: VersionedExecutionPayloadHeader::Deneb(deneb::ExecutionPayloadHeader {
                        parent_hash: hex!("5d1186ae041f58785edb2f01248e95832f2e5e5d6c4eb8f7ff2f58980bfc2de9").into(),
                        fee_recipient: hex!("0000000000000000000000000000000000000000").into(),
                        state_root: hex!("2a66114d20e93082c8e9b47c8d401a937013487d757c9c2f3123cf43dc1f656d").into(),
                        receipts_root: hex!("106f1eaeac04e469da0020ad5c8a72af66323638bd3f561a3c8236063202c120").into(),
                        logs_bloom: hex!("00800000000000008000000000000000000000000000004000000000000000000400000000004000000000001000000010000000000000000000001008000000000000000000000001000008000040000000000000000000000000008000080000000000200000000000000000000000000100000000000000000010000000000000020000000000000000000000000000003000000000080018000000000000000000040004000021000000002000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000200820000000000").into(),
                        prev_randao: hex!("92e063c7e369b74149fdd1d7132ed2f635a19b9d8bff57637b8ee4736576426e").into(),
                        block_number: 4235,
                        gas_limit: 30000000,
                        gas_used: 97692,
                        timestamp: 1710556655,
                        extra_data: hex!("d983010d0b846765746888676f312e32312e368664617277696e").into(),
                        base_fee_per_gas: U256::from(7u64),
                        block_hash: hex!("ce24fe3047aa20a8f222cd1d04567c12b39455400d681141962c2130e690953f").into(),
                        transactions_root: hex!("0c8388731de94771777c60d452077065354d90d6e5088db61fc6a134684195cc").into(),
                        withdrawals_root: hex!("792930bbd5baac43bcc798ee49aa8185ef76bb3b44ba62b91d86ae569e4bb535").into(),
                        blob_gas_used: 0,
                        excess_blob_gas: 0,
                    }),
                    execution_branch: vec![
                            hex!("99d397fa180078e66cd3a3b77bcb07553052f4e21d447167f3a406f663b14e6a").into(),
                            hex!("b46f0c01805fe212e15907981b757e6c496b0cb06664224655613dcec82505bb").into(),
                            hex!("db56114e00fdd4c1f85c892bf35ac9a89289aaecb1ebd0a96cde606a748b5d71").into(),
                            hex!("53ddf17147819c1abb918178b0230d965d1bc2c0d389f45e91e54cb1d2d468aa").into(),
                    ],
                }
            },
        },
        finalized_header: BeaconHeader {
            slot: 4672,
            proposer_index: 4,
            parent_root: hex!("951233bf9f4bddfb2fa8f54e3bd0c7883779ef850e13e076baae3130dd7732db").into(),
            state_root: hex!("4d303003b8cb097cbcc14b0f551ee70dac42de2c1cc2f4acfca7058ca9713291").into(),
            body_root: hex!("664d13952b6f369bf4cf3af74d067ec33616eb57ed3a8a403fd5bae4fbf737dd").into(),
        },
        block_roots_root: hex!("af71048297c070e6539cf3b9b90ae07d86d363454606bc239734629e6b49b983").into(),
    }
}
