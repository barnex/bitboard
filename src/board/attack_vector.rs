pub struct AttacVector {
	/// Indexed by Square.index()
	pub bitfields: [u64; 13],
	/// Indexed by White, Black
	pub all: [u64; 2],
}
