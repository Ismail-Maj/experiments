use itoap;
use std::io::{self, prelude::*};

const FIZZ: &[u8; 5] = b"FIZZ\n";

const BUZZ: &[u8; 5] =  b"BUZZ\n";

const FIZZBUZZ: &[u8; 9] = b"FIZZBUZZ\n";

const NEWLINE: &[u8; 1] = b"\n";

#[inline(always)]
fn serialize(value: usize, mut writer: &mut io::BufWriter<io::StdoutLock>) {
    itoap::write(&mut writer, value);
    writer.write(NEWLINE);
}

fn main() {
    let mut offset: usize = 0;

    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());

    loop {
        writer.write(FIZZBUZZ);

        serialize(offset + 1, &mut writer);

        serialize(offset + 2, &mut writer);

        writer.write(FIZZ);

        serialize(offset + 4, &mut writer);

        writer.write(BUZZ);

        writer.write(FIZZ);

        serialize(offset + 7, &mut writer);

        serialize(offset + 8, &mut writer);

        writer.write(FIZZ);

        writer.write(BUZZ);

        serialize(offset + 11, &mut writer);

        writer.write(FIZZ);

        serialize(offset + 13, &mut writer);

        serialize(offset + 14, &mut writer);

        offset += 15;
    }
}
