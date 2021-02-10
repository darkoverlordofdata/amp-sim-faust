/************************************************************************
************************************************************************
    FAUST Architecture File
    Copyright (C) 2017-2020 GRAME, Centre National de Creation Musicale
    ---------------------------------------------------------------------

    This is sample code. This file is provided as an example of minimal
    FAUST architecture file. Redistribution and use in source and binary
    forms, with or without modification, in part or in full are permitted.
    In particular you can create a derived work of this FAUST architecture
    and distribute that work under terms of your choice.

    This sample code is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
************************************************************************
************************************************************************/

#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

//! Faust JACK architecture file
extern crate jack;
use jack::prelude as j;
use std::io;
extern crate libm;

type F32 = f32;
type F64 = f64;

#[derive(Copy, Clone)]
pub struct ParamIndex(pub i32);

pub trait FaustDsp {
    type T;

    fn new() -> Self where Self: Sized;
    fn metadata(&self, m: &mut dyn Meta);
    fn get_sample_rate(&self) -> i32;
    fn get_num_inputs(&self) -> i32;
    fn get_num_outputs(&self) -> i32;
    fn get_input_rate(&self, channel: i32) -> i32;
    fn get_output_rate(&self, channel: i32) -> i32;
    fn class_init(sample_rate: i32) where Self: Sized;
    fn instance_reset_params(&mut self);
    fn instance_clear(&mut self);
    fn instance_constants(&mut self, sample_rate: i32);
    fn instance_init(&mut self, sample_rate: i32);
    fn init(&mut self, sample_rate: i32);
    fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>);
    fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) where Self: Sized;
    fn get_param(&self, param: ParamIndex) -> Option<Self::T>;
    fn set_param(&mut self, param: ParamIndex, value: Self::T);
    fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]);
}

pub trait Meta {
    // -- metadata declarations
    fn declare(&mut self, key: &str, value: &str);
}

pub trait UI<T> {
    // -- widget's layouts
    fn open_tab_box(&mut self, label: &str);
    fn open_horizontal_box(&mut self, label: &str);
    fn open_vertical_box(&mut self, label: &str);
    fn close_box(&mut self);

    // -- active widgets
    fn add_button(&mut self, label: &str, param: ParamIndex);
    fn add_check_button(&mut self, label: &str, param: ParamIndex);
    fn add_vertical_slider(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);
    fn add_horizontal_slider(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);
    fn add_num_entry(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);

    // -- passive widgets
    fn add_horizontal_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);
    fn add_vertical_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);

    // -- metadata declarations
    fn declare(&mut self, param: Option<ParamIndex>, key: &str, value: &str);
}



pub struct mydspSIG0 {
	iRec33: [i32;2],
}

impl mydspSIG0 {
	
	fn get_num_inputsmydspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsmydspSIG0(&self) -> i32 {
		return 1;
	}
	fn get_input_ratemydspSIG0(&self, channel: i32) -> i32 {
		let mut rate: i32;
		match (channel) {
			_ => {
				rate = -1;
			},
		} 
		return rate;
	}
	fn get_output_ratemydspSIG0(&self, channel: i32) -> i32 {
		let mut rate: i32;
		match (channel) {
			0 => {
				rate = 0;
			},
			_ => {
				rate = -1;
			},
		} 
		return rate;
	}
	
	fn instance_initmydspSIG0(&mut self, sample_rate: i32) {
		let mut l28: i32 = 0;
		loop {
			self.iRec33[l28 as usize] = 0;
			l28 = (l28 + 1);
			if (l28 < 2) { continue; } else { break; }
		}
	}
	
	fn fillmydspSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i in 0..count {
			self.iRec33[0] = (self.iRec33[1] + 1);
			table[i as usize] = F32::cos((9.58738019e-05 * ((self.iRec33[0] + -1) as F32)));
			self.iRec33[1] = self.iRec33[0];
		}
	}

}


pub fn newmydspSIG0() -> mydspSIG0 { 
	mydspSIG0 {
		iRec33: [0;2],
	}
}

pub struct mydspSIG1 {
	iRec36: [i32;2],
}

impl mydspSIG1 {
	
	fn get_num_inputsmydspSIG1(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsmydspSIG1(&self) -> i32 {
		return 1;
	}
	fn get_input_ratemydspSIG1(&self, channel: i32) -> i32 {
		let mut rate: i32;
		match (channel) {
			_ => {
				rate = -1;
			},
		} 
		return rate;
	}
	fn get_output_ratemydspSIG1(&self, channel: i32) -> i32 {
		let mut rate: i32;
		match (channel) {
			0 => {
				rate = 0;
			},
			_ => {
				rate = -1;
			},
		} 
		return rate;
	}
	
	fn instance_initmydspSIG1(&mut self, sample_rate: i32) {
		let mut l31: i32 = 0;
		loop {
			self.iRec36[l31 as usize] = 0;
			l31 = (l31 + 1);
			if (l31 < 2) { continue; } else { break; }
		}
	}
	
	fn fillmydspSIG1(&mut self, count: i32, table: &mut[F32]) {
		for i in 0..count {
			self.iRec36[0] = (self.iRec36[1] + 1);
			table[i as usize] = F32::sin((9.58738019e-05 * ((self.iRec36[0] + -1) as F32)));
			self.iRec36[1] = self.iRec36[0];
		}
	}

}


pub fn newmydspSIG1() -> mydspSIG1 { 
	mydspSIG1 {
		iRec36: [0;2],
	}
}
fn mydsp_faustpower2_f(value: F32) -> F32 {
	return (value * value);
}
static mut ftbl0mydspSIG0: [F32;65536] = [0.0;65536];
static mut ftbl1mydspSIG1: [F32;65536] = [0.0;65536];
fn mydsp_faustpower3_f(value: F32) -> F32 {
	return ((value * value) * value);
}
fn mydsp_faustpower4_f(value: F32) -> F32 {
	return (((value * value) * value) * value);
}
pub struct mydsp {
	iVec0: [i32;2],
	fVslider0: F32,
	fVslider1: F32,
	fVslider2: F32,
	fVslider3: F32,
	fRec9: [F32;2],
	fCheckbox0: F32,
	fHslider0: F32,
	fVslider4: F32,
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fConst2: F32,
	fVslider5: F32,
	fRec10: [F32;2],
	fVslider6: F32,
	fVslider7: F32,
	fConst3: F32,
	fVslider8: F32,
	fRec12: [F32;2],
	fRec11: [F32;2],
	fConst4: F32,
	fRec13: [F32;2],
	fRec14: [F32;2],
	fConst5: F32,
	fCheckbox1: F32,
	fRec15: [F32;2],
	fHslider1: F32,
	fRec19: [F32;2],
	fHslider2: F32,
	fRec20: [F32;2],
	fConst6: F32,
	fConst7: F32,
	fConst8: F32,
	fConst9: F32,
	fConst10: F32,
	fConst11: F32,
	fHslider3: F32,
	fConst12: F32,
	fRec22: [F32;3],
	fConst13: F32,
	fConst14: F32,
	fRec21: [F32;3],
	fHslider4: F32,
	fRec23: [F32;2],
	fConst15: F32,
	fConst16: F32,
	fRec24: [F32;2],
	fHslider5: F32,
	fRec25: [F32;2],
	fHslider6: F32,
	fRec26: [F32;2],
	fVec1: [F32;2],
	fRec18: [F32;2],
	fRec17: [F32;2],
	fRec16: [F32;2],
	fHslider7: F32,
	fRec27: [F32;2],
	fVslider9: F32,
	fRec29: [F32;2],
	IOTA: i32,
	fVec2: [F32;4096],
	fVslider10: F32,
	fRec28: [F32;2],
	fVslider11: F32,
	fVslider12: F32,
	fRec30: [F32;2],
	fVec3: [F32;16384],
	fVslider13: F32,
	fRec31: [F32;2],
	fVslider14: F32,
	fRec32: [F32;2],
	fConst17: F32,
	fVslider15: F32,
	fRec35: [F32;2],
	fRec34: [F32;2],
	fRec37: [F32;2],
	fConst18: F32,
	fRec38: [F32;2],
	fCheckbox2: F32,
	fHslider8: F32,
	fHslider9: F32,
	fHslider10: F32,
	fHslider11: F32,
	fHslider12: F32,
	fHslider13: F32,
	fHslider14: F32,
	fRec44: [F32;2],
	fRec45: [F32;2],
	fRec43: [F32;3],
	fRec42: [F32;3],
	fRec41: [F32;3],
	fRec40: [F32;3],
	fRec39: [F32;2],
	fCheckbox3: F32,
	fConst19: F32,
	fRec46: [F32;2],
	fConst20: F32,
	fRec47: [F32;2],
	fConst21: F32,
	fRec48: [F32;2],
	fConst22: F32,
	fRec49: [F32;2],
	fRec54: [F32;3],
	fRec53: [F32;3],
	fRec52: [F32;3],
	fRec51: [F32;3],
	fRec50: [F32;2],
	fVec4: [F32;2048],
	fRec8: [F32;2],
	fRec56: [F32;2],
	fVec5: [F32;2048],
	fRec55: [F32;2],
	fRec58: [F32;2],
	fVec6: [F32;2048],
	fRec57: [F32;2],
	fRec60: [F32;2],
	fVec7: [F32;2048],
	fRec59: [F32;2],
	fRec62: [F32;2],
	fVec8: [F32;2048],
	fRec61: [F32;2],
	fRec64: [F32;2],
	fVec9: [F32;2048],
	fRec63: [F32;2],
	fRec66: [F32;2],
	fVec10: [F32;2048],
	fRec65: [F32;2],
	fRec68: [F32;2],
	fVec11: [F32;2048],
	fRec67: [F32;2],
	fVec12: [F32;1024],
	fRec6: [F32;2],
	fVec13: [F32;512],
	fRec4: [F32;2],
	fVec14: [F32;512],
	fRec2: [F32;2],
	fVec15: [F32;256],
	fRec0: [F32;2],
}

impl FaustDsp for mydsp {
	type T = F32;
		
