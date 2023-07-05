import Foundation
import Nand7400AsmRust

/// The main interface to the nand7400-asm library. This is a wrapper around the
/// Rust library.
public class Nand7400Asm {
	public static func Add(a: Int32, b: Int32) -> Int32 {
		Nand7400AsmRust.add(a, b)
	}

	public static func Flip(a: Bool) -> Bool {
		Nand7400AsmRust.flip(a)
	}
}
