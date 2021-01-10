cargo build --release

mv target/release/task ./gui/bin/task_unix


cargo build --release  --target x86_64-pc-windows-gnu

mv target/x86_64-pc-windows-gnu/release/task.exe ./gui/bin/task.exe

