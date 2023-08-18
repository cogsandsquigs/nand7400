//
//  ContentView.swift
//  Example
//
//  Created by Ian Pratt on 7/6/23.
//

import Nand7400
import class Nand7400.Formatter
import SwiftUI

let assemblyConf = AssemblerConfig(
	opcodes: [
		Opcode(mnemonic: "nop", binary: 0x00, args: []),
		Opcode(mnemonic: "lda", binary: 0x01, args: [OpcodeArg.immediate]),
		Opcode(mnemonic: "ldb", binary: 0x02, args: [OpcodeArg.indirect]),
		Opcode(mnemonic: "add", binary: 0x03, args: [OpcodeArg.immediate, OpcodeArg.immediate, OpcodeArg.immediate]),
		Opcode(mnemonic: "jmp", binary: 0x04, args: [OpcodeArg.immediate, OpcodeArg.immediate]),
		Opcode(mnemonic: "hlt", binary: 0xff, args: []),
	]
)

struct ContentView: View {
	@State private var assemblyText = """
	; Write some assembly...
	jmp LABEL
	nop
	nop

	LABEL:
	 add #0x01 #0x02 #0x03
	 lda #-0x01
	 ldb +0x01
	"""
	@State private var currentBinary = Data()
	@State private var assembler = Assembler(config: assemblyConf)
	@State private var formatter = Formatter()
	@State private var errorMessage = ""
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

			HStack {
				Button(action: {
					do {
						self.currentBinary = try self.assembler.assemble(source: assemblyText)
						print(self.currentBinary.map { String(format: "0x%02X", $0) }.joined(separator: " "))
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
				.alert(isPresented: self.$haveError) {
					Alert(title: Text("An assembling error occured:"),
					      message: Text(self.errorMessage),
					      dismissButton: .default(Text("OK")))
				}
				.padding()

				Button(action: {
					self.assemblyText = self.formatter.format(source: self.assemblyText)
				}) {
					Image(systemName: "wand.and.stars")
						.imageScale(.large)
						.foregroundColor(.accentColor)
					Text("Format!")
				}
				.padding()
			}
		}
		.padding()
	}
}

struct ContentView_Previews: PreviewProvider {
	static var previews: some View {
		ContentView()
	}
}
