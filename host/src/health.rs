use std::time::{Duration, Instant};

const HEALTH_CHECK_TIMEOUT: Duration = Duration::from_millis(2000);
const READY_MSG: &str = "READY:adc,tim,clk";
const FAIL_MSG: &str = "FAIL:";
const RTT_CHANNEL_ID: usize = 0;
const RTT_READ_BUFFER_SIZE: usize = 64;
const POLL_INTERVAL: Duration = Duration::from_millis(10);



pub fn wait_for_board_ready(rtt_connection: &mut probe_rs::rtt::Rtt)-> anyhow::Result<()>{
    let start_time = Instant::now();
    let received_text = String::new();

    // check channel connection
    let mut rtt_channel = rtt_connection
        .channel(RTT_CHANNEL_ID)
        .expect("RTT channel 0 not found — is defmt-rtt in the firmware?");
    
    loop {
        // CHECK TIMEOUT
        if start_time.elapsed() > HEALTH_CHECK_TIMEOUT{
            anyhow::bail!(
                "health check timed out after {}ms — is the firmware flashed?",
                HEALTH_CHECK_TIMEOUT.as_millis()
            );
        }

        // READING RTT - Real time transfer
        let mut read_buffer = [0u8; RTT_READ_BUFFER_SIZE];
        let bytes_read = rtt_channel.read(&mut read_buffer)?;

        if bytes_read > 0 {
            let text_chunk = string::from_utf8_lossy(&read_buffer[..bytes_read]);
            received_text.push_str(&text_chunk);
        }

        // Check for both outcomes — ready and failed
        match () {
            _ if received_text.contains(READY_MSG) => {
                println!("board ready — health check passed");
                return Ok(());
            }
            _ if received_text.contains(FAIL_MSG) => {
                anyhow::bail!(
                    "firmware self-test failed: {}",
                    received_text.trim()
                );
            }
            // nothing meaningful yet — keep polling
            _ => {}
        }

        std::thread::sleep(POLL_INTERVAL);
    }
}
