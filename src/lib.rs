#![feature(test)]

extern crate simdty;
extern crate llvmint;
extern crate test;

use llvmint::x86;
use simdty::i32x4;

// shift-add
macro_rules! sa {
($out:ident, $i:ident, $mul:expr, $( $( $idx:expr );* ),* ) => { {
    let mut outbuf = i32x4(0, 0, 0, 0);
    let mut destidx = 0;
    let mut shift;
    let mut srcidx;
    unsafe {
        $({
            $(
                shift = ($mul * $idx) % 32;
                srcidx = $idx;
                outbuf |= x86::sse2_pslli_d(*($i.get_unchecked(srcidx)), shift);
            )*
            *($out.get_unchecked_mut(destidx)) = outbuf;
            outbuf = x86::sse2_psrli_d(*($i.get_unchecked(srcidx)), 32 - shift);

            destidx += 1;
        })*
    }
    let _ = outbuf;
    let _ = destidx;
} }
}

// shift-add-mask
macro_rules! sam {
($out:ident, $i:ident, $mul:expr, $( $( $idx:expr );* ),* ) => { {
    let mask = i32x4((1<<$mul)-1, (1<<$mul)-1, (1<<$mul)-1, (1<<$mul)-1);
    let mut outbuf = i32x4(0, 0, 0, 0);
    let mut destidx = 0;
    let mut shift;
    let mut srcidx;
    unsafe {
        $({
            $(
                shift = ($mul * $idx) % 32;
                srcidx = $idx;
                outbuf |= x86::sse2_pslli_d(*($i.get_unchecked(srcidx)) & mask, shift);
            )*
            *($out.get_unchecked_mut(destidx)) = outbuf;
            outbuf = x86::sse2_psrli_d(*($i.get_unchecked(srcidx)) & mask, 32 - shift);
            destidx += 1;
        })*
    }
    let _ = outbuf;
    let _ = destidx;
} }
}

// shift-mask-store
macro_rules! sms {
($out:ident, $i:ident, $mul:expr, $( $( $idx:expr );* ),* ) => { {
    let mask = i32x4((1<<$mul)-1, (1<<$mul)-1, (1<<$mul)-1, (1<<$mul)-1);
    let mut srcidx = 0;
    let mut shift;
    let mut destidx;
    unsafe {
        $({
            $(
                shift = ($mul * $idx) % 32;
                destidx = $idx;
                *($out.get_unchecked_mut(destidx)) =
                    x86::sse2_psrli_d(*($i.get_unchecked(srcidx)), shift) & mask;
            )*

            srcidx += 1;
            let pullbits = $mul - (32 - shift);
            if pullbits > 0 && destidx < 32 {
                *($out.get_unchecked_mut(destidx)) |=
                    x86::sse2_pslli_d(*($i.get_unchecked(srcidx)), $mul - pullbits) & mask;
            }
        })*
    }
    let _ = srcidx;
} }
}


