use std::ops::Shr;

pub mod images;

macro_rules! tuple {
    ($name:ident, $length: literal) => {
        pub const $name: u8 = $length;

        paste::paste! {
            pub fn [<gen_rand_ $name:lower>](amount: u8) -> u64 {
                if amount == 0 {
                    return 0;
                }

                let mut rng = rand::thread_rng();
                let max = 2_u64.pow((amount * $length) as u32);
                let low = 2_u64.pow((amount.saturating_sub(1) * $length) as u32);
                use rand::Rng;
                rng.gen_range(low..max)
            }

            /// Biggest to smallest integers first
            pub fn [<make_ $name:lower _iter>](amount: u8, input: u64) -> impl Iterator<Item = u64> {
                let amount_capped = amount.min((u64::BITS.div_ceil($length)) as u8).max(1);

                let input = if amount == 0 { 0 } else { input };

                tracing::trace!("Input: {input:b} ({input})");

                let last_pair = (amount_capped as u32 * $length);

                (0..amount_capped).scan(last_pair, move |idx, _| {
                    let span = tracing::span!(tracing::Level::TRACE, "glyphs_iter");

                    let _enter = span.enter();
                    *idx -= $length;
                    let pair = input.shr(*idx) & (2_u64.pow($length) - 1);
                    tracing::trace!("Index: {idx}");

                    tracing::trace!("Pair: {pair:b} ({pair})");

                    Some(pair)
                })
            }
        }
    };
}

tuple!(PAIRS, 2);
tuple!(TRIPLETS, 3);
tuple!(QUADRUPLETS, 4);
