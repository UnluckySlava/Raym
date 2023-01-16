title building my-project
mkdir ray-marcher
cargo build --release
xcopy target\release\ray-marcher.exe ray-marcher /Y
mkdir ray-marcher\shaders
xcopy shaders ray-marcher\shaders /Y
xcopy SDL2.dll ray-marcher /Y