#[derive(Clone, Debug)]
pub enum Error {
    ShortInput,
    ShortOutput,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn pack_nomask(output: &mut [i32x4], input: &[i32x4], bits: i32) -> Result<()> {
    if input.len() < 32usize {
        return Err(Error::ShortInput);
    }
    if output.len() < bits as usize {
        return Err(Error::ShortOutput);
    }
    pack_nomask_bits(output, input, bits);
    Ok(())
}

pub fn pack(output: &mut [i32x4], input: &[i32x4], bits: i32) -> Result<()> {
    if input.len() < 32usize {
        return Err(Error::ShortInput);
    }
    if output.len() < bits as usize {
        return Err(Error::ShortOutput);
    }
    pack_bits(output, input, bits);
    Ok(())
}

pub fn unpack(output: &mut [i32x4], input: &[i32x4], bits: i32) -> Result<()> {
    if input.len() < bits as usize {
        return Err(Error::ShortInput);
    }
    if output.len() < 32usize {
        return Err(Error::ShortOutput);
    }
    unpack_bits(output, input, bits);
    Ok(())
}

// GENERATED CODE START
fn pack_nomask_1bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 1, 0;1;2;3;4;5;6;7;8;9;10;11;12;13;14;15;16;17;18;19;20;21;22;23;24;25;26;27;28;29;30;31); }
fn pack_nomask_2bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 2, 0;1;2;3;4;5;6;7;8;9;10;11;12;13;14;15,16;17;18;19;20;21;22;23;24;25;26;27;28;29;30;31); }
fn pack_nomask_3bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 3, 0;1;2;3;4;5;6;7;8;9;10,11;12;13;14;15;16;17;18;19;20;21,22;23;24;25;26;27;28;29;30;31); }
fn pack_nomask_4bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 4, 0;1;2;3;4;5;6;7,8;9;10;11;12;13;14;15,16;17;18;19;20;21;22;23,24;25;26;27;28;29;30;31); }
fn pack_nomask_5bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 5, 0;1;2;3;4;5;6,7;8;9;10;11;12,13;14;15;16;17;18;19,20;21;22;23;24;25,26;27;28;29;30;31); }
fn pack_nomask_6bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 6, 0;1;2;3;4;5,6;7;8;9;10,11;12;13;14;15,16;17;18;19;20;21,22;23;24;25;26,27;28;29;30;31); }
fn pack_nomask_7bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 7, 0;1;2;3;4,5;6;7;8;9,10;11;12;13,14;15;16;17;18,19;20;21;22,23;24;25;26;27,28;29;30;31); }
fn pack_nomask_8bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 8, 0;1;2;3,4;5;6;7,8;9;10;11,12;13;14;15,16;17;18;19,20;21;22;23,24;25;26;27,28;29;30;31); }
fn pack_nomask_9bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 9, 0;1;2;3,4;5;6;7,8;9;10,11;12;13;14,15;16;17,18;19;20;21,22;23;24,25;26;27;28,29;30;31); }
fn pack_nomask_10bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 10, 0;1;2;3,4;5;6,7;8;9,10;11;12,13;14;15,16;17;18;19,20;21;22,23;24;25,26;27;28,29;30;31); }
fn pack_nomask_11bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 11, 0;1;2,3;4;5,6;7;8,9;10;11,12;13;14,15;16;17,18;19;20,21;22;23,24;25;26,27;28;29,30;31); }
fn pack_nomask_12bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 12, 0;1;2,3;4;5,6;7,8;9;10,11;12;13,14;15,16;17;18,19;20;21,22;23,24;25;26,27;28;29,30;31); }
fn pack_nomask_13bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 13, 0;1;2,3;4,5;6;7,8;9,10;11;12,13;14,15;16;17,18;19,20;21;22,23;24,25;26;27,28;29,30;31); }
fn pack_nomask_14bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 14, 0;1;2,3;4,5;6,7;8;9,10;11,12;13,14;15,16;17;18,19;20,21;22,23;24;25,26;27,28;29,30;31); }
fn pack_nomask_15bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 15, 0;1;2,3;4,5;6,7;8,9;10,11;12,13;14,15;16;17,18;19,20;21,22;23,24;25,26;27,28;29,30;31); }
fn pack_nomask_16bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 16, 0;1,2;3,4;5,6;7,8;9,10;11,12;13,14;15,16;17,18;19,20;21,22;23,24;25,26;27,28;29,30;31); }
fn pack_nomask_17bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 17, 0;1,2;3,4;5,6;7,8;9,10;11,12;13,14;15,16,17;18,19;20,21;22,23;24,25;26,27;28,29;30,31); }
fn pack_nomask_18bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 18, 0;1,2;3,4;5,6;7,8,9;10,11;12,13;14,15,16;17,18;19,20;21,22;23,24,25;26,27;28,29;30,31); }
fn pack_nomask_19bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 19, 0;1,2;3,4;5,6,7;8,9;10,11,12;13,14;15,16,17;18,19;20,21,22;23,24;25,26,27;28,29;30,31); }
fn pack_nomask_20bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 20, 0;1,2;3,4,5;6,7,8;9,10;11,12,13;14,15,16;17,18;19,20,21;22,23,24;25,26;27,28,29;30,31); }
fn pack_nomask_21bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 21, 0;1,2;3,4,5;6,7,8;9,10,11;12,13,14;15,16,17;18,19,20;21,22,23;24,25,26;27,28,29;30,31); }
fn pack_nomask_22bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 22, 0;1,2,3;4,5,6;7,8,9;10,11,12;13,14,15,16;17,18,19;20,21,22;23,24,25;26,27,28;29,30,31); }
fn pack_nomask_23bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 23, 0;1,2,3;4,5,6,7;8,9,10;11,12,13,14;15,16,17;18,19,20,21;22,23,24;25,26,27,28;29,30,31); }
fn pack_nomask_24bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 24, 0;1,2,3,4;5,6,7,8;9,10,11,12;13,14,15,16;17,18,19,20;21,22,23,24;25,26,27,28;29,30,31); }
fn pack_nomask_25bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 25, 0;1,2,3,4;5,6,7,8,9;10,11,12,13;14,15,16,17,18;19,20,21,22;23,24,25,26,27;28,29,30,31); }
fn pack_nomask_26bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 26, 0;1,2,3,4,5;6,7,8,9,10;11,12,13,14,15,16;17,18,19,20,21;22,23,24,25,26;27,28,29,30,31); }
fn pack_nomask_27bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 27, 0;1,2,3,4,5,6;7,8,9,10,11,12;13,14,15,16,17,18,19;20,21,22,23,24,25;26,27,28,29,30,31); }
fn pack_nomask_28bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 28, 0;1,2,3,4,5,6,7,8;9,10,11,12,13,14,15,16;17,18,19,20,21,22,23,24;25,26,27,28,29,30,31); }
fn pack_nomask_29bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 29, 0;1,2,3,4,5,6,7,8,9,10;11,12,13,14,15,16,17,18,19,20,21;22,23,24,25,26,27,28,29,30,31); }
fn pack_nomask_30bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 30, 0;1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16;17,18,19,20,21,22,23,24,25,26,27,28,29,30,31); }
fn pack_nomask_31bit(output: &mut [i32x4], input: &[i32x4]) {
    sa!(output, input, 31, 0;1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31); }
