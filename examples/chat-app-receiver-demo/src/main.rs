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
    println!("📥 waiting to receive messages...");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let top_message = receive(&mq)?;
        if !top_message.is_empty() {
            println!(
                "📩 received a message saying: {}",
                std::str::from_utf8(&top_message)?
            );
        }
        set(&kv, &format!("message-{}", msg_counter), &top_message)?;
        msg_counter += 1;
    }
}
