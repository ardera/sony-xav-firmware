use std::io::Write;

use tracing::{info_span, level_filters::LevelFilter};
use tracing_subscriber::{fmt::format::FmtSpan, util::SubscriberInitExt};

fn main() {
    let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .with_writer(non_blocking)
        .with_span_events(FmtSpan::CLOSE)
        .finish()
        .init();

    let key: [u8; 16] = [
        0xb7, 0x72, 0x10, 0x03, 0x00, 0x8c, 0x82, 0x7e, 0xaa, 0xd1, 0x83, 0x58, 0x23, 0xef, 0x82,
        0x5c,
    ];

    let bytes = std::fs::read("./XAV-AX5550D_v200/SHSO2001.FIR").unwrap();
    let bytes = &bytes[0x80..];

    let mut file = std::fs::File::create("./XAV-AX5550D_V200/SHMD2001.FIR.decrypted").unwrap();

    let mut bytes_out = Vec::<u8>::new();
    bytes_out.reserve(bytes.len());

    info_span!("XOR").in_scope(|| {
        bytes
            .iter()
            .zip(key.iter().cycle())
            .zip(bytes_out.iter_mut())
            .for_each(|((&byte, &key), out)| {
                *out = byte ^ key;
            });
    });

    info_span!("write").in_scope(|| {
        file.write_all(bytes_out.as_ref()).unwrap();
        file.sync_all().unwrap();
    });

    info_span!("drop file").in_scope(|| {
        drop(file);
    });

    drop(guard);
}
