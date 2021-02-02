declare name "amp-sim";
declare version "0.1";
declare author "darkoverlordofdata";
declare description "Amplifier demo application.";
declare license 	"MIT";
declare copyright 	"(c)DarkOverlordOfData 2021";

import("stdfaust.lib");
import("layout2.dsp");

process = _,_ : +
	: component("amplifier.dsp")
	: component("flanger.dsp")
	: component("chorus.dsp")
	: component("phaser.dsp")
	: component("freeverb.dsp");
