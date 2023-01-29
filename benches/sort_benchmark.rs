use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

const TRANSLATE_MAP: [i32; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 16
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, // 32
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, // 48
    54, 55, 56, 57, 58, 59, 60, 61, 62, 0, 0, 0, 0, 0, 0, 0, // 64
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, // 80
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 0, 0, 0, 0, 0, 0, // 96
    0, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, // 112
    41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 0, 0, 0, 0, 0, // 128
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 144
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 160
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 176
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 192
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 208
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 224
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 240
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 256
];

const ALPHABET_SIZE: i32 = 63;

fn nchunks_num<const T: usize>(doc: &[u8]) -> Vec<(i32, usize)> {
    assert!(T >= 2 && T < 5);
    let total_chunks = (doc.len() + T - 1) / T;
    let mut chunks: Vec<(i32, usize)> = Vec::with_capacity(total_chunks);

    if T == 2 {
        for (i, nchunk) in doc.chunks(T).enumerate() {
            // let nchunk = &doc[i * T..i * T + T];
            let nchunk_num = (TRANSLATE_MAP[nchunk[0] as usize] * ALPHABET_SIZE)
                + TRANSLATE_MAP[nchunk[1] as usize];
            chunks.push((nchunk_num, i * T));
        }
    } else if T == 3 {
        for (i, nchunk) in doc.chunks(T).enumerate() {
            // let nchunk = &doc[i * T..i * T + T];
            let nchunk_num = (TRANSLATE_MAP[nchunk[0] as usize] * (ALPHABET_SIZE * ALPHABET_SIZE))
                + TRANSLATE_MAP[nchunk[1] as usize] * ALPHABET_SIZE
                + TRANSLATE_MAP[nchunk[2] as usize];
            chunks.push((nchunk_num, i * T));
        }
    } else if T == 4 {
        for (i, nchunk) in doc.chunks(T).enumerate() {
            // let nchunk = &doc[i * T..i * T + T];
            let nchunk_num = (TRANSLATE_MAP[nchunk[0] as usize]
                * (ALPHABET_SIZE * ALPHABET_SIZE * ALPHABET_SIZE))
                + TRANSLATE_MAP[nchunk[1] as usize] * (ALPHABET_SIZE * ALPHABET_SIZE)
                + TRANSLATE_MAP[nchunk[2] as usize] * ALPHABET_SIZE
                + TRANSLATE_MAP[nchunk[3] as usize];
            chunks.push((nchunk_num, i * T));
        }
    }

    chunks.sort();
    chunks
}

fn nchunks_chars<const T: usize>(doc: &[u8]) -> Vec<(&[u8], usize)> {
    assert!(T >= 2 && T < 5);
    if doc.len() % T != 0 {}
    let total_chunks = (doc.len() + T - 1) / T;
    let mut chunks: Vec<(&[u8], usize)> = Vec::with_capacity(total_chunks);

    if T == 2 {
        for i in 0..total_chunks {
            let nchunk = &doc[i * T..i * T + T];
            chunks.push((nchunk, i * T));
        }
    } else if T == 3 {
        for i in 0..total_chunks {
            let nchunk = &doc[i * T..i * T + T];
            chunks.push((nchunk, i * T));
        }
    } else if T == 4 {
        for i in 0..total_chunks {
            let nchunk = &doc[i * T..i * T + T];
            chunks.push((nchunk, i * T));
        }
    }

    chunks.sort();
    chunks
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut dna = "TAGTATTCTCTTACCTTCTGGATATTAGGAACAATATCATAAGAAGGTTGTACACCCTTTGCGATATTGGGAGTAATATCGTCCTGTATTCCCCTGGATAT".to_owned();
    const N: usize = 2usize;
    if dna.len() % N != 0 {
        dna.push_str(&"$".repeat(dna.len() % N));
    }
    let mut group = c.benchmark_group("NChunks");
    group.bench_with_input(BenchmarkId::new("Nums", 1), dna.as_bytes(), |b, _| {
        b.iter(|| nchunks_num::<N>(black_box(dna.as_bytes())))
    });

    group.bench_with_input(BenchmarkId::new("Chars", 1), dna.as_bytes(), |b, _| {
        b.iter(|| nchunks_chars::<N>(black_box(dna.as_bytes())))
    });
    group.finish();

    const N_3: usize = 3usize;
    if dna.len() % N_3 != 0 {
        dna.push_str(&"$".repeat(dna.len() % N_3));
    }

    let mut g2 = c.benchmark_group("NChunks3");
    g2.bench_with_input(BenchmarkId::new("Nums3", 1), dna.as_bytes(), |b, _| {
        b.iter(|| nchunks_num::<N_3>(black_box(dna.as_bytes())))
    });

    g2.bench_with_input(BenchmarkId::new("Chars3", 1), dna.as_bytes(), |b, _| {
        b.iter(|| nchunks_chars::<N_3>(black_box(dna.as_bytes())))
    });
    g2.finish();

    const N_4: usize = 4usize;
    if dna.len() % N_4 != 0 {
        dna.push_str(&"$".repeat(dna.len() % N_4));
    }

    let mut g3 = c.benchmark_group("NChunks4");
    g3.bench_with_input(BenchmarkId::new("Nums3", 1), dna.as_bytes(), |b, _| {
        b.iter(|| nchunks_num::<N_4>(black_box(dna.as_bytes())))
    });

    g3.bench_with_input(BenchmarkId::new("Chars3", 1), dna.as_bytes(), |b, _| {
        b.iter(|| nchunks_chars::<N_4>(black_box(dna.as_bytes())))
    });
    g3.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