	fn new() -> mydsp { 
		mydsp {
			iVec0: [0;2],
			fVslider0: 0.0,
			fVslider1: 0.0,
			fVslider2: 0.0,
			fVslider3: 0.0,
			fRec9: [0.0;2],
			fCheckbox0: 0.0,
			fHslider0: 0.0,
			fVslider4: 0.0,
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fConst2: 0.0,
			fVslider5: 0.0,
			fRec10: [0.0;2],
			fVslider6: 0.0,
			fVslider7: 0.0,
			fConst3: 0.0,
			fVslider8: 0.0,
			fRec12: [0.0;2],
			fRec11: [0.0;2],
			fConst4: 0.0,
			fRec13: [0.0;2],
			fRec14: [0.0;2],
			fConst5: 0.0,
			fCheckbox1: 0.0,
			fRec15: [0.0;2],
			fHslider1: 0.0,
			fRec19: [0.0;2],
			fHslider2: 0.0,
			fRec20: [0.0;2],
			fConst6: 0.0,
			fConst7: 0.0,
			fConst8: 0.0,
			fConst9: 0.0,
			fConst10: 0.0,
			fConst11: 0.0,
			fHslider3: 0.0,
			fConst12: 0.0,
			fRec22: [0.0;3],
			fConst13: 0.0,
			fConst14: 0.0,
			fRec21: [0.0;3],
			fHslider4: 0.0,
			fRec23: [0.0;2],
			fConst15: 0.0,
			fConst16: 0.0,
			fRec24: [0.0;2],
			fHslider5: 0.0,
			fRec25: [0.0;2],
			fHslider6: 0.0,
			fRec26: [0.0;2],
			fVec1: [0.0;2],
			fRec18: [0.0;2],
			fRec17: [0.0;2],
			fRec16: [0.0;2],
			fHslider7: 0.0,
			fRec27: [0.0;2],
			fVslider9: 0.0,
			fRec29: [0.0;2],
			IOTA: 0,
			fVec2: [0.0;4096],
			fVslider10: 0.0,
			fRec28: [0.0;2],
			fVslider11: 0.0,
			fVslider12: 0.0,
			fRec30: [0.0;2],
			fVec3: [0.0;16384],
			fVslider13: 0.0,
			fRec31: [0.0;2],
			fVslider14: 0.0,
			fRec32: [0.0;2],
			fConst17: 0.0,
			fVslider15: 0.0,
			fRec35: [0.0;2],
			fRec34: [0.0;2],
			fRec37: [0.0;2],
			fConst18: 0.0,
			fRec38: [0.0;2],
			fCheckbox2: 0.0,
			fHslider8: 0.0,
			fHslider9: 0.0,
			fHslider10: 0.0,
			fHslider11: 0.0,
			fHslider12: 0.0,
			fHslider13: 0.0,
			fHslider14: 0.0,
			fRec44: [0.0;2],
			fRec45: [0.0;2],
			fRec43: [0.0;3],
			fRec42: [0.0;3],
			fRec41: [0.0;3],
			fRec40: [0.0;3],
			fRec39: [0.0;2],
			fCheckbox3: 0.0,
			fConst19: 0.0,
			fRec46: [0.0;2],
			fConst20: 0.0,
			fRec47: [0.0;2],
			fConst21: 0.0,
			fRec48: [0.0;2],
			fConst22: 0.0,
			fRec49: [0.0;2],
			fRec54: [0.0;3],
			fRec53: [0.0;3],
			fRec52: [0.0;3],
			fRec51: [0.0;3],
			fRec50: [0.0;2],
			fVec4: [0.0;2048],
			fRec8: [0.0;2],
			fRec56: [0.0;2],
			fVec5: [0.0;2048],
			fRec55: [0.0;2],
			fRec58: [0.0;2],
			fVec6: [0.0;2048],
			fRec57: [0.0;2],
			fRec60: [0.0;2],
			fVec7: [0.0;2048],
			fRec59: [0.0;2],
			fRec62: [0.0;2],
			fVec8: [0.0;2048],
			fRec61: [0.0;2],
			fRec64: [0.0;2],
			fVec9: [0.0;2048],
			fRec63: [0.0;2],
			fRec66: [0.0;2],
			fVec10: [0.0;2048],
			fRec65: [0.0;2],
			fRec68: [0.0;2],
			fVec11: [0.0;2048],
			fRec67: [0.0;2],
			fVec12: [0.0;1024],
			fRec6: [0.0;2],
			fVec13: [0.0;512],
			fRec4: [0.0;2],
			fVec14: [0.0;512],
			fRec2: [0.0;2],
			fVec15: [0.0;256],
			fRec0: [0.0;2],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("analyzers.lib/name", "Faust Analyzer Library");
		m.declare("analyzers.lib/version", "0.1");
		m.declare("author", "darkoverlordofdata");
		m.declare("basics.lib/name", "Faust Basic Element Library");
		m.declare("basics.lib/version", "0.1");
		m.declare("copyright", "(c)DarkOverlordOfData 2021");
		m.declare("delays.lib/name", "Faust Delay Library");
		m.declare("delays.lib/version", "0.1");
		m.declare("description", "Amplifier demo application.");
		m.declare("filename", "amp_sim.dsp");
		m.declare("filters.lib/dcblocker:author", "Julius O. Smith III");
		m.declare("filters.lib/dcblocker:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/dcblocker:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/fir:author", "Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/iir:author", "Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/name", "Faust Filters Library");
		m.declare("filters.lib/nlf2:author", "Julius O. Smith III");
		m.declare("filters.lib/nlf2:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/nlf2:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/peak_eq:author", "Julius O. Smith III");
		m.declare("filters.lib/peak_eq:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/peak_eq:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/pole:author", "Julius O. Smith III");
		m.declare("filters.lib/pole:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/pole:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/tf1:author", "Julius O. Smith III");
		m.declare("filters.lib/tf1:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2:author", "Julius O. Smith III");
		m.declare("filters.lib/tf2:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2s:author", "Julius O. Smith III");
		m.declare("filters.lib/tf2s:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2s:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/version", "0.3");
		m.declare("filters.lib/zero:author", "Julius O. Smith III");
		m.declare("filters.lib/zero:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/zero:license", "MIT-style STK-4.3 license");
		m.declare("freeverb.dsp/author", "Grame");
		m.declare("freeverb.dsp/copyright", "(c) GRAME 2006 and MoForte Inc. 2017");
		m.declare("freeverb.dsp/license", "BSD");
		m.declare("freeverb.dsp/name", "freeverb");
		m.declare("freeverb.dsp/reference", "https://ccrma.stanford.edu/~jos/pasp/Freeverb.html");
		m.declare("freeverb.dsp/version", "1.0");
		m.declare("layout2.dsp/designer", "Robert A. Moog");
		m.declare("license", "MIT");
		m.declare("maths.lib/author", "GRAME");
		m.declare("maths.lib/copyright", "GRAME");
		m.declare("maths.lib/license", "LGPL with exception");
		m.declare("maths.lib/name", "Faust Math Library");
		m.declare("maths.lib/version", "2.3");
		m.declare("name", "amp-sim");
		m.declare("oscillators.lib/name", "Faust Oscillator Library");
		m.declare("oscillators.lib/version", "0.1");
		m.declare("phaflangers.lib/name", "Faust Phaser and Flanger Library");
		m.declare("phaflangers.lib/version", "0.0");
		m.declare("platform.lib/name", "Generic Platform Library");
		m.declare("platform.lib/version", "0.1");
		m.declare("routes.lib/name", "Faust Signal Routing Library");
		m.declare("routes.lib/version", "0.2");
		m.declare("signals.lib/name", "Faust Signal Routing Library");
		m.declare("signals.lib/version", "0.0");
		m.declare("version", "0.1");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 2;
	}
	fn get_num_outputs(&self) -> i32 {
		return 2;
	}
	fn get_input_rate(&self, channel: i32) -> i32 {
		let mut rate: i32;
		match (channel) {
			0 => {
				rate = 1;
			},
			1 => {
				rate = 1;
			},
			_ => {
				rate = -1;
			},
		} 
		return rate;
	}
	fn get_output_rate(&self, channel: i32) -> i32 {
		let mut rate: i32;
		match (channel) {
			0 => {
				rate = 1;
			},
			1 => {
				rate = 1;
			},
			_ => {
				rate = -1;
			},
		} 
		return rate;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: mydspSIG0 = newmydspSIG0();
		sig0.instance_initmydspSIG0(sample_rate);
		sig0.fillmydspSIG0(65536, unsafe { &mut ftbl0mydspSIG0 });
		let mut sig1: mydspSIG1 = newmydspSIG1();
		sig1.instance_initmydspSIG1(sample_rate);
		sig1.fillmydspSIG1(65536, unsafe { &mut ftbl1mydspSIG1 });
	}
	fn instance_reset_params(&mut self) {
		self.fVslider0 = 0.0;
		self.fVslider1 = 0.333299994;
		self.fVslider2 = 0.5;
		self.fVslider3 = 0.5;
		self.fCheckbox0 = 0.0;
		self.fHslider0 = 0.0;
		self.fVslider4 = 0.0;
		self.fVslider5 = 0.5;
		self.fVslider6 = 0.0;
		self.fVslider7 = 0.0;
		self.fVslider8 = 0.5;
		self.fCheckbox1 = 0.0;
		self.fHslider1 = 1.0;
		self.fHslider2 = -0.400000006;
		self.fHslider3 = 0.0;
		self.fHslider4 = 1.0;
		self.fHslider5 = 4.0;
		self.fHslider6 = 1.0;
		self.fHslider7 = -3.0;
		self.fVslider9 = 0.0;
		self.fVslider10 = 0.219999999;
		self.fVslider11 = 0.0;
		self.fVslider12 = 0.75;
		self.fVslider13 = 0.5;
		self.fVslider14 = 0.5;
		self.fVslider15 = 0.5;
		self.fCheckbox2 = 0.0;
		self.fHslider8 = 1.0;
		self.fHslider9 = 1000.0;
		self.fHslider10 = 0.0;
		self.fHslider11 = 1.5;
		self.fHslider12 = 100.0;
		self.fHslider13 = 800.0;
		self.fHslider14 = 0.5;
		self.fCheckbox3 = 0.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
		}
		for l1 in 0..2 {
			self.fRec9[l1 as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec10[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec12[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec11[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec13[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec14[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec15[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec19[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec20[l9 as usize] = 0.0;
		}
		for l10 in 0..3 {
			self.fRec22[l10 as usize] = 0.0;
		}
		for l11 in 0..3 {
			self.fRec21[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec23[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec24[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec25[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec26[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fVec1[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec18[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec17[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec16[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec27[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec29[l21 as usize] = 0.0;
		}
		self.IOTA = 0;
		for l22 in 0..4096 {
			self.fVec2[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec28[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec30[l24 as usize] = 0.0;
		}
		for l25 in 0..16384 {
			self.fVec3[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec31[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec32[l27 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec35[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec34[l30 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec37[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec38[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec44[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec45[l35 as usize] = 0.0;
		}
		for l36 in 0..3 {
			self.fRec43[l36 as usize] = 0.0;
		}
		for l37 in 0..3 {
			self.fRec42[l37 as usize] = 0.0;
		}
		for l38 in 0..3 {
			self.fRec41[l38 as usize] = 0.0;
		}
		for l39 in 0..3 {
			self.fRec40[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec39[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec46[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec47[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec48[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec49[l44 as usize] = 0.0;
		}
		for l45 in 0..3 {
			self.fRec54[l45 as usize] = 0.0;
		}
		for l46 in 0..3 {
			self.fRec53[l46 as usize] = 0.0;
		}
		for l47 in 0..3 {
			self.fRec52[l47 as usize] = 0.0;
		}
		for l48 in 0..3 {
			self.fRec51[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec50[l49 as usize] = 0.0;
		}
		for l50 in 0..2048 {
			self.fVec4[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec8[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec56[l52 as usize] = 0.0;
		}
		for l53 in 0..2048 {
			self.fVec5[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec55[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec58[l55 as usize] = 0.0;
		}
		for l56 in 0..2048 {
			self.fVec6[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec57[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec60[l58 as usize] = 0.0;
		}
		for l59 in 0..2048 {
			self.fVec7[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec59[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec62[l61 as usize] = 0.0;
		}
		for l62 in 0..2048 {
			self.fVec8[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec61[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec64[l64 as usize] = 0.0;
		}
		for l65 in 0..2048 {
			self.fVec9[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec63[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec66[l67 as usize] = 0.0;
		}
		for l68 in 0..2048 {
			self.fVec10[l68 as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec65[l69 as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fRec68[l70 as usize] = 0.0;
		}
		for l71 in 0..2048 {
			self.fVec11[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec67[l72 as usize] = 0.0;
		}
		for l73 in 0..1024 {
			self.fVec12[l73 as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fRec6[l74 as usize] = 0.0;
		}
		for l75 in 0..512 {
			self.fVec13[l75 as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fRec4[l76 as usize] = 0.0;
		}
		for l77 in 0..512 {
			self.fVec14[l77 as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec2[l78 as usize] = 0.0;
		}
		for l79 in 0..256 {
			self.fVec15[l79 as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec0[l80 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(192000.0, F32::max(1.0, (self.fSampleRate as F32)));
		self.fConst1 = F32::exp((0.0 - (44.1223412 / self.fConst0)));
		self.fConst2 = (1.0 - self.fConst1);
		self.fConst3 = (1.0 / self.fConst0);
		self.fConst4 = (6.28318548 / self.fConst0);
		self.fConst5 = (10.0 / self.fConst0);
		self.fConst6 = (12566.3711 / self.fConst0);
		self.fConst7 = F32::tan(self.fConst6);
		self.fConst8 = (2.0 * (1.0 - (1.0 / mydsp_faustpower2_f(self.fConst7))));
		self.fConst9 = F32::tan((6283.18555 / self.fConst0));
		self.fConst10 = (2.0 * (1.0 - (1.0 / mydsp_faustpower2_f(self.fConst9))));
		self.fConst11 = (1.0 / self.fConst9);
		self.fConst12 = (3141.59277 / (self.fConst0 * F32::sin(self.fConst6)));
		self.fConst13 = (1.0 / self.fConst7);
		self.fConst14 = (3141.59277 / (self.fConst0 * F32::sin((25132.7422 / self.fConst0))));
		self.fConst15 = F32::exp((0.0 - (25.0 / self.fConst0)));
		self.fConst16 = (1.0 - self.fConst15);
		self.fConst17 = (0.333333343 / self.fConst0);
		self.fConst18 = (0.142857149 / self.fConst0);
		self.fConst19 = (0.5 / self.fConst0);
		self.fConst20 = (0.25 / self.fConst0);
		self.fConst21 = (0.166666672 / self.fConst0);
		self.fConst22 = (0.125 / self.fConst0);
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		mydsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("amp-sim");
		ui_interface.open_horizontal_box("0x00");
		ui_interface.declare(None, "1", "");
		ui_interface.open_horizontal_box("Effects");
		ui_interface.declare(None, "5", "");
		ui_interface.open_horizontal_box("Flanger");
		ui_interface.declare(None, "0", "");
		ui_interface.open_vertical_box("Knobs");
		ui_interface.declare(Some(ParamIndex(0)), "1", "");
		ui_interface.declare(Some(ParamIndex(0)), "midi", "ctrl 50");
		ui_interface.declare(Some(ParamIndex(0)), "style", "knob");
		ui_interface.add_vertical_slider("Delay", ParamIndex(0), 0.22, 0.0, 1.0, 1.0);
		ui_interface.declare(Some(ParamIndex(1)), "1", "");
		ui_interface.declare(Some(ParamIndex(1)), "midi", "ctrl 51");
		ui_interface.declare(Some(ParamIndex(1)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(1)), "unit", "Hz");
		ui_interface.add_vertical_slider("Rate", ParamIndex(1), 0.5, 0.0, 10.0, 0.01);
		ui_interface.declare(Some(ParamIndex(2)), "3", "");
		ui_interface.declare(Some(ParamIndex(2)), "midi", "ctrl 52");
		ui_interface.declare(Some(ParamIndex(2)), "style", "knob");
		ui_interface.add_vertical_slider("Depth", ParamIndex(2), 0.75, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(3)), "5", "");
		ui_interface.declare(Some(ParamIndex(3)), "midi", "ctrl 53");
		ui_interface.declare(Some(ParamIndex(3)), "style", "knob");
		ui_interface.add_vertical_slider("Feedback", ParamIndex(3), 0.0, -0.995, 0.98999999999999999, 0.001);
		ui_interface.declare(Some(ParamIndex(4)), "7", "");
		ui_interface.declare(Some(ParamIndex(4)), "midi", "ctrl 54");
		ui_interface.declare(Some(ParamIndex(4)), "style", "knob");
		ui_interface.add_vertical_slider("Waveshape", ParamIndex(4), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_vertical_box("Switches");
		ui_interface.declare(Some(ParamIndex(5)), "0", "");
		ui_interface.declare(Some(ParamIndex(5)), "midi", "ctrl 102");
		ui_interface.declare(Some(ParamIndex(5)), "style", "knob");
		ui_interface.add_vertical_slider("Enable", ParamIndex(5), 0.0, 0.0, 1.0, 1.0);
		ui_interface.declare(Some(ParamIndex(6)), "1", "");
		ui_interface.declare(Some(ParamIndex(6)), "midi", "ctrl 49");
		ui_interface.declare(Some(ParamIndex(6)), "style", "knob");
		ui_interface.add_vertical_slider("Invert", ParamIndex(6), 0.0, 0.0, 1.0, 1.0);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.declare(None, "6", "");
		ui_interface.open_horizontal_box("Chorus");
		ui_interface.declare(None, "0", "");
		ui_interface.open_vertical_box("Knobs");
		ui_interface.declare(Some(ParamIndex(7)), "0", "");
		ui_interface.declare(Some(ParamIndex(7)), "midi", "ctrl 55");
		ui_interface.declare(Some(ParamIndex(7)), "style", "knob");
		ui_interface.add_vertical_slider("Delay", ParamIndex(7), 0.5, 0.0, 1.0, 1.0);
		ui_interface.declare(Some(ParamIndex(8)), "1", "");
		ui_interface.declare(Some(ParamIndex(8)), "midi", "ctrl 56");
		ui_interface.declare(Some(ParamIndex(8)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(8)), "unit", "Hz");
		ui_interface.add_vertical_slider("Rate", ParamIndex(8), 0.5, 0.01, 7.0, 0.0001);
		ui_interface.declare(Some(ParamIndex(9)), "4", "");
		ui_interface.declare(Some(ParamIndex(9)), "midi", "ctrl 57");
		ui_interface.declare(Some(ParamIndex(9)), "style", "knob");
		ui_interface.add_vertical_slider("Depth", ParamIndex(9), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(10)), "6", "");
		ui_interface.declare(Some(ParamIndex(10)), "midi", "ctrl 58");
		ui_interface.declare(Some(ParamIndex(10)), "style", "knob");
		ui_interface.add_vertical_slider("Deviation", ParamIndex(10), 0.5, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_vertical_box("Switches");
		ui_interface.declare(Some(ParamIndex(11)), "0", "");
		ui_interface.declare(Some(ParamIndex(11)), "midi", "ctrl 103");
		ui_interface.declare(Some(ParamIndex(11)), "style", "knob");
		ui_interface.add_vertical_slider("Enable", ParamIndex(11), 0.0, 0.0, 1.0, 1.0);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.declare(None, "7", "");
		ui_interface.open_horizontal_box("Reverb");
		ui_interface.declare(None, "0", "");
		ui_interface.open_vertical_box("Knobs");
		ui_interface.declare(Some(ParamIndex(12)), "midi", "ctrl 3");
		ui_interface.declare(Some(ParamIndex(12)), "style", "knob");
		ui_interface.add_vertical_slider("Damp", ParamIndex(12), 0.5, 0.0, 1.0, 0.025000000000000001);
		ui_interface.declare(Some(ParamIndex(13)), "midi", "ctrl 4");
		ui_interface.declare(Some(ParamIndex(13)), "style", "knob");
		ui_interface.add_vertical_slider("RoomSize", ParamIndex(13), 0.5, 0.0, 1.0, 0.025000000000000001);
		ui_interface.declare(Some(ParamIndex(14)), "midi", "ctrl 79");
		ui_interface.declare(Some(ParamIndex(14)), "style", "knob");
		ui_interface.add_vertical_slider("Wet", ParamIndex(14), 0.33329999999999999, 0.0, 1.0, 0.025000000000000001);
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_vertical_box("Switches");
		ui_interface.declare(Some(ParamIndex(15)), "0", "");
		ui_interface.declare(Some(ParamIndex(15)), "midi", "ctrl 104");
		ui_interface.declare(Some(ParamIndex(15)), "style", "knob");
		ui_interface.add_vertical_slider("Enable", ParamIndex(15), 0.0, 0.0, 1.0, 1.0);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.declare(None, "tooltip", "Reference:   https://ccrma.stanford.edu/~jos/pasp/Flanging.html");
		ui_interface.open_vertical_box("PHASER2");
		ui_interface.declare(None, "0", "");
		ui_interface.open_horizontal_box("0x00");
		ui_interface.declare(Some(ParamIndex(16)), "0", "");
		ui_interface.declare(Some(ParamIndex(16)), "tooltip", "When this is checked, the phaser   has no effect");
		ui_interface.add_check_button("Bypass", ParamIndex(16));
		ui_interface.declare(Some(ParamIndex(17)), "1", "");
		ui_interface.add_check_button("Invert Internal Phaser Sum", ParamIndex(17));
		ui_interface.declare(Some(ParamIndex(18)), "2", "");
		ui_interface.add_check_button("Vibrato Mode", ParamIndex(18));
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_horizontal_box("0x00");
		ui_interface.declare(Some(ParamIndex(19)), "1", "");
		ui_interface.declare(Some(ParamIndex(19)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(19)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Speed", ParamIndex(19), 0.5, 0.0, 10.0, 0.001);
		ui_interface.declare(Some(ParamIndex(20)), "2", "");
		ui_interface.declare(Some(ParamIndex(20)), "style", "knob");
		ui_interface.add_horizontal_slider("Notch Depth (Intensity)", ParamIndex(20), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(21)), "3", "");
		ui_interface.declare(Some(ParamIndex(21)), "style", "knob");
		ui_interface.add_horizontal_slider("Feedback Gain", ParamIndex(21), 0.0, -0.999, 0.999, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("0x00");
		ui_interface.declare(Some(ParamIndex(22)), "1", "");
		ui_interface.declare(Some(ParamIndex(22)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(22)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(22)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Notch width", ParamIndex(22), 1000.0, 10.0, 5000.0, 1.0);
		ui_interface.declare(Some(ParamIndex(23)), "2", "");
		ui_interface.declare(Some(ParamIndex(23)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(23)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(23)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Min Notch1 Freq", ParamIndex(23), 100.0, 20.0, 5000.0, 1.0);
		ui_interface.declare(Some(ParamIndex(24)), "3", "");
		ui_interface.declare(Some(ParamIndex(24)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(24)), "style", "knob");
		ui_interface.declare(Some(ParamIndex(24)), "unit", "Hz");
		ui_interface.add_horizontal_slider("Max Notch1 Freq", ParamIndex(24), 800.0, 20.0, 10000.0, 1.0);
		ui_interface.declare(Some(ParamIndex(25)), "4", "");
		ui_interface.declare(Some(ParamIndex(25)), "style", "knob");
		ui_interface.add_horizontal_slider("Notch Freq Ratio: NotchFreq(n+1)/NotchFreq(n)", ParamIndex(25), 1.5, 1.1000000000000001, 4.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_horizontal_box("0x00");
		ui_interface.declare(Some(ParamIndex(26)), "unit", "dB");
		ui_interface.add_horizontal_slider("Phaser Output Level", ParamIndex(26), 0.0, -60.0, 10.0, 0.10000000000000001);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.open_horizontal_box("PowerAmp FAUST");
		ui_interface.declare(Some(ParamIndex(27)), "style", "knob");
		ui_interface.add_horizontal_slider("Curve k", ParamIndex(27), 1.0, 0.10000000000000001, 4.0, 0.001);
		ui_interface.declare(Some(ParamIndex(28)), "style", "knob");
		ui_interface.add_horizontal_slider("Drive gain", ParamIndex(28), 4.0, -10.0, 10.0, 0.001);
		ui_interface.declare(Some(ParamIndex(29)), "style", "knob");
		ui_interface.add_horizontal_slider("Level", ParamIndex(29), -3.0, -24.0, 24.0, 1.0);
		ui_interface.declare(Some(ParamIndex(30)), "name", "MV");
		ui_interface.declare(Some(ParamIndex(30)), "style", "knob");
		ui_interface.add_horizontal_slider("Master Volume", ParamIndex(30), 1.0, 0.0, 4.0, 0.10000000000000001);
		ui_interface.declare(Some(ParamIndex(31)), "name", "Level");
		ui_interface.declare(Some(ParamIndex(31)), "style", "knob");
		ui_interface.add_horizontal_slider("Negative gain", ParamIndex(31), -0.40000000000000002, -0.80000000000000004, 1.0, 0.01);
		ui_interface.declare(Some(ParamIndex(32)), "name", "p1Gain");
		ui_interface.declare(Some(ParamIndex(32)), "style", "knob");
		ui_interface.add_horizontal_slider("Presence", ParamIndex(32), 0.0, -15.0, 15.0, 0.10000000000000001);
		ui_interface.declare(Some(ParamIndex(33)), "style", "knob");
		ui_interface.add_horizontal_slider("Saturation dry wet", ParamIndex(33), 1.0, 0.0, 1.0, 0.001);
		ui_interface.add_check_button("bypass", ParamIndex(34));
		ui_interface.close_box();
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			16 => Some(self.fCheckbox0),
			34 => Some(self.fCheckbox1),
			18 => Some(self.fCheckbox2),
			17 => Some(self.fCheckbox3),
			26 => Some(self.fHslider0),
			33 => Some(self.fHslider1),
			21 => Some(self.fHslider10),
			25 => Some(self.fHslider11),
			23 => Some(self.fHslider12),
			24 => Some(self.fHslider13),
			19 => Some(self.fHslider14),
			31 => Some(self.fHslider2),
			32 => Some(self.fHslider3),
			30 => Some(self.fHslider4),
			28 => Some(self.fHslider5),
			27 => Some(self.fHslider6),
			29 => Some(self.fHslider7),
			20 => Some(self.fHslider8),
			22 => Some(self.fHslider9),
			15 => Some(self.fVslider0),
			14 => Some(self.fVslider1),
			0 => Some(self.fVslider10),
			6 => Some(self.fVslider11),
			2 => Some(self.fVslider12),
			7 => Some(self.fVslider13),
			10 => Some(self.fVslider14),
			8 => Some(self.fVslider15),
			13 => Some(self.fVslider2),
			12 => Some(self.fVslider3),
			11 => Some(self.fVslider4),
			9 => Some(self.fVslider5),
			5 => Some(self.fVslider6),
			4 => Some(self.fVslider7),
			1 => Some(self.fVslider8),
			3 => Some(self.fVslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			16 => { self.fCheckbox0 = value }
			34 => { self.fCheckbox1 = value }
			18 => { self.fCheckbox2 = value }
			17 => { self.fCheckbox3 = value }
			26 => { self.fHslider0 = value }
			33 => { self.fHslider1 = value }
			21 => { self.fHslider10 = value }
			25 => { self.fHslider11 = value }
			23 => { self.fHslider12 = value }
			24 => { self.fHslider13 = value }
			19 => { self.fHslider14 = value }
			31 => { self.fHslider2 = value }
			32 => { self.fHslider3 = value }
			30 => { self.fHslider4 = value }
			28 => { self.fHslider5 = value }
			27 => { self.fHslider6 = value }
			29 => { self.fHslider7 = value }
			20 => { self.fHslider8 = value }
			22 => { self.fHslider9 = value }
			15 => { self.fVslider0 = value }
			14 => { self.fVslider1 = value }
			0 => { self.fVslider10 = value }
			6 => { self.fVslider11 = value }
			2 => { self.fVslider12 = value }
			7 => { self.fVslider13 = value }
			10 => { self.fVslider14 = value }
			8 => { self.fVslider15 = value }
			13 => { self.fVslider2 = value }
			12 => { self.fVslider3 = value }
			11 => { self.fVslider4 = value }
			9 => { self.fVslider5 = value }
			5 => { self.fVslider6 = value }
			4 => { self.fVslider7 = value }
			1 => { self.fVslider8 = value }
			3 => { self.fVslider9 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (inputs0, inputs1) = if let [inputs0, inputs1, ..] = inputs {
			let inputs0 = inputs0[..count as usize].iter();
			let inputs1 = inputs1[..count as usize].iter();
			(inputs0, inputs1)
		} else {
			panic!("wrong number of inputs");
		};
		let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			(outputs0, outputs1)
		} else {
			panic!("wrong number of outputs");
		};
		let mut iSlow0: i32 = (1 - ((self.fVslider0 as F32) as i32));
		let mut fSlow1: F32 = (self.fVslider1 as F32);
		let mut fSlow2: F32 = ((0.280000001 * (self.fVslider2 as F32)) + 0.699999988);
		let mut fSlow3: F32 = (0.400000006 * (self.fVslider3 as F32));
		let mut fSlow4: F32 = (1.0 - fSlow3);
		let mut iSlow5: i32 = ((self.fCheckbox0 as F32) as i32);
		let mut fSlow6: F32 = F32::powf(10.0, (0.0500000007 * (self.fHslider0 as F32)));
		let mut iSlow7: i32 = (1 - ((self.fVslider4 as F32) as i32));
		let mut fSlow8: F32 = (self.fConst2 * (self.fVslider5 as F32));
		let mut iSlow9: i32 = (1 - ((self.fVslider6 as F32) as i32));
		let mut fSlow10: F32 = (self.fVslider7 as F32);
		let mut fSlow11: F32 = (self.fConst2 * (self.fVslider8 as F32));
		let mut fSlow12: F32 = (0.5 * (1.0 - fSlow10));
		let mut fSlow13: F32 = (self.fCheckbox1 as F32);
		let mut fSlow14: F32 = (0.00499999989 * (self.fHslider1 as F32));
		let mut fSlow15: F32 = (0.00100000005 * (self.fHslider2 as F32));
		let mut fSlow16: F32 = (self.fHslider3 as F32);
		let mut iSlow17: i32 = ((fSlow16 > 0.0) as i32);
		let mut fSlow18: F32 = F32::powf(10.0, (0.0500000007 * F32::abs(fSlow16)));
		let mut fSlow19: F32 = (self.fConst12 * fSlow18);
		let mut fSlow20: F32 = if (iSlow17 as i32 != 0) { self.fConst12 } else { fSlow19 };
		let mut fSlow21: F32 = (1.0 - (self.fConst11 * (fSlow20 - self.fConst11)));
		let mut fSlow22: F32 = ((self.fConst11 * (self.fConst11 + fSlow20)) + 1.0);
		let mut fSlow23: F32 = if (iSlow17 as i32 != 0) { fSlow19 } else { self.fConst12 };
		let mut fSlow24: F32 = ((self.fConst11 * (self.fConst11 + fSlow23)) + 1.0);
		let mut fSlow25: F32 = (1.0 - (self.fConst11 * (fSlow23 - self.fConst11)));
		let mut fSlow26: F32 = (self.fConst14 * fSlow18);
		let mut fSlow27: F32 = if (iSlow17 as i32 != 0) { self.fConst14 } else { fSlow26 };
		let mut fSlow28: F32 = (1.0 - (self.fConst13 * (fSlow27 - self.fConst13)));
		let mut fSlow29: F32 = ((self.fConst13 * (self.fConst13 + fSlow27)) + 1.0);
		let mut fSlow30: F32 = if (iSlow17 as i32 != 0) { fSlow26 } else { self.fConst14 };
		let mut fSlow31: F32 = ((self.fConst13 * (self.fConst13 + fSlow30)) + 1.0);
		let mut fSlow32: F32 = (1.0 - (self.fConst13 * (fSlow30 - self.fConst13)));
		let mut fSlow33: F32 = (0.00100000005 * (self.fHslider4 as F32));
		let mut fSlow34: F32 = (0.00499999989 * (self.fHslider5 as F32));
		let mut fSlow35: F32 = (0.00499999989 * (self.fHslider6 as F32));
		let mut fSlow36: F32 = (0.00499999989 * F32::powf(10.0, (0.0500000007 * (self.fHslider7 as F32))));
		let mut fSlow37: F32 = (self.fConst2 * (self.fVslider9 as F32));
		let mut fSlow38: F32 = (2003.0 * (self.fVslider10 as F32));
		let mut iSlow39: i32 = ((self.fVslider11 as F32) as i32);
		let mut fSlow40: F32 = (self.fConst2 * (self.fVslider12 as F32));
		let mut fSlow41: F32 = (8.19200039 * (self.fVslider13 as F32));
		let mut fSlow42: F32 = (6.2500003e-05 * (self.fVslider14 as F32));
		let mut fSlow43: F32 = (self.fConst2 * (self.fVslider15 as F32));
		let mut fSlow44: F32 = (0.5 * if (((self.fCheckbox2 as F32) as i32) as i32 != 0) { 2.0 } else { (self.fHslider8 as F32) });
		let mut fSlow45: F32 = (1.0 - fSlow44);
		let mut fSlow46: F32 = F32::exp((self.fConst3 * (0.0 - (3.14159274 * (self.fHslider9 as F32)))));
		let mut fSlow47: F32 = mydsp_faustpower2_f(fSlow46);
		let mut fSlow48: F32 = (0.0 - (2.0 * fSlow46));
		let mut fSlow49: F32 = (self.fHslider10 as F32);
		let mut fSlow50: F32 = (self.fHslider11 as F32);
		let mut fSlow51: F32 = (self.fHslider12 as F32);
		let mut fSlow52: F32 = (self.fConst4 * fSlow51);
		let mut fSlow53: F32 = (0.5 * (0.0 - (self.fConst4 * (fSlow51 - F32::max(fSlow51, (self.fHslider13 as F32))))));
		let mut fSlow54: F32 = (self.fConst4 * (self.fHslider14 as F32));
		let mut fSlow55: F32 = F32::sin(fSlow54);
		let mut fSlow56: F32 = F32::cos(fSlow54);
		let mut fSlow57: F32 = mydsp_faustpower2_f(fSlow50);
		let mut fSlow58: F32 = mydsp_faustpower3_f(fSlow50);
		let mut fSlow59: F32 = mydsp_faustpower4_f(fSlow50);
		let mut fSlow60: F32 = if (((self.fCheckbox3 as F32) as i32) as i32 != 0) { (-1.0 * fSlow44) } else { fSlow44 };
		let mut fSlow61: F32 = (1.0 - fSlow1);
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec9[0] = ((fSlow3 * self.fRec9[1]) + (fSlow4 * self.fRec8[1]));
			self.fRec10[0] = (fSlow8 + (self.fConst1 * self.fRec10[1]));
			self.fRec12[0] = (fSlow11 + (self.fConst1 * self.fRec12[1]));
			let mut fTemp0: F32 = (self.fRec11[1] + (self.fConst3 * self.fRec12[0]));
			self.fRec11[0] = (fTemp0 - F32::floor(fTemp0));
			let mut fTemp1: F32 = (self.fConst4 * self.fRec12[0]);
			let mut fTemp2: F32 = F32::sin(fTemp1);
			let mut fTemp3: F32 = F32::cos(fTemp1);
			self.fRec13[0] = ((self.fRec14[1] * fTemp2) + (self.fRec13[1] * fTemp3));
			let mut fTemp4: F32 = ((1 - self.iVec0[1]) as F32);
			self.fRec14[0] = ((fTemp4 + (self.fRec14[1] * fTemp3)) - (fTemp2 * self.fRec13[1]));
			let mut fTemp5: F32 = ((fSlow10 * (1.0 - F32::abs(((2.0 * self.fRec11[0]) + -1.0)))) + (fSlow12 * (self.fRec13[0] + 1.0)));
			let mut fTemp6: F32 = ((*input0 as F32) + (*input1 as F32));
			let mut fTemp7: F32 = (self.fConst5 + self.fRec15[1]);
			let mut fTemp8: F32 = (self.fRec15[1] - self.fConst5);
			self.fRec15[0] = if (((fTemp7 < fSlow13) as i32) as i32 != 0) { fTemp7 } else { if (((fTemp8 > fSlow13) as i32) as i32 != 0) { fTemp8 } else { fSlow13 } };
			self.fRec19[0] = (fSlow14 + (0.995000005 * self.fRec19[1]));
			self.fRec20[0] = (fSlow15 + (0.999000013 * self.fRec20[1]));
			let mut fTemp9: F32 = (self.fConst10 * self.fRec22[1]);
			self.fRec22[0] = (self.fRec16[1] - (((self.fRec22[2] * fSlow21) + fTemp9) / fSlow22));
			let mut fTemp10: F32 = (self.fConst8 * self.fRec21[1]);
			self.fRec21[0] = ((((fTemp9 + (self.fRec22[0] * fSlow24)) + (self.fRec22[2] * fSlow25)) / fSlow22) - (((self.fRec21[2] * fSlow28) + fTemp10) / fSlow29));
			self.fRec23[0] = (fSlow33 + (0.999000013 * self.fRec23[1]));
			let mut fTemp11: F32 = (1.0 - self.fRec15[0]);
			let mut fTemp12: F32 = (((self.fRec20[0] * ((fTemp10 + (self.fRec21[0] * fSlow31)) + (self.fRec21[2] * fSlow32))) / fSlow29) + ((fTemp6 * self.fRec23[0]) * fTemp11));
			let mut fTemp13: F32 = F32::abs(fTemp12);
			self.fRec24[0] = F32::max(fTemp13, ((self.fConst15 * self.fRec24[1]) + (self.fConst16 * fTemp13)));
			self.fRec25[0] = (fSlow34 + (0.995000005 * self.fRec25[1]));
			let mut fTemp14: F32 = F32::min(3.0, F32::max(-3.0, (self.fRec24[0] + (self.fRec25[0] * fTemp12))));
			self.fRec26[0] = (fSlow35 + (0.995000005 * self.fRec26[1]));
			let mut fTemp15: F32 = mydsp_faustpower2_f(self.fRec26[0]);
			let mut fTemp16: F32 = (fTemp15 * mydsp_faustpower2_f(fTemp14));
			let mut fTemp17: F32 = (fTemp16 + 27.0);
			let mut fTemp18: F32 = ((9.0 * fTemp15) + 27.0);
			let mut fTemp19: F32 = (((9.0 * fTemp16) + 27.0) * (fTemp15 + 27.0));
			let mut fTemp20: F32 = (((1.0 - self.fRec19[0]) * fTemp12) + (0.239999995 * ((((self.fRec19[0] * fTemp14) * fTemp17) * fTemp18) / fTemp19)));
			self.fVec1[0] = fTemp20;
			self.fRec18[0] = (self.fVec1[1] - (0.239999995 * ((((fTemp14 * fTemp17) * fTemp18) * (self.fRec18[1] - fTemp20)) / fTemp19)));
			self.fRec17[0] = ((self.fRec18[0] + (0.995000005 * self.fRec17[1])) - self.fRec18[1]);
			self.fRec16[0] = self.fRec17[0];
			self.fRec27[0] = (fSlow36 + (0.995000005 * self.fRec27[1]));
			let mut fTemp21: F32 = ((fTemp6 * self.fRec15[0]) + (4.0 * ((self.fRec16[0] * self.fRec27[0]) * fTemp11)));
			let mut fTemp22: F32 = if (iSlow9 as i32 != 0) { 0.0 } else { fTemp21 };
			self.fRec29[0] = (fSlow37 + (self.fConst1 * self.fRec29[1]));
			let mut fTemp23: F32 = ((self.fRec29[0] * self.fRec28[1]) - fTemp22);
			self.fVec2[(self.IOTA & 4095) as usize] = fTemp23;
			let mut fTemp24: F32 = (fSlow38 * fTemp5);
			let mut fTemp25: F32 = (fTemp24 + 44.0);
			let mut iTemp26: i32 = (fTemp25 as i32);
			let mut fTemp27: F32 = F32::floor(fTemp25);
			self.fRec28[0] = ((self.fVec2[((self.IOTA - std::cmp::min(2049, std::cmp::max(0, iTemp26))) & 4095) as usize] * (fTemp27 + (-43.0 - fTemp24))) + ((fTemp24 + (44.0 - fTemp27)) * self.fVec2[((self.IOTA - std::cmp::min(2049, std::cmp::max(0, (iTemp26 + 1)))) & 4095) as usize]));
			self.fRec30[0] = (fSlow40 + (self.fConst1 * self.fRec30[1]));
			let mut fTemp28: F32 = if (iSlow9 as i32 != 0) { fTemp21 } else { ((fTemp22 + (self.fRec28[0] * if (iSlow39 as i32 != 0) { (-1.0 * self.fRec30[0]) } else { self.fRec30[0] })) / (self.fRec30[0] + 1.0)) };
			let mut fTemp29: F32 = if (iSlow7 as i32 != 0) { 0.0 } else { fTemp28 };
			let mut fTemp30: F32 = (self.fRec10[0] * fTemp29);
			self.fVec3[(self.IOTA & 16383) as usize] = fTemp30;
			self.fRec31[0] = (fSlow41 + (0.999000013 * self.fRec31[1]));
			self.fRec32[0] = ((0.999000013 * self.fRec32[1]) + (fSlow42 * self.fRec31[0]));
			self.fRec35[0] = (fSlow43 + (self.fConst1 * self.fRec35[1]));
			let mut fTemp31: F32 = (self.fRec34[1] + (self.fConst17 * self.fRec35[0]));
			self.fRec34[0] = (fTemp31 - F32::floor(fTemp31));
			let mut fTemp32: F32 = F32::min(8192.0, ((0.375 * self.fRec31[0]) + (self.fRec32[0] * unsafe { ftbl0mydspSIG0[((65536.0 * self.fRec34[0]) as i32) as usize] })));
			let mut iTemp33: i32 = (fTemp32 as i32);
			let mut fTemp34: F32 = F32::floor(fTemp32);
			let mut fTemp35: F32 = (self.fRec37[1] + (self.fConst3 * self.fRec35[0]));
			self.fRec37[0] = (fTemp35 - F32::floor(fTemp35));
			let mut fTemp36: F32 = F32::min(8192.0, ((0.125 * self.fRec31[0]) + (self.fRec32[0] * unsafe { ftbl1mydspSIG1[((65536.0 * self.fRec37[0]) as i32) as usize] })));
			let mut fTemp37: F32 = F32::floor(fTemp36);
			let mut iTemp38: i32 = (fTemp36 as i32);
			let mut fTemp39: F32 = ((1.0 - self.fRec10[0]) * fTemp29);
			let mut fTemp40: F32 = (self.fRec38[1] + (self.fConst18 * self.fRec35[0]));
			self.fRec38[0] = (fTemp40 - F32::floor(fTemp40));
			let mut fTemp41: F32 = F32::min(8192.0, ((0.875 * self.fRec31[0]) + (self.fRec32[0] * (0.0 - unsafe { ftbl0mydspSIG0[((65536.0 * self.fRec38[0]) as i32) as usize] }))));
			let mut iTemp42: i32 = (fTemp41 as i32);
			let mut fTemp43: F32 = F32::floor(fTemp41);
			let mut fTemp44: F32 = if (iSlow7 as i32 != 0) { fTemp28 } else { (((0.707106769 * ((self.fVec3[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, iTemp33))) & 16383) as usize] * (fTemp34 + (1.0 - fTemp32))) + ((fTemp32 - fTemp34) * self.fVec3[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, (iTemp33 + 1)))) & 16383) as usize]))) + (((fTemp36 - fTemp37) * self.fVec3[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, (iTemp38 + 1)))) & 16383) as usize]) + (fTemp39 + (self.fVec3[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, iTemp38))) & 16383) as usize] * (fTemp37 + (1.0 - fTemp36)))))) - (0.707106769 * ((self.fVec3[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, iTemp42))) & 16383) as usize] * (fTemp43 + (1.0 - fTemp41))) + ((fTemp41 - fTemp43) * self.fVec3[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, (iTemp42 + 1)))) & 16383) as usize])))) };
			let mut fTemp45: F32 = if (iSlow5 as i32 != 0) { 0.0 } else { fTemp44 };
			self.fRec44[0] = ((fSlow55 * self.fRec45[1]) + (fSlow56 * self.fRec44[1]));
			self.fRec45[0] = ((fTemp4 + (fSlow56 * self.fRec45[1])) - (fSlow55 * self.fRec44[1]));
			let mut fTemp46: F32 = (fSlow52 + (fSlow53 * (1.0 - self.fRec44[0])));
			let mut fTemp47: F32 = (self.fRec43[1] * F32::cos((fSlow50 * fTemp46)));
			self.fRec43[0] = (((fSlow6 * fTemp45) + (fSlow49 * self.fRec39[1])) - ((fSlow48 * fTemp47) + (fSlow47 * self.fRec43[2])));
			let mut fTemp48: F32 = (self.fRec42[1] * F32::cos((fSlow57 * fTemp46)));
			self.fRec42[0] = ((fSlow48 * (fTemp47 - fTemp48)) + (self.fRec43[2] + (fSlow47 * (self.fRec43[0] - self.fRec42[2]))));
			let mut fTemp49: F32 = (self.fRec41[1] * F32::cos((fSlow58 * fTemp46)));
			self.fRec41[0] = ((fSlow48 * (fTemp48 - fTemp49)) + (self.fRec42[2] + (fSlow47 * (self.fRec42[0] - self.fRec41[2]))));
			let mut fTemp50: F32 = (self.fRec40[1] * F32::cos((fSlow59 * fTemp46)));
			self.fRec40[0] = ((fSlow48 * (fTemp49 - fTemp50)) + (self.fRec41[2] + (fSlow47 * (self.fRec41[0] - self.fRec40[2]))));
			self.fRec39[0] = ((fSlow47 * self.fRec40[0]) + ((fSlow48 * fTemp50) + self.fRec40[2]));
			let mut fTemp51: F32 = if (iSlow5 as i32 != 0) { fTemp44 } else { ((fSlow6 * (fTemp45 * fSlow45)) + (self.fRec39[0] * fSlow60)) };
			let mut fTemp52: F32 = if (iSlow0 as i32 != 0) { 0.0 } else { fTemp51 };
			let mut fTemp53: F32 = (self.fRec46[1] + (self.fConst19 * self.fRec35[0]));
			self.fRec46[0] = (fTemp53 - F32::floor(fTemp53));
			let mut iTemp54: i32 = ((65536.0 * self.fRec46[0]) as i32);
			let mut fTemp55: F32 = F32::min(8192.0, ((0.25 * self.fRec31[0]) + (self.fRec32[0] * ((0.707106769 * unsafe { ftbl1mydspSIG1[iTemp54 as usize] }) + (0.707106769 * unsafe { ftbl0mydspSIG0[iTemp54 as usize] })))));
			let mut iTemp56: i32 = (fTemp55 as i32);
			let mut fTemp57: F32 = F32::floor(fTemp55);
			let mut fTemp58: F32 = (self.fRec47[1] + (self.fConst20 * self.fRec35[0]));
			self.fRec47[0] = (fTemp58 - F32::floor(fTemp58));
			let mut iTemp59: i32 = ((65536.0 * self.fRec47[0]) as i32);
			let mut fTemp60: F32 = F32::min(8192.0, ((0.5 * self.fRec31[0]) + (self.fRec32[0] * ((0.707106769 * unsafe { ftbl0mydspSIG0[iTemp59 as usize] }) - (0.707106769 * unsafe { ftbl1mydspSIG1[iTemp59 as usize] })))));
			let mut iTemp61: i32 = (fTemp60 as i32);
			let mut fTemp62: F32 = F32::floor(fTemp60);
			let mut fTemp63: F32 = (self.fRec48[1] + (self.fConst21 * self.fRec35[0]));
			self.fRec48[0] = (fTemp63 - F32::floor(fTemp63));
			let mut iTemp64: i32 = ((65536.0 * self.fRec48[0]) as i32);
			let mut fTemp65: F32 = F32::min(8192.0, ((0.75 * self.fRec31[0]) + (self.fRec32[0] * (0.0 - ((0.707106769 * unsafe { ftbl1mydspSIG1[iTemp64 as usize] }) + (0.707106769 * unsafe { ftbl0mydspSIG0[iTemp64 as usize] }))))));
			let mut iTemp66: i32 = (fTemp65 as i32);
			let mut fTemp67: F32 = F32::floor(fTemp65);
			let mut fTemp68: F32 = (self.fRec49[1] + (self.fConst22 * self.fRec35[0]));
			self.fRec49[0] = (fTemp68 - F32::floor(fTemp68));
			let mut iTemp69: i32 = ((65536.0 * self.fRec49[0]) as i32);
			let mut fTemp70: F32 = F32::min(8192.0, (self.fRec31[0] + (self.fRec32[0] * ((0.707106769 * unsafe { ftbl1mydspSIG1[iTemp69 as usize] }) - (0.707106769 * unsafe { ftbl0mydspSIG0[iTemp69 as usize] })))));
			let mut iTemp71: i32 = (fTemp70 as i32);
			let mut fTemp72: F32 = F32::floor(fTemp70);
			let mut fTemp73: F32 = if (iSlow7 as i32 != 0) { fTemp28 } else { (fTemp39 - ((((0.382683426 * ((self.fVec3[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, iTemp56))) & 16383) as usize] * (fTemp57 + (1.0 - fTemp55))) + ((fTemp55 - fTemp57) * self.fVec3[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, (iTemp56 + 1)))) & 16383) as usize]))) + (0.923879504 * ((self.fVec3[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, iTemp61))) & 16383) as usize] * (fTemp62 + (1.0 - fTemp60))) + ((fTemp60 - fTemp62) * self.fVec3[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, (iTemp61 + 1)))) & 16383) as usize])))) + (0.923879504 * ((self.fVec3[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, iTemp66))) & 16383) as usize] * (fTemp67 + (1.0 - fTemp65))) + ((fTemp65 - fTemp67) * self.fVec3[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, (iTemp66 + 1)))) & 16383) as usize])))) + (0.382683426 * ((self.fVec3[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, iTemp71))) & 16383) as usize] * (fTemp72 + (1.0 - fTemp70))) + ((fTemp70 - fTemp72) * self.fVec3[((self.IOTA - std::cmp::min(8193, std::cmp::max(0, (iTemp71 + 1)))) & 16383) as usize]))))) };
			let mut fTemp74: F32 = if (iSlow5 as i32 != 0) { 0.0 } else { fTemp73 };
			let mut fTemp75: F32 = (fSlow52 + (fSlow53 * (1.0 - self.fRec45[0])));
			let mut fTemp76: F32 = (self.fRec54[1] * F32::cos((fSlow50 * fTemp75)));
			self.fRec54[0] = (((fSlow6 * fTemp74) + (fSlow49 * self.fRec50[1])) - ((fSlow48 * fTemp76) + (fSlow47 * self.fRec54[2])));
			let mut fTemp77: F32 = (self.fRec53[1] * F32::cos((fSlow57 * fTemp75)));
			self.fRec53[0] = ((fSlow48 * (fTemp76 - fTemp77)) + (self.fRec54[2] + (fSlow47 * (self.fRec54[0] - self.fRec53[2]))));
			let mut fTemp78: F32 = (self.fRec52[1] * F32::cos((fSlow58 * fTemp75)));
			self.fRec52[0] = ((fSlow48 * (fTemp77 - fTemp78)) + (self.fRec53[2] + (fSlow47 * (self.fRec53[0] - self.fRec52[2]))));
			let mut fTemp79: F32 = (self.fRec51[1] * F32::cos((fSlow59 * fTemp75)));
			self.fRec51[0] = ((fSlow48 * (fTemp78 - fTemp79)) + (self.fRec52[2] + (fSlow47 * (self.fRec52[0] - self.fRec51[2]))));
			self.fRec50[0] = ((fSlow47 * self.fRec51[0]) + ((fSlow48 * fTemp79) + self.fRec51[2]));
			let mut fTemp80: F32 = if (iSlow5 as i32 != 0) { fTemp73 } else { ((fSlow6 * (fSlow45 * fTemp74)) + (self.fRec50[0] * fSlow60)) };
			let mut fTemp81: F32 = if (iSlow0 as i32 != 0) { 0.0 } else { fTemp80 };
			let mut fTemp82: F32 = (0.0149999997 * (fTemp52 + fTemp81));
			self.fVec4[(self.IOTA & 2047) as usize] = ((fSlow2 * self.fRec9[0]) + fTemp82);
			self.fRec8[0] = self.fVec4[((self.IOTA - 1116) & 2047) as usize];
			self.fRec56[0] = ((fSlow3 * self.fRec56[1]) + (fSlow4 * self.fRec55[1]));
			self.fVec5[(self.IOTA & 2047) as usize] = (fTemp82 + (fSlow2 * self.fRec56[0]));
			self.fRec55[0] = self.fVec5[((self.IOTA - 1188) & 2047) as usize];
			self.fRec58[0] = ((fSlow3 * self.fRec58[1]) + (fSlow4 * self.fRec57[1]));
			self.fVec6[(self.IOTA & 2047) as usize] = (fTemp82 + (fSlow2 * self.fRec58[0]));
			self.fRec57[0] = self.fVec6[((self.IOTA - 1277) & 2047) as usize];
			self.fRec60[0] = ((fSlow3 * self.fRec60[1]) + (fSlow4 * self.fRec59[1]));
			self.fVec7[(self.IOTA & 2047) as usize] = (fTemp82 + (fSlow2 * self.fRec60[0]));
			self.fRec59[0] = self.fVec7[((self.IOTA - 1356) & 2047) as usize];
			self.fRec62[0] = ((fSlow3 * self.fRec62[1]) + (fSlow4 * self.fRec61[1]));
			self.fVec8[(self.IOTA & 2047) as usize] = (fTemp82 + (fSlow2 * self.fRec62[0]));
			self.fRec61[0] = self.fVec8[((self.IOTA - 1422) & 2047) as usize];
			self.fRec64[0] = ((fSlow3 * self.fRec64[1]) + (fSlow4 * self.fRec63[1]));
			self.fVec9[(self.IOTA & 2047) as usize] = (fTemp82 + (fSlow2 * self.fRec64[0]));
			self.fRec63[0] = self.fVec9[((self.IOTA - 1491) & 2047) as usize];
			self.fRec66[0] = ((fSlow3 * self.fRec66[1]) + (fSlow4 * self.fRec65[1]));
			self.fVec10[(self.IOTA & 2047) as usize] = (fTemp82 + (fSlow2 * self.fRec66[0]));
			self.fRec65[0] = self.fVec10[((self.IOTA - 1557) & 2047) as usize];
			self.fRec68[0] = ((fSlow3 * self.fRec68[1]) + (fSlow4 * self.fRec67[1]));
			self.fVec11[(self.IOTA & 2047) as usize] = (fTemp82 + (fSlow2 * self.fRec68[0]));
			self.fRec67[0] = self.fVec11[((self.IOTA - 1617) & 2047) as usize];
			let mut fTemp83: F32 = (((((((self.fRec8[0] + self.fRec55[0]) + self.fRec57[0]) + self.fRec59[0]) + self.fRec61[0]) + self.fRec63[0]) + self.fRec65[0]) + self.fRec67[0]);
			self.fVec12[(self.IOTA & 1023) as usize] = (fTemp83 + (0.5 * self.fRec6[1]));
			self.fRec6[0] = self.fVec12[((self.IOTA - 556) & 1023) as usize];
			let mut fRec7: F32 = (self.fRec6[1] - fTemp83);
			self.fVec13[(self.IOTA & 511) as usize] = (fRec7 + (0.5 * self.fRec4[1]));
			self.fRec4[0] = self.fVec13[((self.IOTA - 441) & 511) as usize];
			let mut fRec5: F32 = (self.fRec4[1] - fRec7);
			self.fVec14[(self.IOTA & 511) as usize] = (fRec5 + (0.5 * self.fRec2[1]));
			self.fRec2[0] = self.fVec14[((self.IOTA - 341) & 511) as usize];
			let mut fRec3: F32 = (self.fRec2[1] - fRec5);
			self.fVec15[(self.IOTA & 255) as usize] = (fRec3 + (0.5 * self.fRec0[1]));
			self.fRec0[0] = self.fVec15[((self.IOTA - 225) & 255) as usize];
			let mut fRec1: F32 = (self.fRec0[1] - fRec3);
			let mut fTemp84: F32 = (fSlow1 * fRec1);
			*output0 = (if (iSlow0 as i32 != 0) { fTemp51 } else { (fTemp84 + (fSlow61 * fTemp52)) } as F32);
			*output1 = (if (iSlow0 as i32 != 0) { fTemp80 } else { (fTemp84 + (fSlow61 * fTemp81)) } as F32);
			self.iVec0[1] = self.iVec0[0];
			self.fRec9[1] = self.fRec9[0];
			self.fRec10[1] = self.fRec10[0];
			self.fRec12[1] = self.fRec12[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec13[1] = self.fRec13[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec19[1] = self.fRec19[0];
			self.fRec20[1] = self.fRec20[0];
			self.fRec22[2] = self.fRec22[1];
			self.fRec22[1] = self.fRec22[0];
			self.fRec21[2] = self.fRec21[1];
			self.fRec21[1] = self.fRec21[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec26[1] = self.fRec26[0];
			self.fVec1[1] = self.fVec1[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec16[1] = self.fRec16[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec29[1] = self.fRec29[0];
			self.IOTA = (self.IOTA + 1);
			self.fRec28[1] = self.fRec28[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec34[1] = self.fRec34[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec44[1] = self.fRec44[0];
			self.fRec45[1] = self.fRec45[0];
			self.fRec43[2] = self.fRec43[1];
			self.fRec43[1] = self.fRec43[0];
			self.fRec42[2] = self.fRec42[1];
			self.fRec42[1] = self.fRec42[0];
			self.fRec41[2] = self.fRec41[1];
			self.fRec41[1] = self.fRec41[0];
			self.fRec40[2] = self.fRec40[1];
			self.fRec40[1] = self.fRec40[0];
			self.fRec39[1] = self.fRec39[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec47[1] = self.fRec47[0];
			self.fRec48[1] = self.fRec48[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec54[2] = self.fRec54[1];
			self.fRec54[1] = self.fRec54[0];
			self.fRec53[2] = self.fRec53[1];
			self.fRec53[1] = self.fRec53[0];
			self.fRec52[2] = self.fRec52[1];
			self.fRec52[1] = self.fRec52[0];
			self.fRec51[2] = self.fRec51[1];
			self.fRec51[1] = self.fRec51[0];
			self.fRec50[1] = self.fRec50[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec56[1] = self.fRec56[0];
			self.fRec55[1] = self.fRec55[0];
			self.fRec58[1] = self.fRec58[0];
			self.fRec57[1] = self.fRec57[0];
			self.fRec60[1] = self.fRec60[0];
			self.fRec59[1] = self.fRec59[0];
			self.fRec62[1] = self.fRec62[0];
			self.fRec61[1] = self.fRec61[0];
			self.fRec64[1] = self.fRec64[0];
			self.fRec63[1] = self.fRec63[0];
			self.fRec66[1] = self.fRec66[0];
			self.fRec65[1] = self.fRec65[0];
			self.fRec68[1] = self.fRec68[0];
			self.fRec67[1] = self.fRec67[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec0[1] = self.fRec0[0];
		}
	}

}


fn main() {

    // Create JACK client
    let (client, _status) = j::Client::new("faust_rust", j::client_options::NO_START_SERVER).unwrap();

    // Allocation DSP on the heap
    let mut dsp = Box::new(mydsp::new());

    println!("Faust Rust code running with JACK: sample-rate = {} buffer-size = {}", client.sample_rate(), client.buffer_size());

    println!("get_num_inputs: {}", dsp.get_num_inputs());
    println!("get_num_outputs: {}", dsp.get_num_outputs());

    // Init DSP with a given SR
    dsp.init(client.sample_rate() as i32);

    // Register ports. They will be used in a callback that will be
    // called when new data is available.

    let in_a = client.register_port("in1", j::AudioInSpec::default()).unwrap();
    let in_b = client.register_port("in2", j::AudioInSpec::default()).unwrap();

    let mut out_a = client.register_port("out1", j::AudioOutSpec::default()).unwrap();
    let mut out_b = client.register_port("out2", j::AudioOutSpec::default()).unwrap();

    let process_callback = move |_: &j::Client, ps: &j::ProcessScope| -> j::JackControl {
        let mut out_a_p = j::AudioOutPort::new(&mut out_a, ps);
        let mut out_b_p = j::AudioOutPort::new(&mut out_b, ps);

        let in_a_p = j::AudioInPort::new(&in_a, ps);
        let in_b_p = j::AudioInPort::new(&in_b, ps);

        let input0: &[f32] = &in_a_p;
        let input1: &[f32] = &in_b_p;

        let output0: &mut[f32] = &mut out_a_p;
        let output1: &mut[f32] = &mut out_b_p;

        let inputs = &[input0, input1];
        let outputs = &mut[output0, output1];

        dsp.compute(in_a_p.len() as i32, inputs, outputs);

        j::JackControl::Continue
    };
    let process = j::ClosureProcessHandler::new(process_callback);

    // Activate the client, which starts the processing.
    let active_client = j::AsyncClient::new(client, (), process).unwrap();

    // Wait for user input to quit
    println!("Press enter/return to quit...");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).ok();

    active_client.deactivate().unwrap();
}
