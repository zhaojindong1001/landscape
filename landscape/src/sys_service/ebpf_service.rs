use std::sync::Arc;

use tokio::sync::{oneshot, Mutex};

#[derive(Clone)]
#[allow(dead_code)]
pub struct LandscapeEbpfService {
    tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

impl LandscapeEbpfService {
    pub fn new() -> Self {
        let (tx, rx) = oneshot::channel::<()>();
        std::thread::spawn(move || {
            landscape_ebpf::base::ip_mac::neigh_update(rx).unwrap();
        });

        LandscapeEbpfService { tx: Arc::new(Mutex::new(Some(tx))) }
    }

    pub async fn stop(&self) {
        if let Some(tx) = self.tx.lock().await.take() {
            let _ = tx.send(());
            tracing::info!("eBPF neigh_update service stop signal sent");
        }
    }
}
