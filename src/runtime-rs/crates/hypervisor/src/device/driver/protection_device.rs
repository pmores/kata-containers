// Copyright (c) 2023 Red Hat
//
// SPDX-License-Identifier: Apache-2.0
//



#[derive(Debug, Clone)]
pub enum ProtectionDeviceConfig {
    Snp(SnpConfig),
    Tdx(TdxConfig),
}


#[derive(Debug, Clone)]
struct SnpConfig {
    firmware: String,
    certs_path: String,
    reduced_phys_bits: u8,// (?)
}

#[derive(Debug, Clone)]
struct TdxConfig {
    firmware: String,
}

#[derive(Debug, Clone)]
pub struct ProtectionDevice {
    pub device_id: String,
    pub config: ProtectionDeviceConfig,
}

impl ProtectionDevice {
    fn new(device_id: &String, config: &ProtectionDeviceConfig) -> Self {
        Self {
            device_id: device_id.clone(),
            config: config.clone(),
        }
    }
}