#!/usr/bin/env python

import sys
from math import ceil

def gen_packbit(l, fn, macro):
    s = "fn %s(output: &mut [i32x4], input: &[i32x4]) {\n" % (fn)
    s += "    %s!(output, input, %s, " % (macro, l)

    acc = []
    idx = 0
    for i in range(31):
        acc.append(str(i))
        nextidx = ((i + 1) * l) / 32;
        if(idx != nextidx):
            idx = nextidx;
            s += "%s," % (";".join(acc))
            acc = []
    s += "%s); }\n" % (";".join(acc + ["31"]))
    return s

def gen_pack(fn, fnbit, macro):
    s = ""
    for l in range(1, 32):
        s += gen_packbit(l, fnbit % l, macro)

    s += "fn %s(output: &mut [i32x4], input: &[i32x4], bits: i32) {\n" % (fn)
    s += "    match bits {\n"
    s += "        0 => (),\n"
    for l in range(1, 32):
        s += "        %s => %s(output, input),\n" % (l, fnbit % l)
    s += '        _ => panic!("!invalid bit length")\n'
    s += "    }\n}\n"
    return s

def gen_pack_unpack():
    s = "// GENERATED CODE START\n"
    s += gen_pack("pack_nomask_bits", "pack_nomask_%sbit", "sa") + "\n"
    s += gen_pack("pack_bits", "pack_mask_%sbit", "sam") + "\n"
    s += gen_pack("unpack_bits", "unpack_%sbit", "sms")
    s += "// GENERATED CODE END"
    return s

def gen_bench():
    s = "// GENERATED CODE START\n"
    for l in range(1, 32):
        s += "#[bench] fn bench_pack_nomask%02d(b: &mut test::Bencher) { bench_pack_nomask!(b, %s); }\n" % (l, l)
        s += "#[bench] fn bench_pack%02d(b: &mut test::Bencher) { bench_pack!(b, %s); }\n" % (l, l)
        s += "#[bench] fn bench_unpack%02d(b: &mut test::Bencher) { bench_unpack!(b, %s); }\n" % (l, l)
    s += "// GENERATED CODE END"
    return s

if  __name__ == "__main__":
    if len(sys.argv) == 2 and sys.argv[1] == "bench":
        print gen_bench()
    else:
        print gen_pack_unpack()