fn pack_nomask_bits(output: &mut [i32x4], input: &[i32x4], bits: i32) {
    match bits {
        0 => (),
        1 => pack_nomask_1bit(output, input),
        2 => pack_nomask_2bit(output, input),
        3 => pack_nomask_3bit(output, input),
        4 => pack_nomask_4bit(output, input),
        5 => pack_nomask_5bit(output, input),
        6 => pack_nomask_6bit(output, input),
        7 => pack_nomask_7bit(output, input),
        8 => pack_nomask_8bit(output, input),
        9 => pack_nomask_9bit(output, input),
        10 => pack_nomask_10bit(output, input),
        11 => pack_nomask_11bit(output, input),
        12 => pack_nomask_12bit(output, input),
        13 => pack_nomask_13bit(output, input),
        14 => pack_nomask_14bit(output, input),
        15 => pack_nomask_15bit(output, input),
        16 => pack_nomask_16bit(output, input),
        17 => pack_nomask_17bit(output, input),
        18 => pack_nomask_18bit(output, input),
        19 => pack_nomask_19bit(output, input),
        20 => pack_nomask_20bit(output, input),
        21 => pack_nomask_21bit(output, input),
        22 => pack_nomask_22bit(output, input),
        23 => pack_nomask_23bit(output, input),
        24 => pack_nomask_24bit(output, input),
        25 => pack_nomask_25bit(output, input),
        26 => pack_nomask_26bit(output, input),
        27 => pack_nomask_27bit(output, input),
        28 => pack_nomask_28bit(output, input),
        29 => pack_nomask_29bit(output, input),
        30 => pack_nomask_30bit(output, input),
        31 => pack_nomask_31bit(output, input),
        _ => panic!("!invalid bit length")
    }
}

