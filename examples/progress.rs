// Copyright 2017-2022 allenbenz <allenbenz@users.noreply.github.com>
// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{thread::sleep, time::Duration as StdDuration};
use tauri_winrt_notification::{Toast, Duration, Progress, NotificationUpdateResult};

fn main() {
    let mut progress = Progress {
        tag: "my_tag".to_string(),
        title: "video.mp4".to_string(),
        status: "Transferring files...".to_string(),
        value: 0.0,
        value_string: "0/1000 MB".to_string(),
    };

    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("File Transfer from Phone")
        .text1("Transferring files to your computer...")
        .progress(&progress)
        .duration(Duration::Long)
        .show()
        .expect("Unable to send notification");

    for i in 1..=10 {
        sleep(StdDuration::from_secs(1));
        
        progress.value = i as f32 / 10.0;
        progress.value_string = format!("{}/1000 MB", i * 100);

        if i == 10 {
            progress.status = String::from("Completed");
        };

        if let Ok(update_result) = Toast::update_progress(Toast::POWERSHELL_APP_ID, &progress) {
            match update_result {
                NotificationUpdateResult::Succeeded => {
                    println!("Notification updated successfully.");
                },
                NotificationUpdateResult::Failed => {   
                    println!("Failed to update notification")
                },
                NotificationUpdateResult::NotificationNotFound => {
                    println!("Notification not found. Please ensure the notification ID and Tag are correct.");
                },
                _ => println!("Unknown notification update result"),
            }
        };
    }
}
