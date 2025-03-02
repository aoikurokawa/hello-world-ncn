use std::fmt::{Debug, Formatter};

use solana_program_test::{processor, ProgramTest, ProgramTestContext};

use super::ncn_portal_client::NcnPortalProgramClient;

pub struct TestBuilder {
    pub context: ProgramTestContext,
}

impl Debug for TestBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestBuilder",)
    }
}

impl TestBuilder {
    pub async fn new() -> Self {
        // $ cargo-build-sbf && SBF_OUT_DIR=$(pwd)/target/sbf-solana-solana/release cargo nextest run
        let program_test = ProgramTest::new(
            "ncn_portal_program",
            ncn_portal_program::id(),
            processor!(ncn_portal_program::process_instruction),
        );
        let context = program_test.start_with_context().await;
        Self { context }
    }

    pub fn ncn_portal_program_client(&self) -> NcnPortalProgramClient {
        NcnPortalProgramClient::new(
            self.context.banks_client.clone(),
            self.context.payer.insecure_clone(),
        )
    }
}