fn pack_mask_1bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 1, 0;1;2;3;4;5;6;7;8;9;10;11;12;13;14;15;16;17;18;19;20;21;22;23;24;25;26;27;28;29;30;31); }
fn pack_mask_2bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 2, 0;1;2;3;4;5;6;7;8;9;10;11;12;13;14;15,16;17;18;19;20;21;22;23;24;25;26;27;28;29;30;31); }
fn pack_mask_3bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 3, 0;1;2;3;4;5;6;7;8;9;10,11;12;13;14;15;16;17;18;19;20;21,22;23;24;25;26;27;28;29;30;31); }
fn pack_mask_4bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 4, 0;1;2;3;4;5;6;7,8;9;10;11;12;13;14;15,16;17;18;19;20;21;22;23,24;25;26;27;28;29;30;31); }
fn pack_mask_5bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 5, 0;1;2;3;4;5;6,7;8;9;10;11;12,13;14;15;16;17;18;19,20;21;22;23;24;25,26;27;28;29;30;31); }
fn pack_mask_6bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 6, 0;1;2;3;4;5,6;7;8;9;10,11;12;13;14;15,16;17;18;19;20;21,22;23;24;25;26,27;28;29;30;31); }
fn pack_mask_7bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 7, 0;1;2;3;4,5;6;7;8;9,10;11;12;13,14;15;16;17;18,19;20;21;22,23;24;25;26;27,28;29;30;31); }
fn pack_mask_8bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 8, 0;1;2;3,4;5;6;7,8;9;10;11,12;13;14;15,16;17;18;19,20;21;22;23,24;25;26;27,28;29;30;31); }
fn pack_mask_9bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 9, 0;1;2;3,4;5;6;7,8;9;10,11;12;13;14,15;16;17,18;19;20;21,22;23;24,25;26;27;28,29;30;31); }
fn pack_mask_10bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 10, 0;1;2;3,4;5;6,7;8;9,10;11;12,13;14;15,16;17;18;19,20;21;22,23;24;25,26;27;28,29;30;31); }
fn pack_mask_11bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 11, 0;1;2,3;4;5,6;7;8,9;10;11,12;13;14,15;16;17,18;19;20,21;22;23,24;25;26,27;28;29,30;31); }
fn pack_mask_12bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 12, 0;1;2,3;4;5,6;7,8;9;10,11;12;13,14;15,16;17;18,19;20;21,22;23,24;25;26,27;28;29,30;31); }
fn pack_mask_13bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 13, 0;1;2,3;4,5;6;7,8;9,10;11;12,13;14,15;16;17,18;19,20;21;22,23;24,25;26;27,28;29,30;31); }
fn pack_mask_14bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 14, 0;1;2,3;4,5;6,7;8;9,10;11,12;13,14;15,16;17;18,19;20,21;22,23;24;25,26;27,28;29,30;31); }
fn pack_mask_15bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 15, 0;1;2,3;4,5;6,7;8,9;10,11;12,13;14,15;16;17,18;19,20;21,22;23,24;25,26;27,28;29,30;31); }
fn pack_mask_16bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 16, 0;1,2;3,4;5,6;7,8;9,10;11,12;13,14;15,16;17,18;19,20;21,22;23,24;25,26;27,28;29,30;31); }
fn pack_mask_17bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 17, 0;1,2;3,4;5,6;7,8;9,10;11,12;13,14;15,16,17;18,19;20,21;22,23;24,25;26,27;28,29;30,31); }
fn pack_mask_18bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 18, 0;1,2;3,4;5,6;7,8,9;10,11;12,13;14,15,16;17,18;19,20;21,22;23,24,25;26,27;28,29;30,31); }
fn pack_mask_19bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 19, 0;1,2;3,4;5,6,7;8,9;10,11,12;13,14;15,16,17;18,19;20,21,22;23,24;25,26,27;28,29;30,31); }
fn pack_mask_20bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 20, 0;1,2;3,4,5;6,7,8;9,10;11,12,13;14,15,16;17,18;19,20,21;22,23,24;25,26;27,28,29;30,31); }
fn pack_mask_21bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 21, 0;1,2;3,4,5;6,7,8;9,10,11;12,13,14;15,16,17;18,19,20;21,22,23;24,25,26;27,28,29;30,31); }
fn pack_mask_22bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 22, 0;1,2,3;4,5,6;7,8,9;10,11,12;13,14,15,16;17,18,19;20,21,22;23,24,25;26,27,28;29,30,31); }
fn pack_mask_23bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 23, 0;1,2,3;4,5,6,7;8,9,10;11,12,13,14;15,16,17;18,19,20,21;22,23,24;25,26,27,28;29,30,31); }
fn pack_mask_24bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 24, 0;1,2,3,4;5,6,7,8;9,10,11,12;13,14,15,16;17,18,19,20;21,22,23,24;25,26,27,28;29,30,31); }
fn pack_mask_25bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 25, 0;1,2,3,4;5,6,7,8,9;10,11,12,13;14,15,16,17,18;19,20,21,22;23,24,25,26,27;28,29,30,31); }
fn pack_mask_26bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 26, 0;1,2,3,4,5;6,7,8,9,10;11,12,13,14,15,16;17,18,19,20,21;22,23,24,25,26;27,28,29,30,31); }
fn pack_mask_27bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 27, 0;1,2,3,4,5,6;7,8,9,10,11,12;13,14,15,16,17,18,19;20,21,22,23,24,25;26,27,28,29,30,31); }
fn pack_mask_28bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 28, 0;1,2,3,4,5,6,7,8;9,10,11,12,13,14,15,16;17,18,19,20,21,22,23,24;25,26,27,28,29,30,31); }
fn pack_mask_29bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 29, 0;1,2,3,4,5,6,7,8,9,10;11,12,13,14,15,16,17,18,19,20,21;22,23,24,25,26,27,28,29,30,31); }
fn pack_mask_30bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 30, 0;1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16;17,18,19,20,21,22,23,24,25,26,27,28,29,30,31); }
fn pack_mask_31bit(output: &mut [i32x4], input: &[i32x4]) {
    sam!(output, input, 31, 0;1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31); }
