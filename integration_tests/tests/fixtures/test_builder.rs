use solana_program_test::{processor, BanksClientError, ProgramTest, ProgramTestContext};
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

    pub async fn warp_slot_incremental(&mut self, incremental_slots: u64) -> TestResult<()> {
        let clock: Clock = self.context.banks_client.get_sysvar().await?;
        self.context
            .warp_to_slot(clock.slot.checked_add(incremental_slots).unwrap())
            .map_err(|_| BanksClientError::ClientError("failed to warp slot"))?;
        Ok(())
    }

    // 1. Setup NCN
    pub async fn create_test_ncn(&mut self) -> TestResult<TestNcn> {
        let mut restaking_program_client = self.restaking_program_client();

        // let mut vault_program_client = self.vault_program_client();

        // vault_program_client.do_initialize_config().await?;

        restaking_program_client.do_initialize_config().await?;

        let ncn_root = restaking_program_client
            .do_initialize_ncn(Some(self.context.payer.insecure_clone()))
            .await?;

        // tip_router_client.setup_tip_router(&ncn_root).await?;

        // tip_router_client
        //     .do_set_config_fees(
        //         Some(300),
        //         None,
        //         Some(self.context.payer.pubkey()),
        //         Some(270),
        //         None,
        //         Some(15),
        //         &ncn_root,
        //     )
        //     .await?;

        Ok(TestNcn {
            ncn_root: ncn_root.clone(),
            operators: vec![],
            // vaults: vec![],
        })
    }

    // 2. Setup Operators
    pub async fn add_operators_to_test_ncn(
        &mut self,
        test_ncn: &mut TestNcn,
        operator_count: usize,
        operator_fees_bps: Option<u16>,
    ) -> TestResult<()> {
        let mut restaking_program_client = self.restaking_program_client();

        for _ in 0..operator_count {
            let operator_root = restaking_program_client
                .do_initialize_operator(operator_fees_bps)
                .await?;

            // ncn <> operator
            restaking_program_client
                .do_initialize_ncn_operator_state(
                    &test_ncn.ncn_root,
                    &operator_root.operator_pubkey,
                )
                .await?;
            self.warp_slot_incremental(1).await.unwrap();
            restaking_program_client
                .do_ncn_warmup_operator(&test_ncn.ncn_root, &operator_root.operator_pubkey)
                .await?;
            restaking_program_client
                .do_operator_warmup_ncn(&operator_root, &test_ncn.ncn_root.ncn_pubkey)
                .await?;

            test_ncn.operators.push(operator_root);
        }

        Ok(())
    }
}
