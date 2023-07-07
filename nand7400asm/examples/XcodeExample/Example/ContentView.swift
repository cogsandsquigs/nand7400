//
//  ContentView.swift
//  Example
//
//  Created by admin on 7/6/23.
//

import Nand7400Asm
import SwiftUI

struct ContentView: View {
	@State private var assembler = AssemblerFfi(config: AssemblerConfig(opcodes: []))
	@State private var isError = false

	var body: some View {
		VStack {
			Button(action: {
				print(try! assembler.assemble(source: "test").map { String(format: "%01X", $0) }.joined())
			}) {
				Image(systemName: "slider.horizontal.2.square.badge.arrow.down")
					.imageScale(.large)
					.foregroundColor(.accentColor)
				Text("Assemble!")
			}
			.alert(isPresented: $isError) {
				Alert(title: Text("Can't be scheduled"),
				      message: Text("Try changing the name"),
				      dismissButton: .default(Text("OK")))
			}
//			.buttonStyle(.bordered)
		}
		.padding()
	}
}

struct ContentView_Previews: PreviewProvider {
	static var previews: some View {
		ContentView()
	}
}