fn pack_bits(output: &mut [i32x4], input: &[i32x4], bits: i32) {
    match bits {
        0 => (),
        1 => pack_mask_1bit(output, input),
        2 => pack_mask_2bit(output, input),
        3 => pack_mask_3bit(output, input),
        4 => pack_mask_4bit(output, input),
        5 => pack_mask_5bit(output, input),
        6 => pack_mask_6bit(output, input),
        7 => pack_mask_7bit(output, input),
        8 => pack_mask_8bit(output, input),
        9 => pack_mask_9bit(output, input),
        10 => pack_mask_10bit(output, input),
        11 => pack_mask_11bit(output, input),
        12 => pack_mask_12bit(output, input),
        13 => pack_mask_13bit(output, input),
        14 => pack_mask_14bit(output, input),
        15 => pack_mask_15bit(output, input),
        16 => pack_mask_16bit(output, input),
        17 => pack_mask_17bit(output, input),
        18 => pack_mask_18bit(output, input),
        19 => pack_mask_19bit(output, input),
        20 => pack_mask_20bit(output, input),
        21 => pack_mask_21bit(output, input),
        22 => pack_mask_22bit(output, input),
        23 => pack_mask_23bit(output, input),
        24 => pack_mask_24bit(output, input),
        25 => pack_mask_25bit(output, input),
        26 => pack_mask_26bit(output, input),
        27 => pack_mask_27bit(output, input),
        28 => pack_mask_28bit(output, input),
        29 => pack_mask_29bit(output, input),
        30 => pack_mask_30bit(output, input),
        31 => pack_mask_31bit(output, input),
        _ => panic!("!invalid bit length")
    }
}

