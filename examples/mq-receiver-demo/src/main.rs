use anyhow::Result;

use mq::*;
wit_bindgen_rust::import!("../../wit/mq.wit");

fn main() -> Result<()> {
    let resource_descriptor = get_mq()?;
    for _ in 0..3 {
        println!(
            "top message in the queue: {:#?}",
            std::str::from_utf8(&receive(&resource_descriptor)?)?
        );
    }
    Ok(())
}

impl From<mq::Error> for anyhow::Error {
    fn from(_: mq::Error) -> Self {
        anyhow::anyhow!("mq error")
    }
}