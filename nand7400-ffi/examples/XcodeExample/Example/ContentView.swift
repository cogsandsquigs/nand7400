//
//  ContentView.swift
//  Example
//
//  Created by admin on 7/6/23.
//

import Nand7400
import SwiftUI

struct ContentView: View {
	@State private var assemblyText = "// Write some assembly..."
	@State private var currentBinary: Data = Data()
	@State private var assembler = Assembler(config: AssemblerConfig(opcodes: []))
	@State private var errorMessage: String? = nil
	@State private var haveError = false
	
	var body: some View {
		VStack {
			HStack {
				TextEditor(text: self.$assemblyText)
					.font(Font.system(size:15).monospaced())
					.padding(.top, 5)
					.overlay(
						RoundedRectangle(cornerRadius: 4)
							.stroke(.blue, lineWidth: 2)
					)
				
				Divider()
				
				Text(self.currentBinary.map {String(format: "%01x", $0)}.joined(separator:" "))
				
			}
			
			Button(action: {
				do {
					self.currentBinary = try assembler.assemble(source: assemblyText)
				} catch AssemblerError.OpcodeDne(mnemonic: let mnemonic, span: _) {
					self.errorMessage = "Opcode \'" + mnemonic + "\' does not exist!"
					self.haveError = true
				} catch {
					self.errorMessage = "An error occured!"
					self.haveError = true
					print(error)
					
				}
			}) {
				Image(systemName: "slider.horizontal.2.square.badge.arrow.down")
					.imageScale(.large)
					.foregroundColor(.accentColor)
				Text("Assemble!")
			}
			.alert(isPresented: $haveError) {
				Alert(title: Text("An assembling error occured:"),
					  message: Text(errorMessage!),
					  dismissButton: .default(Text("OK")))
				
				
				
			}
			.padding()
		}
		.padding()
	}
}

struct ContentView_Previews: PreviewProvider {
	static var previews: some View {
		ContentView()
	}
}
