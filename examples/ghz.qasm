// Code generated by NVIDIA's nvq++ compiler
OPENQASM 2.0;

include "qelib1.inc";


qreg var0[5];
h var0[0];
cx var0[0], var0[1];
cx var0[1], var0[2];
cx var0[2], var0[3];
cx var0[3], var0[4];
creg var3[5];
measure var0 -> var3;