fn unpack_1bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 1, 0;1;2;3;4;5;6;7;8;9;10;11;12;13;14;15;16;17;18;19;20;21;22;23;24;25;26;27;28;29;30;31); }
fn unpack_2bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 2, 0;1;2;3;4;5;6;7;8;9;10;11;12;13;14;15,16;17;18;19;20;21;22;23;24;25;26;27;28;29;30;31); }
fn unpack_3bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 3, 0;1;2;3;4;5;6;7;8;9;10,11;12;13;14;15;16;17;18;19;20;21,22;23;24;25;26;27;28;29;30;31); }
fn unpack_4bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 4, 0;1;2;3;4;5;6;7,8;9;10;11;12;13;14;15,16;17;18;19;20;21;22;23,24;25;26;27;28;29;30;31); }
fn unpack_5bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 5, 0;1;2;3;4;5;6,7;8;9;10;11;12,13;14;15;16;17;18;19,20;21;22;23;24;25,26;27;28;29;30;31); }
fn unpack_6bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 6, 0;1;2;3;4;5,6;7;8;9;10,11;12;13;14;15,16;17;18;19;20;21,22;23;24;25;26,27;28;29;30;31); }
fn unpack_7bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 7, 0;1;2;3;4,5;6;7;8;9,10;11;12;13,14;15;16;17;18,19;20;21;22,23;24;25;26;27,28;29;30;31); }
fn unpack_8bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 8, 0;1;2;3,4;5;6;7,8;9;10;11,12;13;14;15,16;17;18;19,20;21;22;23,24;25;26;27,28;29;30;31); }
fn unpack_9bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 9, 0;1;2;3,4;5;6;7,8;9;10,11;12;13;14,15;16;17,18;19;20;21,22;23;24,25;26;27;28,29;30;31); }
fn unpack_10bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 10, 0;1;2;3,4;5;6,7;8;9,10;11;12,13;14;15,16;17;18;19,20;21;22,23;24;25,26;27;28,29;30;31); }
fn unpack_11bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 11, 0;1;2,3;4;5,6;7;8,9;10;11,12;13;14,15;16;17,18;19;20,21;22;23,24;25;26,27;28;29,30;31); }
fn unpack_12bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 12, 0;1;2,3;4;5,6;7,8;9;10,11;12;13,14;15,16;17;18,19;20;21,22;23,24;25;26,27;28;29,30;31); }
fn unpack_13bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 13, 0;1;2,3;4,5;6;7,8;9,10;11;12,13;14,15;16;17,18;19,20;21;22,23;24,25;26;27,28;29,30;31); }
fn unpack_14bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 14, 0;1;2,3;4,5;6,7;8;9,10;11,12;13,14;15,16;17;18,19;20,21;22,23;24;25,26;27,28;29,30;31); }
fn unpack_15bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 15, 0;1;2,3;4,5;6,7;8,9;10,11;12,13;14,15;16;17,18;19,20;21,22;23,24;25,26;27,28;29,30;31); }
fn unpack_16bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 16, 0;1,2;3,4;5,6;7,8;9,10;11,12;13,14;15,16;17,18;19,20;21,22;23,24;25,26;27,28;29,30;31); }
fn unpack_17bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 17, 0;1,2;3,4;5,6;7,8;9,10;11,12;13,14;15,16,17;18,19;20,21;22,23;24,25;26,27;28,29;30,31); }
fn unpack_18bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 18, 0;1,2;3,4;5,6;7,8,9;10,11;12,13;14,15,16;17,18;19,20;21,22;23,24,25;26,27;28,29;30,31); }
fn unpack_19bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 19, 0;1,2;3,4;5,6,7;8,9;10,11,12;13,14;15,16,17;18,19;20,21,22;23,24;25,26,27;28,29;30,31); }
fn unpack_20bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 20, 0;1,2;3,4,5;6,7,8;9,10;11,12,13;14,15,16;17,18;19,20,21;22,23,24;25,26;27,28,29;30,31); }
fn unpack_21bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 21, 0;1,2;3,4,5;6,7,8;9,10,11;12,13,14;15,16,17;18,19,20;21,22,23;24,25,26;27,28,29;30,31); }
fn unpack_22bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 22, 0;1,2,3;4,5,6;7,8,9;10,11,12;13,14,15,16;17,18,19;20,21,22;23,24,25;26,27,28;29,30,31); }
fn unpack_23bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 23, 0;1,2,3;4,5,6,7;8,9,10;11,12,13,14;15,16,17;18,19,20,21;22,23,24;25,26,27,28;29,30,31); }
fn unpack_24bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 24, 0;1,2,3,4;5,6,7,8;9,10,11,12;13,14,15,16;17,18,19,20;21,22,23,24;25,26,27,28;29,30,31); }
fn unpack_25bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 25, 0;1,2,3,4;5,6,7,8,9;10,11,12,13;14,15,16,17,18;19,20,21,22;23,24,25,26,27;28,29,30,31); }
fn unpack_26bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 26, 0;1,2,3,4,5;6,7,8,9,10;11,12,13,14,15,16;17,18,19,20,21;22,23,24,25,26;27,28,29,30,31); }
fn unpack_27bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 27, 0;1,2,3,4,5,6;7,8,9,10,11,12;13,14,15,16,17,18,19;20,21,22,23,24,25;26,27,28,29,30,31); }
fn unpack_28bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 28, 0;1,2,3,4,5,6,7,8;9,10,11,12,13,14,15,16;17,18,19,20,21,22,23,24;25,26,27,28,29,30,31); }
fn unpack_29bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 29, 0;1,2,3,4,5,6,7,8,9,10;11,12,13,14,15,16,17,18,19,20,21;22,23,24,25,26,27,28,29,30,31); }
fn unpack_30bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 30, 0;1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16;17,18,19,20,21,22,23,24,25,26,27,28,29,30,31); }
fn unpack_31bit(output: &mut [i32x4], input: &[i32x4]) {
    sms!(output, input, 31, 0;1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31); }
