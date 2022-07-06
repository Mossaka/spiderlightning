use anyhow::Result;

use kv::*;
wit_bindgen_rust::import!("../../wit/kv.wit");
wit_error_rs::impl_error!(kv::Error);

use mq::*;
wit_bindgen_rust::import!("../../wit/mq.wit");
wit_error_rs::impl_error!(mq::Error);

fn main() -> Result<()> {
    let kv = get_kv("my-container")?;
    let mq = get_mq("wasi-cloud-queue")?;

    let mut msg_counter = 0;
    loop {
        println!("📨 write a message to send: ");
        let mut msg = "".to_string();
        std::io::stdin().read_line(&mut msg)?;
        send(&mq, msg.as_bytes())?;
        set(&kv, &format!("message-{}", msg_counter), msg.as_bytes())?;
        msg_counter += 1;
    }
}
