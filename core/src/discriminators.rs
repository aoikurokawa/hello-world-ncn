use jito_bytemuck::Discriminator;

use crate::message::Message;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HelloWorldNcnDiscriminator {
    Config = 1,

    Message = 2,
}

impl Discriminator for Message {
    const DISCRIMINATOR: u8 = HelloWorldNcnDiscriminator::Message as u8;
}
