# [DGEWS](https://github.com/MrTitanHearted/dgews)

[![DGEWS](https://img.shields.io/badge/dgews-v0.1.4-important)](https://crates.io/crates/dgews/) ![DGEWS](https://img.shields.io/crates/l/dgews)

**_DGEWS_** is a simple multithreaded toy windowing system for only learning purposes.

```toml
[dependencies]
dgews="0.1.4"
```

----------------------------------------------------------------

**DGEWS** is an ordinary window manager for only _Windows_ users. It is pretty simple and straightforward so that anyone can understand and sometimes _learn_ something from it. Indead, my goal is actually to learn the cycle of programming a practical crates and the developement of them.

----------------------------------------------------------------

> ## Usage

**DGEWS** usues the c++ win32 api library wrapper [_winapi_](https://crates.io/crates/winapi) to build its windows. Therefore, anyone else except for _Windows OS_ users cannot use this crate. Currently, this crate is not stable and crashes sometimes, that is why it is better not to use it in production (though I think nobody will use it while [_winit_](https://crates.io/crates/winit) crate is ready to use). But, if someone wants to use some of its features in their own projects, just take it as I have stated that it is only for educational purposes.

----------------------------------------------------------------

> ## Example

**Manager** is a central point of this crate: it processes everything and users retrieve the event messages from it. Moreover, window or windows are created with that. The _run()_ method accepts a closure that must be called in each event and users will be given events, the manager itself as well as the control flow of that main events loop

```rust
extern crate dgews;
use dgews::prelude::*; // prelude module contains everything

fn main() {
    let mut manager = Manager::new(WindowBuilder::default()
            .with_title("DGEWS Window")
            .with_dimensions(800, 640)
            .with_theme(Theme::Dark)
            .with_resizable(true))
        .add_window("Hooray", WindowBuilder::new()
            .with_title("Finally")
            .with_dimensions(400, 300)
            .with_theme(Theme::Dark)
            .with_pos(700, 700));

    let _hwnd = manager.window().unwrap();

    manager.run(|events, control_flow, manager| {
        match events {
            Events::WindowEvents { id, event } => match event {
                WindowEvents::Create => println!("[INFO]: a new window with id: {} has been created", id),

                WindowEvents::Close => {
                    println!("[INFO]: a window with id: {} has been closed", id);
                    *control_flow = ControlFlow::Exit; // to exit with panicing, use ControlFlow::ExitWithCode(<your number>) instead.
                },

                WindowEvents::SetFocus => println!("[INFO]: window with id: {} gained the focus", id),

                WindowEvents::LostFocus => println!("[INFO]: window with id: {} Lost the focus", id),

                _=> {}
            },

            Events::MouseEvents { id: _, event } => match event {
                MouseEvents::MouseMove { x, y, last_x, last_y, dx, dy } => {
                    println!("[INFO]: mouse moved in the window with id {}: x={}, y={}, last_x={}, last_y={} dx={} dy={};", manager.window().unwrap().get_id(), x, y, last_x, last_y, dx, dy);
                },
                
                _=> {}
            }

            _=> *control_flow = ControlFlow::Continue,
        }

        if manager.get_key(Key::ESCAPE) == Action::Release {
            println!("[INFO]: program is exiting");
            *control_flow = ControlFlow::Exit;
        }
    });
}
```

----------------------------------------------------------------

> ## Features

### current features

* **_Multithreaded_**: windows have their own threads and they send messages from there so that when the user is interacting, windows will be still refreshing without waiting for the user to finish their event;
* _Winit stylish style_: when I saw the winit crate first time, I really liked it for its structure. Thus, I decided to make something that resembles it;
* _Icons_: users can use their own icons;
* _Themes_: there is now only light and dark themes;
* _~~Ready~~ events processing_: it actually needs some work there 🤷‍♂️;
* _Easy_: a glance of attention to the [_documentation_](https://docs.rs/dgews/latest/dgews/) is enough to utilize the crate;
* _HasRawWindowHandle_ and _HasRawDisplayHandle_ traits are implemeneted so that you can use them with other crates such as wgpu-rs;

### and its glitches and shortcomings

* _Not ready yet_: it is only in alpha mode;
* _System keys error_: I don't know why but some system keys such as Alt key are not working properly;
* **_Crashes_**: actually the reason I like rust lang is because it is very fast and safe at the time. However, I think due to the lack of my experience in multithreaded programming, I have lost that feature of having something working 100% all the time;
* _Not cross-platform_: I have used the Windows api crate, so no cross-platform support 😔;

### plans

* _Better documentation_: I think it has 'pretty nice' documentation. But still there is more to make it even better and more user-friendly.
* _Fix the bugs_: I am working on the issues like with the system keys or the window not opening,

----------------------------------------------------------------

> ### Contributions

Everyone is welcome who are willing to contribute even to the documentation. Thanks in advance.
Contact me via:

* My outlook email address: abduqodirovmuhammadhon@outlook.com;
* Or with google email address: abduqodirovmuhammadhon@gmail.com;
* And my id in telegram: @MrTitanHearted;

----------------------------------------------------------------

> I am a student of a lyceum that is why I think you won't get the response immediately, however, I will try my best to reply back as soon as possible.

----------------------------------------------------------------

## Licenses

1. [The MIT License](https://github.com/MrTitanHearted/dgews/LICENSE-MIT)
2. [The APACHE 2.0 License](https://github.com/MrTitanHearted/dgews/LICENSE-APACHE)
