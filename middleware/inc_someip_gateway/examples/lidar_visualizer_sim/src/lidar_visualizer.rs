// *******************************************************************************
// Copyright (c) 2025 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache License Version 2.0 which is available at
// <https://www.apache.org/licenses/LICENSE-2.0>
//
// SPDX-License-Identifier: Apache-2.0
// *******************************************************************************

use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

const MW_COM_CONFIG_PATH: &str = "examples/lidar_visualizer_sim/config/mw_com_config.json";
const LIDAR_FRAME_INSTANCE_SPECIFIER_ID: &str = "lidar/LidarFrame1";
const SERVICE_DISCOVERY_SLEEP_DURATION: Duration = Duration::from_millis(500);

fn main() {
    mw_com::initialize(Some(Path::new(MW_COM_CONFIG_PATH)));

    let lidar_frame_instance_specifier =
        mw_com::InstanceSpecifier::try_from(LIDAR_FRAME_INSTANCE_SPECIFIER_ID)
            .expect("LiDAR frame instance specifier creation failed");

    loop {
        let handles = loop {
            let handles = mw_com::proxy::find_service(lidar_frame_instance_specifier.clone())
                .expect("LiDAR frame instance specifier resolution failed");

            if !handles.is_empty() {
                break handles;
            } else {
                println!("No LiDAR frame service found, retrying...");
                sleep(SERVICE_DISCOVERY_SLEEP_DURATION);
            }
        };

        let lidar_frame_types::LidarFrameInterface::Proxy { lidar_frame_ } =
            lidar_frame_types::LidarFrameInterface::Proxy::new(&handles[0])
                .expect("Failed to create LiDAR frame proxy");

        let subscribed_lidar_frame = lidar_frame_
            .subscribe(1)
            .expect("Failed to subscribe to LiDAR frame event");

        println!("LiDAR visualizer subscribed to internal middleware topic.");

        loop {
            if let Some(frame) = subscribed_lidar_frame.get_new_sample() {
                println!(
                    "[lidar_visualizer] frame_id={} timestamp_ns={} point_count={} payload_size={}",
                    frame.frame_id,
                    frame.timestamp_ns,
                    frame.point_count,
                    frame.payload_size
                );
            }

            sleep(Duration::from_millis(20));
        }
    }
}