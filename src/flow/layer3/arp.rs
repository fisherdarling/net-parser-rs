use crate::flow::Flow;
use crate::flow::errors::Error;
use crate::flow::info::layer2::Info;
use crate::flow::layer3::FlowExtraction;
use crate::layer3::Arp;

pub mod errors {
    use crate::errors::Error as NetParserError;
    use thiserror::{Error as ThisError};

    #[derive(Debug, ThisError)]
    pub enum Error {
        #[error("Error parsing ARP")]
        NetParser(#[from] NetParserError),
        #[error("ARP cannot be converted to a flow")]
        Flow,
    }

    unsafe impl Sync for Error {}
    unsafe impl Send for Error {}
}

impl FlowExtraction for Arp {
    fn extract_flow(&self, _l2: Info) -> Result<Flow, Error> {
        Err(Error::L3(errors::Error::Flow.into()))
    }
}