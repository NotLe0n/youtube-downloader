# Youtube Downloader
A simple and fast (🚀) desktop application to download videos and music from youtube.

<img src="https://user-images.githubusercontent.com/26361108/174882469-85d3bde3-1577-4e0d-ad86-d649f4359309.png" 
     width=50%
     height=50%>

### Dependencies
* youtube-dl
* rfd
* egui
* eframe
* serde

### Testing locally

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra speech-dispatcher-devel libxkbcommon-devel pkg-config openssl-devel libxcb-devel`
