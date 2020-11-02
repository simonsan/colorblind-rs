// Some magic incantations to make the compiler more pedantic (like --pedantic)
#![deny(
	future_incompatible,
	nonstandard_style,
	rust_2018_idioms,
	missing_copy_implementations,
	trivial_numeric_casts,
	unsafe_code
)]

#[cfg(not(debug_assertions))]
#[macro_use]
extern crate human_panic;

fn main() {
	// Human Panic. Only enabled when *not* debugging.
	#[cfg(not(debug_assertions))]
	{
		setup_panic!();
	}

	// Setup Logging
	// log::setup_logging()?;
}

enum ColorBlindType {
	Achromatomaly,
	Achromatopsia,
	Deuteranomaly, // Malfunctioning M-cone (green)
	Deuteranopia,  // Missing. M-cone (green)
	Protanomaly,   // Malfunctioning L-cone (red)
	Protanopia,    // Missing L-cone (red)
	Tritanomaly,
	Tritanopia,
}
