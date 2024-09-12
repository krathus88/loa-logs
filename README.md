# <img src="https://i.imgur.com/eej47rS.png" width="30"/> LOA Logs

[![GitHub](https://img.shields.io/github/downloads/krathus88/loa-logs/total?style=for-the-badge&color=%23ff9800)](https://github.com/krathus88/loa-logs/releases/latest) [![Discord](https://img.shields.io/discord/1278117034756018319?color=%235865F2&label=Discord&style=for-the-badge)](https://discord.gg/dVNBVNJUh5)

[![GitHub](https://img.shields.io/github/v/release/krathus88/loa-logs?style=flat-square)](https://github.com/krathus88/loa-logs/releases)
[![GitHub](https://img.shields.io/github/license/krathus88/loa-logs?style=flat-square)](https://github.com/krathus88/loa-logs/blob/master/LICENSE)

[<img src="static/kofi.png" alt="Ko-fi" width="230"/>](https://ko-fi.com/synow)

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/synow)

LOA Logs is a "blazingly fast" open source Lost Ark DPS meter, written in Rust by [Snow](https://github.com/snoww).

This project is an opinionated flavor of [LOA Details](https://github.com/lost-ark-dev/loa-details) by Herysia and Mathi, but should share very similar user interfaces and settings. The packet sniffing and processing has been completely ported over to Rust, with [`meter-core-rs`](https://github.com/snoww/meter-core-rs). The Rust port could not be made without Herysia and Henjuro's work on [`meter-core`](https://github.com/lost-ark-dev/meter-core).

This project was designed specifically with hell-raiding in mind.

# Download

https://github.com/krathus88/loa-logs/releases

\*currently only Windows 7 and up is supported

> [!IMPORTANT]
>
> ### Prerequisites
>
> Npcap is required to run LOA Logs.
>
> Download [here](https://npcap.com/#download).

# Supporting the Project

You can support me directly by buying me a [coffee.](https://www.buymeacoffee.com/synow)

You can also support LOA Details' Herysia's [Patreon.](https://patreon.com/Herysia)

# FAQ

#### Q: Meter window is missing / meter window is tiny

A: Right-click the taskbar icon (located in the bottom right of your screen, next to the system time), click reset position, or load saved position. Adjust the size of the window and location, and then save the position.

#### Q: Meter isn't detecting anything...

A: There can be multiple reasons. Did you install Npcap? If that still doesn't work, enable raw socket mode by doing the following: _Settings > General > Un-check Auto Network Selection > Raw Socket_. You must restart the meter as admin.

#### Q: How to use ExitLag with LOA Logs?

A: ExitLag recently updated their settings which changed how they redirect packets. Change your ExitLag settings to _Packet redirection method > Legacy - NDIS_. If that still doesn't work. Turn on raw socket by following the steps above.

#### Q: Should I run it in a VM?

A: Probably unnecessary. Meter is currently in a gray area by AGS, and they have not been banning any users for using it.

#### Q: Missing `packet.dll`

A: You need install Npcap. If you already have Npcap installed and error still shows, please uninstall it, and then reinstall the latest version using the link above.

#### Q: The installer crashes or takes forever to install

A: Are you trying to install on a custom install folder with different permissions? You might need to run the installer in administrator mode due to permission issues.

#### Q: The meter crashes immediately when trying to open it.

A: There could be two possible reasons. 1. The meter needs Microsoft Edge Webview2 Runtime to run. Yours is probably missing or out of date. Go uninstall it first (it won't let you install it if you have an older version installed), then download and install from [here](https://go.microsoft.com/fwlink/p/?LinkId=2124703) (https://go.microsoft.com/fwlink/p/?LinkId=2124703). 2. If you installed the meter in another folder that might require elevated permissions, you would need to run the program in administrator mode.

#### Q: The meter window lags a lot when dragging around.

A: Are you on Windows 11? Disable blur in the settings (settings > accessibility). If you wish to have a dark background with blur disabled, also disable the transparency setting to have a pseudo dark mode.

#### Q: Why isn't my item level shown next to my name when others have it?

A: You opened the meter too late, and it wasn't able to get your character information. It is doing its best by guessing. You can fix this by: switching characters, or changing parties around. (note: you need to enable "show gear score" in settings to show item level)

#### Q: There are too many/too few columns in the meter.

A: You can change whatever column you want to show in the settings. TIP: you can `SHIFT+SCROLL` to scroll horizontally.

#### Q: Are you going to implement rDPS like LOA Details?

A: rDPS is finally implemented as of v1.8.0. Make sure to thank Herysia and Mathi.

#### Q: Help, my issue isn't listed here.

A: Create an issue here on GitHub, or send a message in the #loa-logs channel on Discord. [(invite)](https://discord.gg/sbSa3pkDF5)

#### Q: Is it really "blazingly fast"?

A: [Yes.](https://i.imgur.com/QsLAntt.png)

## Screenshots

### In-game Overlay (optional Boss HP bar)

![log_image](https://i.imgur.com/luHu7Fz.png)

### Damage Breakdown with DPS Charts

<img src="https://i.imgur.com/T4HX6XK.png" width="500"/>

### rDPS

<img src="https://i.imgur.com/cxKz9pP.png"/>

### Skill Breakdown

<img src="https://i.imgur.com/P5Mb9oe.png" width="600"/>

### Arcana Card Tracking

<img src="https://i.imgur.com/afoAVOZ.png" width="500"/>

### Buff Uptime Tracking

<img src="https://i.imgur.com/9SkFQs3.png" width="800"/>

### Opener Rotation

<img src="https://i.imgur.com/hcpHAKG.png" width="600"/>

### Past Encounters

<img src="https://i.imgur.com/RZT6Rww.png" width="500"/>

#### Search Filters

<img src="https://i.imgur.com/5aJJISG.png" width="400"/>
