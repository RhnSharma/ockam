use ockam::{route, Context, Entity, Identity, Result, TcpTransport, Vault, TCP};
use ockam_influxdb::{InfluxClient, InfluxError};
use std::io::{Read, Write};
use std::thread::sleep;
use std::time::Duration;

#[ockam::node]
async fn main(ctx: Context) -> Result<()> {
    // TCP
    let _tcp = TcpTransport::create(&ctx).await?;

    let vault = Vault::create(&ctx)?;
    let mut entity = Entity::create(&ctx, &vault)?;

    // Client application stuff
    let api_url = "http://127.0.0.1:8086";
    let org = "2498129fd117c716";
    let bucket = "ockam-bucket";
    let ttl = 5_000; // ms

    let lease_manager_route = route![(TCP, "127.0.0.1:4000"), "token_lease_service"];
    let leased_token = entity.get_lease(&lease_manager_route, org, bucket, ttl)?;
    let mut influx_client = InfluxClient::new(api_url, org, bucket, leased_token.value());

    print!("Sending metrics");
    loop {
        let response = influx_client.send_metrics().await;
        if let Err(influx_error) = response {
            if let InfluxError::Authentication(_) = influx_error {
                println!("\nAuthentication failed. Revoking lease.");
                entity.revoke_lease(&lease_manager_route, leased_token.clone())?;
                println!("Press enter to get a new lease.");
                std::io::stdin().read(&mut [0; 1]).unwrap();
                let leased_token = entity.get_lease(&lease_manager_route, org, bucket, ttl)?;
                influx_client.set_token(leased_token.value());
            }
        }
        print!(".");
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }
    //ctx.stop().await?;
    //Ok(())
}