fn unpack_bits(output: &mut [i32x4], input: &[i32x4], bits: i32) {
    match bits {
        0 => (),
        1 => unpack_1bit(output, input),
        2 => unpack_2bit(output, input),
        3 => unpack_3bit(output, input),
        4 => unpack_4bit(output, input),
        5 => unpack_5bit(output, input),
        6 => unpack_6bit(output, input),
        7 => unpack_7bit(output, input),
        8 => unpack_8bit(output, input),
        9 => unpack_9bit(output, input),
        10 => unpack_10bit(output, input),
        11 => unpack_11bit(output, input),
        12 => unpack_12bit(output, input),
        13 => unpack_13bit(output, input),
        14 => unpack_14bit(output, input),
        15 => unpack_15bit(output, input),
        16 => unpack_16bit(output, input),
        17 => unpack_17bit(output, input),
        18 => unpack_18bit(output, input),
        19 => unpack_19bit(output, input),
        20 => unpack_20bit(output, input),
        21 => unpack_21bit(output, input),
        22 => unpack_22bit(output, input),
        23 => unpack_23bit(output, input),
        24 => unpack_24bit(output, input),
        25 => unpack_25bit(output, input),
        26 => unpack_26bit(output, input),
        27 => unpack_27bit(output, input),
        28 => unpack_28bit(output, input),
        29 => unpack_29bit(output, input),
        30 => unpack_30bit(output, input),
        31 => unpack_31bit(output, input),
        _ => panic!("!invalid bit length")
    }
}
// GENERATED CODE END

