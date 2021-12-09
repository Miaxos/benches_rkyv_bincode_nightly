use rand::Rng;
use rand_pcg::Lcg64Xsh32;

#[derive(
    Clone,
    PartialEq,
    rkyv::Serialize,
    rkyv::Deserialize,
    rkyv::Archive,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct Periods {
    pub weekdays: Vec<u32>,
}

impl Default for Periods {
    fn default() -> Self {
        let state: u64 = rand::random();
        let stream: u64 = rand::random();

        let mut rand = Lcg64Xsh32::new(state, stream);

        Self {
            weekdays: std::iter::repeat(rand.gen_range(90_000_000_u32..99_000_000_u32))
                .take(2_000)
                .collect::<Vec<_>>(),
        }
    }
}
