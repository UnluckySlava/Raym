title building my-project
mkdir ray-marcher
cargo build
xcopy target\debug\raym.exe ray-marcher /Y
mkdir ray-marcher\shaders
xcopy shaders ray-marcher\shaders /Y
xcopy SDL2.dll ray-marcher /Y
