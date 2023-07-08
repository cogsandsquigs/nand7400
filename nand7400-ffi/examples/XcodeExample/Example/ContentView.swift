//
//  ContentView.swift
//  Example
//
//  Created by admin on 7/6/23.
//

import Nand7400
import SwiftUI

struct ContentView: View {
	@State private var assembler = Assembler(config: AssemblerConfig(opcodes: []))
	@State private var error: Error? = nil
	@State private var haveError = false
	
	var body: some View {
		VStack {
			Button(action: {
				do {
					print(try assembler.assemble(source: "test").map { String(format: "%01X", $0) }.joined())
				} catch {
					self.error = error
				}
			}) {
				Image(systemName: "slider.horizontal.2.square.badge.arrow.down")
					.imageScale(.large)
					.foregroundColor(.accentColor)
				Text("Assemble!")
			}
			.alert(isPresented: $haveError) {
				Alert(title: Text("Error!"),
					  message: Text(error!.localizedDescription),
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