pub fn equal(a: &[i32x4], b: &[i32x4]) {
    assert!(a.len() == 32 && b.len() == 32);
    for i in (0..32) {
        assert_eq!(a[i].0, b[i].0);
        assert_eq!(a[i].1, b[i].1);
        assert_eq!(a[i].2, b[i].2);
        assert_eq!(a[i].3, b[i].3);
    }
}

#[test]
fn test_pack1() {
    let input: [i32x4; 32] = [i32x4(1,0,1,0);32];

    let mut output = [i32x4(0, 0, 0, 0)];
    pack_nomask_1bit(&mut output, &input);
    assert_eq!(output[0].0, 0b11111111111111111111111111111111u32 as i32);
    assert_eq!(output[0].1, 0);
    assert_eq!(output[0].2, 0b11111111111111111111111111111111u32 as i32);
    assert_eq!(output[0].3, 0);

    let mut output2: [i32x4; 32] = [i32x4(0,0,0,0);32];
    unpack_1bit(&mut output2[..], &output);
    equal(&output2[..], &input[..]);
}

#[test]
fn test_pack2() {
    let input: [i32x4; 32] = [i32x4(1,0,1,0);32];

    let mut output = [i32x4(0, 0, 0, 0), i32x4(0, 0, 0, 0)];
    pack_nomask_2bit(&mut output, &input);
    assert_eq!(output[0].0, 0b01010101010101010101010101010101);
    assert_eq!(output[0].1, 0);
    assert_eq!(output[0].2, 0b01010101010101010101010101010101);
    assert_eq!(output[0].3, 0);

    assert_eq!(output[1].0, 0b01010101010101010101010101010101);
    assert_eq!(output[1].1, 0);
    assert_eq!(output[1].2, 0b01010101010101010101010101010101);
    assert_eq!(output[1].3, 0);

    let mut output2: [i32x4; 32] = [i32x4(0,0,0,0);32];
    unpack_2bit(&mut output2[..], &output);
    equal(&output2[..], &input[..]);
}

#[test]
fn test_pack3() {
    let input: [i32x4; 32] = [i32x4(1,0,1,0);32];

    let mut output = [i32x4(0, 0, 0, 0), i32x4(0, 0, 0, 0), i32x4(0, 0, 0, 0)];
    pack_nomask_3bit(&mut output, &input);
    assert_eq!(output[0].0, 0b01001001001001001001001001001001);
    assert_eq!(output[0].1, 0);
    assert_eq!(output[0].2, 0b01001001001001001001001001001001);
    assert_eq!(output[0].3, 0);

    assert_eq!(output[1].0, 0b10010010010010010010010010010010u32 as i32);
    assert_eq!(output[1].1, 0);
    assert_eq!(output[1].2, 0b10010010010010010010010010010010u32 as i32);
    assert_eq!(output[1].3, 0);

    assert_eq!(output[2].0, 0b00100100100100100100100100100100);
    assert_eq!(output[2].1, 0);
    assert_eq!(output[2].2, 0b00100100100100100100100100100100);
    assert_eq!(output[2].3, 0);

    let mut output2: [i32x4; 32] = [i32x4(0,0,0,0);32];
    unpack_3bit(&mut output2[..], &output);
    equal(&output2[..], &input[..]);
}

#[bench]
fn bench_pack1(b: &mut test::Bencher) {
    let mut input: [i32x4; 32] = [i32x4(1,0,1,0);32];
    let mut output = [i32x4(0, 0, 0, 0), i32x4(0, 0, 0, 0), i32x4(0, 0, 0, 0), i32x4(0, 0, 0, 0)];

    b.bytes = 4u64 * 32 * 2;
    b.iter(test::black_box(|| {
        pack_nomask(&mut output, &input, 3).unwrap();
        unpack(&mut input, &output, 3).unwrap();
    }))
}
