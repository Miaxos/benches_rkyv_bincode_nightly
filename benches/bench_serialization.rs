use bench_serialization::Periods;
use bincode::Options;
use criterion::Criterion;

fn serialize_periods(c: &mut Criterion) {
    let mut group = c.benchmark_group("Periods");

    let objects = std::iter::repeat(Periods::default())
        .take(10_000)
        .collect::<Vec<_>>();

    group.bench_function("Serialization/Bincode", |b| {
        b.iter(|| {
            criterion::black_box(
                bincode::DefaultOptions::new()
                    .with_fixint_encoding()
                    .allow_trailing_bytes()
                    .with_big_endian()
                    .with_no_limit()
                    .serialize(criterion::black_box(&objects)),
            )
        });
    });

    group.finish();
}

fn serialize_periods_rkyv(c: &mut Criterion) {
    use rkyv::{
        ser::{
            serializers::{AlignedSerializer, BufferScratch, CompositeSerializer},
            Serializer,
        },
        AlignedVec, Infallible,
    };

    const BUFFER_LEN: usize = 10_000_000;
    const SCRATCH_LEN: usize = 512_000;

    let mut group = c.benchmark_group("Periods");

    let mut serialize_buffer = AlignedVec::with_capacity(BUFFER_LEN);
    let mut serialize_scratch = AlignedVec::with_capacity(SCRATCH_LEN);
    unsafe {
        serialize_scratch.set_len(SCRATCH_LEN);
    }

    let objects = std::iter::repeat(Periods::default())
        .take(10_000)
        .collect::<Vec<_>>();

    group.bench_function("Serialization/rkyv", |b| {
        b.iter(|| {
            serialize_buffer.clear();

            let mut serializer = CompositeSerializer::new(
                AlignedSerializer::new(criterion::black_box(&mut serialize_buffer)),
                BufferScratch::new(criterion::black_box(&mut serialize_scratch)),
                Infallible,
            );

            criterion::black_box(
                serializer
                    .serialize_value(criterion::black_box(&objects))
                    .unwrap(),
            );
        });
    });

    group.finish();
}

criterion::criterion_group!(serialize_rykv, serialize_periods_rkyv);
criterion::criterion_group!(serialize_blbl, serialize_periods);

criterion::criterion_main!(serialize_rykv, serialize_blbl);
