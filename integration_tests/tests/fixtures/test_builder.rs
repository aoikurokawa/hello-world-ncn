use solana_program_test::{processor, ProgramTest, ProgramTestContext};
use solana_sdk::clock::Clock;

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

impl TestBuilder {
    pub async fn new() -> Self {
        // let run_as_bpf = std::env::vars().any(|(key, _)| key.eq("SBF_OUT_DIR"));

        let program_test = {
            let mut program_test = ProgramTest::new(
                "hello_world_ncn_program",
                hello_world_ncn_program::id(),
                processor!(hello_world_ncn_program::process_instruction),
            );
            program_test.add_program(
                "jito_restaking_program",
                jito_restaking_program::id(),
                processor!(jito_restaking_program::process_instruction),
            );

            program_test
        };

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

    pub async fn get_current_slot(&mut self) -> TestResult<u64> {
        let clock: Clock = self.context.banks_client.get_sysvar().await?;
        Ok(clock.slot)
    }
}
