use metrics_client::*;
use metrics_client::models::metrics::MixMetric;

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let cfg = Config::new("http://testnet-metrics.nymtech.net:8080".into());
    let client = Client::new(cfg);

    rt.block_on(client.post_mix_metrics(
            MixMetric {
                pub_key: "<your node's identity key>".to_string(),
                received: 10,
                sent: [("DiYR9o8KgeQ81woKPYVAu4LNaAEg8SWkiufDCahNnPov".to_string(), 10)].iter().cloned().collect()
            }
        ));
}
