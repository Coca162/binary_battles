use tracing_subscriber::{filter::LevelFilter, EnvFilter};

use crate::glyphs::images::*;

pub mod glyphs;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::TRACE.into())
                .from_env_lossy(),
        )
        .init();

    loop {
        let (num, buffer1) = generate_random_triplets_image(3);
        let buffer2 = create_triplets_image(num);

        if buffer1 != buffer2 {
            println!("not chill");
            return;
        }

        buffer1.save(format!("./outputs/{num:b}.png")).unwrap();

        println!("chill");
    }
}
