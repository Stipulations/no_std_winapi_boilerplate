compile with:

release:
cargo rustc --release -- -C link-args="/ENTRY:mainCRTStartup /SUBSYSTEM:console"

debug:
cargo rustc -- -C link-args="/ENTRY:mainCRTStartup /SUBSYSTEM:console"