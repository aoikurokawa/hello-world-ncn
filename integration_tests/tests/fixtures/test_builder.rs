use solana_program_test::{ProgramTest, ProgramTestContext};

use super::{
    hello_world_ncn_client::HelloWorldNcnClient,
    restaking_client::{NcnRoot, OperatorRoot, RestakingProgramClient},
    TestResult,
};

pub struct TestNcn {
    pub ncn_root: NcnRoot,
    pub operators: Vec<OperatorRoot>,
}

#[allow(dead_code)]
pub struct TestNcnNode {
    pub ncn_root: NcnRoot,
    pub operator_root: OperatorRoot,

    pub ncn_vault_connected: bool,
    pub operator_vault_connected: bool,
    pub delegation: u64,
}

pub struct TestBuilder {
    context: ProgramTestContext,
}

// impl Debug for TestBuilder {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "TestBuilder",)
//     }
// }

impl TestBuilder {
    pub async fn new() -> Self {
        // let run_as_bpf = std::env::vars().any(|(key, _)| key.eq("SBF_OUT_DIR"));

        let program_test = {
            let mut program_test = ProgramTest::new(
                "hello_world_ncn_program",
                hello_world_ncn_program::id(),
                None,
            );
            program_test.add_program("jito_restaking_program", jito_restaking_program::id(), None);

            // Tests that invoke this program should be in the "bpf" module so we can run them separately with the bpf vm.
            // Anchor programs do not expose a compatible entrypoint for solana_program_test::processor!

            program_test
        };
        // } else {
        //     let mut program_test = ProgramTest::new(
        //         "jito_tip_router_program",
        //         jito_tip_router_program::id(),
        //         processor!(jito_tip_router_program::process_instruction),
        //     );
        //     program_test.add_program(
        //         "jito_vault_program",
        //         jito_vault_program::id(),
        //         processor!(jito_vault_program::process_instruction),
        //     );
        //     program_test.add_program(
        //         "jito_restaking_program",
        //         jito_restaking_program::id(),
        //         processor!(jito_restaking_program::process_instruction),
        //     );
        //     program_test.add_program(
        //         "spl_stake_pool",
        //         spl_stake_pool::id(),
        //         processor!(spl_stake_pool::processor::Processor::process),
        //     );
        //     program_test
        // };

        // // Add switchboard account
        // {
        //     let switchboard_accounts = get_switchboard_accounts();

        //     for (address, account) in switchboard_accounts.iter() {
        //         program_test.add_account(*address, account.clone());
        //     }
        // }

        // // Stake pool keypair is needed to create the pool, and JitoSOL mint authority is based on this keypair
        // let stake_pool_keypair = Keypair::new();
        // let jitosol_mint_authority = find_withdraw_authority_program_address(
        //     &spl_stake_pool::id(),
        //     &stake_pool_keypair.pubkey(),
        // );
        // // Needed to create JitoSOL mint since we don't have access to the original keypair in the tests
        // program_test.add_account(JITOSOL_MINT, token_mint_account(&jitosol_mint_authority.0));

        Self {
            context: program_test.start_with_context().await,
        }
    }

    pub fn hello_world_ncn_client(&self) -> HelloWorldNcnClient {
        HelloWorldNcnClient::new(
            self.context.banks_client.clone(),
            self.context.payer.insecure_clone(),
        )
    }

    pub fn restaking_program_client(&self) -> RestakingProgramClient {
        RestakingProgramClient::new(
            self.context.banks_client.clone(),
            self.context.payer.insecure_clone(),
        )
    }

    pub async fn setup_ncn(&mut self) -> TestResult<NcnRoot> {
        let mut restaking_program_client = self.restaking_program_client();
        // let mut vault_program_client = self.vault_program_client();

        // vault_program_client.do_initialize_config().await?;
        restaking_program_client.do_initialize_config().await?;
        let ncn_root = restaking_program_client
            .do_initialize_ncn(Some(self.context.payer.insecure_clone()))
            .await?;

        Ok(ncn_root)
    }
}
