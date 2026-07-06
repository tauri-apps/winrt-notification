// Copyright 2017-2022 allenbenz <allenbenz@users.noreply.github.com>
// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::path::{absolute, Path};
use tauri_winrt_notification::{IconCrop, Toast};

fn main() {
    // Paths must be absolute and can look like this
    // C:\\absolute\\path\\to\\image.jpeg
    // c:/this/style/works/too/image.png
    // UNC paths, so paths starting with \\?\ are not supported.
    // Rust's canonicalize() function returns UNC paths so you want to use absolute() or the dunce crate instead.
    let icon_path = absolute(Path::new("./examples/icon.png")).unwrap();
    Toast::new(Toast::POWERSHELL_APP_ID)
        .hero(&icon_path, "alt text")
        .icon(&icon_path, IconCrop::Circular, "alt text")
        .title("Lots of pictures here")
        .text1("One above the text as the hero")
        .text2("One to the left as an icon, and several below")
        .image(&icon_path, "the sun")
        .image(&icon_path, "the moon")
        .sound(None) // will be silent
        .show()
        .expect("notification failed");
}
