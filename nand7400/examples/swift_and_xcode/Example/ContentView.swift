//
//  ContentView.swift
//  Example
//
//  Created by admin on 7/6/23.
//

import Nand7400
import SwiftUI

let assemblyConf = AssemblerConfig(
	opcodes: [
		Opcode(mnemonic: "nop", binary: 0x00, numArgs: 0),
		Opcode(mnemonic: "lda", binary: 0x01, numArgs: 1),
		Opcode(mnemonic: "ldb", binary: 0x02, numArgs: 1),
		Opcode(mnemonic: "add", binary: 0x03, numArgs: 3),
		Opcode(mnemonic: "jmp", binary: 0x04, numArgs: 2),
		Opcode(mnemonic: "hlt", binary: 0xff, numArgs: 0),
	]
)

struct ContentView: View {
	@State private var assemblyText = "// Write some assembly...\njmp LABEL\nnop\nnop\n\nLABEL:\n\tadd 0x01 0x02 0x03\n\tlda -0x01\n\tldb +0x01"
	@State private var currentBinary: Data = .init()
	@State private var assembler = Assembler(config: assemblyConf)
	@State private var errorMessage: String = ""
	@State private var haveError = false
	
	var body: some View {
		VStack {
			VStack {
				TextEditor(text: self.$assemblyText)
					.font(Font.system(size: 15).monospaced())
					.padding(.top, 5)
				
				Divider()
				
				Text(self.currentBinary.map { String(format: "0x%02X", $0) }.joined(separator: " "))
					.font(Font.system(size: 15).monospaced())
			}
			.overlay(
				RoundedRectangle(cornerRadius: 4)
					.stroke(.blue, lineWidth: 2)
			)
			Button(action: {
				do {
					self.currentBinary = try assembler.assemble(source: assemblyText)
					print(self.currentBinary.map { String(format: "0x%02X", $0) }.joined(separator: " "))
				}
				catch {
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
			.alert(isPresented: self.$haveError) {
				Alert(title: Text("An assembling error occured:"),
				      message: Text(self.errorMessage),
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
