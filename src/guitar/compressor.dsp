declare name "amp-sim";
declare version "0.1";
declare author "darkoverlordofdata";
declare description "Amplifier demo application.";
declare license 	"MIT";
declare copyright 	"(c)DarkOverlordOfData 2021";

import("stdfaust.lib");
// import("layout2.dsp");


process = 
    dm.compressor_demo ; // stereo compressor
