use std::fmt::Debug;

use serde::Serialize;
use tokio::sync::watch;

use landscape_macro::LdApiError;

pub mod controller;
pub mod manager;

#[derive(thiserror::Error, Debug, LdApiError)]
#[api_error(crate_path = "crate")]
pub enum ServiceConfigError {
    #[error("{service_name} service config not found")]
    #[api_error(id = "service.config_not_found", status = 404)]
    NotFound { service_name: &'static str },
}

#[derive(Serialize, Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(tag = "t")]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    // 启动中
    Staring,
    // 正在运行
    Running,
    // 正在停止
    Stopping,
    // 停止运行
    #[default]
    Stop,
}

impl ServiceStatus {
    // 检查当前状态是否可以转换到目标状态
    pub fn can_transition_to(&self, target: &ServiceStatus) -> bool {
        let can = matches!(
            (self, target),
            (ServiceStatus::Stop, ServiceStatus::Staring)
                | (ServiceStatus::Staring, ServiceStatus::Running)
                | (ServiceStatus::Staring, ServiceStatus::Stopping)
                | (ServiceStatus::Staring, ServiceStatus::Stop)
                | (ServiceStatus::Running, ServiceStatus::Stopping)
                | (ServiceStatus::Running, ServiceStatus::Stop)
                | (ServiceStatus::Stopping, ServiceStatus::Stop)
        );
        if !can {
            tracing::error!(
                "can not change status, current status: {self:?}, target status: {target:?}"
            );
        }
        can
    }
}

/// 被观测的服务
#[derive(Clone, Debug)]
pub struct WatchService(pub watch::Sender<ServiceStatus>);

impl WatchService {
    pub fn new() -> Self {
        let (sender, _) = watch::channel(ServiceStatus::default());
        Self(sender)
    }

    pub fn just_change_status(&self, new_status: ServiceStatus) {
        self.0.send_if_modified(|current| {
            if current.can_transition_to(&new_status) {
                tracing::debug!("change to new status: {new_status:?}");
                *current = new_status;
            }
            true
        });
    }

    pub fn is_exit(&self) -> bool {
        matches!(*self.0.borrow(), ServiceStatus::Stopping | ServiceStatus::Stop)
    }

    pub fn is_running(&self) -> bool {
        matches!(*self.0.borrow(), ServiceStatus::Running)
    }

    pub fn is_stop(&self) -> bool {
        matches!(*self.0.borrow(), ServiceStatus::Stop)
    }

    pub fn subscribe(&self) -> watch::Receiver<ServiceStatus> {
        self.0.subscribe()
    }

    pub async fn changed(&self) -> Result<(), watch::error::RecvError> {
        self.0.subscribe().changed().await
    }

    /// Will not send stop
    pub async fn wait_to_stopping(&self) {
        let _ =
            self.0.subscribe().wait_for(|status| matches!(status, ServiceStatus::Stopping)).await;
    }

    /// will send `stopping` to service, and wait until stop
    pub async fn wait_stop(&self) {
        wait_status_stop(&self.0).await;
    }

    pub async fn wait_start(&self) {
        wait_status_running(&self.0).await;
    }
}

async fn wait_status_stop(ip_service_status: &watch::Sender<ServiceStatus>) {
    let mut do_wait = false;
    ip_service_status.send_if_modified(|status| {
        tracing::info!("当前服务的状态: {:?}", status);
        match status {
            ServiceStatus::Staring | ServiceStatus::Running => {
                if status.can_transition_to(&ServiceStatus::Stopping) {
                    *status = ServiceStatus::Stopping;
                }
                do_wait = true;
                true
            }
            ServiceStatus::Stopping => {
                do_wait = true;
                false
            }
            ServiceStatus::Stop { .. } => {
                do_wait = false;
                false
            }
        }
    });
    tracing::info!("当前需要等待之前状态结束吗?: {do_wait}");
    if do_wait {
        tracing::info!("那么进行等待");
        let _ = ip_service_status
            .subscribe()
            .wait_for(|status| matches!(status, ServiceStatus::Stop))
            .await;
        tracing::info!("前一个服务等待停止结束");
    }
}

async fn wait_status_running(ip_service_status: &watch::Sender<ServiceStatus>) {
    let mut do_wait = false;

    let status = ip_service_status.borrow().clone();
    if matches!(status, ServiceStatus::Staring) {
        do_wait = true;
    }
    if do_wait {
        let _ = ip_service_status
            .subscribe()
            .wait_for(|status| matches!(status, ServiceStatus::Running))
            .await;
    }
}

impl Serialize for WatchService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.borrow().serialize(serializer)
    }
}
