use ledger_apdu::APDUCommand;
use ledger_transport::Exchange;

use crate::api::{constants, errors, helpers};

pub fn exec(transport: &dyn Exchange) -> Result<(), errors::APIError> {
    let cmd = APDUCommand {
        cla: constants::APDUCLASSB0,
        ins: constants::APDUInstructionsBolos::AppExitB0 as u8,
        p1: 0,
        p2: 0,
        data: Vec::new(),
    };
    helpers::exec::<()>(transport, cmd)
}
