declare name "amp-sim";
declare version "0.1";
declare author "darkoverlordofdata";
declare description "Amplifier demo application.";
declare license 	"MIT";
declare copyright 	"(c)DarkOverlordOfData 2021";

import("stdfaust.lib");
// import("layout2.dsp");

// process = 
//     dm.cubicnl_demo : // distortion 
//     dm.wah4_demo <: // wah pedal
//     dm.phaser2_demo : // stereo phaser 
//     dm.compressor_demo : // stereo compressor
//     dm.zita_light; // stereo reverb

process = 
    dm.cubicnl_demo;// distortion 

