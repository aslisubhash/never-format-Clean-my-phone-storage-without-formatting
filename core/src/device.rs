use crate::error::Result;
use crate::transport::{AdbTransport, DeviceTransport, IosTransport, MockTransport, MtpTransport};
use crate::types::DeviceInfo;

pub struct DeviceManager {
    pub include_mock: bool,
}

impl Default for DeviceManager {
    fn default() -> Self {
        Self {
            include_mock: cfg!(debug_assertions),
        }
    }
}

impl DeviceManager {
    pub fn discover(&self) -> Result<Vec<DeviceInfo>> {
        let mut devices = Vec::new();

        if let Ok(adb) = AdbTransport::detect() {
            if let Ok(list) = adb.list_devices() {
                devices.extend(list);
            }
        }

        let mtp = MtpTransport::probe();
        if mtp.is_available() {
            if let Ok(list) = mtp.list_devices() {
                devices.extend(list);
            }
        }

        let ios = IosTransport::probe();
        if let Ok(list) = ios.list_devices() {
            devices.extend(list);
        }

        if devices.is_empty() && self.include_mock {
            let mock = MockTransport::default();
            devices.extend(mock.list_devices()?);
        }

        Ok(devices)
    }

    pub fn transport_for(device: &DeviceInfo) -> Result<Box<dyn DeviceTransport>> {
        match device.transport.as_str() {
            "adb" => Ok(Box::new(AdbTransport::detect()?)),
            "mock" => Ok(Box::new(MockTransport::default())),
            "ios" => Ok(Box::new(IosTransport::probe())),
            "mtp" => Ok(Box::new(MtpTransport::probe())),
            other => Err(crate::error::Error::Transport(format!(
                "unknown transport: {other}"
            ))),
        }
    }
}
