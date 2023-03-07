# QuickMenu
Rust based program you can execute whenever you want and will pipe data into a rofi-like app launcher

The project is not ready yet

# Functionalities
- Desktop, app launcher agnostic
  - Should work on whatever linux desktop
  - Should be able to communicate with wthever app launcher you want (fuzzel, rofi, etc)
  - Thanks to this it won't care if you are on wayland or xorg
- Main menu
  - Power menu
    - Log Out
    - Suspend
    - Hibernate
    - Power Off
    - Restart
  - Power management menu
    - Performance mode
    - Normal mode
    - Power Saving mode
  - Tiling menu
    - Alternating
    
  - Input/Output Menu
  - Light mode menu
    - Dark mode
    - Light mode
    - Red light intensity
  - Wifi menu
  - Bluetooth menu
    - Enable/Disable
    - Pair device
    - Connect Device

# Motivations

I'm a big fan of the KISS philosophy as a minimalist. I also think that KISS is even more relevant for FOSS since it needs to be easy to adapt, understand, bug fix, etc.
Those are the reasons why I switched to a window manager on wayland.
But perfection is not quite there yet.
As of now, I'm using a bunch of applications using different frameworks and theme and that some time are only working on wayland.
I want to create a minimalist experience yet efficient and practical, desktop agnostic and compositor agnostic.
This is why I decided to work on a simple Quick Menu written in Rust.
After this I would love to build a wayland bar.
I'll start by writting some script to connect to already existing menu, then I'll create my own menu based on Iced GUI library and Rsocket.
