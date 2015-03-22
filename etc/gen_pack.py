#!/usr/bin/env python

import sys
from math import ceil

def print_packbit(l, fn, macro):
    s = "fn %s(output: &mut [i32x4], input: &[i32x4]) {\n" % (fn)
    s += "    %s!(output, input, %s, " % (macro, l)

    acc = []
    idx = 0
    for i in range(31):
        acc.append(i)
        nextidx = ((i + 1) * l) / 32;
        if(idx != nextidx):
            idx = nextidx;
            s += "%s, " % (acc)
            acc = []
    s += "%s);\n}\n" % (acc + [31])
    return s

def print_pack(fn, fnbit, macro):
    s = ""
    for l in range(1, 32):
        s += print_packbit(l, fnbit % l, macro)

    s += "pub fn %s(output: &mut [i32x4], input: &[i32x4], bits: i32) {\n" % (fn)
    s += "    match bits {\n"
    for l in range(1, 32):
        s += "        %s => %s(output, input),\n" % (l, fnbit % l)
    s += '        _ => panic!("!invalid bit length")\n'
    s += "    }\n}\n"
    return s

print print_pack("pack_nomask", "pack_nomask_%sbit", "sa")
print print_pack("pack", "pack_mask_%sbit", "sam")
print print_pack("unpack", "unpack_%sbit", "sam")
