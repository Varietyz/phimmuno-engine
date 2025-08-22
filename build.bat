cargo build --release
cargo build --release --target x86_64-pc-windows-msvc
phi --phimmuno-remove
copy target\release\phimmuno-engine.exe "C:\Users\JayBa\.phicode\bin\"