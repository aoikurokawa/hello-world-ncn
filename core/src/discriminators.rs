use jito_bytemuck::Discriminator;

use crate::{config::Config, message::Message};

/// Discriminators for HelloWorldNcn accounts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HelloWorldNcnDiscriminator {
    Config = 1,

    Message = 2,
}

impl Discriminator for Config {
    const DISCRIMINATOR: u8 = HelloWorldNcnDiscriminator::Config as u8;
}

impl Discriminator for Message {
    const DISCRIMINATOR: u8 = HelloWorldNcnDiscriminator::Message as u8;
}
