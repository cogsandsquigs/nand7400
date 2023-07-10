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
		Opcode(mnemonic: "lda", binary: 0x01, numArgs: 0),
		Opcode(mnemonic: "ldb", binary: 0x02, numArgs: 0),
		Opcode(mnemonic: "add", binary: 0x03, numArgs: 3),
		Opcode(mnemonic: "jmp", binary: 0x04, numArgs: 2),
		Opcode(mnemonic: "hlt", binary: 0xFF, numArgs: 0),
	]
)

struct ContentView: View {
	@State private var assemblyText = "// Write some assembly..."
	@State private var currentBinary: Data = Data()
	@State private var assembler = Assembler(config: assemblyConf)
	@State private var errorMessage: String = ""
	@State private var haveError = false
	
	var body: some View {
		VStack {
			HStack {
				TextEditor(text: self.$assemblyText)
					.font(Font.system(size:15).monospaced())
					.padding(.top, 5)
				
				Divider()
				
				Text(self.currentBinary.map {String(format: "0x%02x", $0)}.joined(separator:" "))
				
			}
			.overlay(
				RoundedRectangle(cornerRadius: 4)
					.stroke(.blue, lineWidth: 2)
			)
			
			
			Button(action: {
				do {
					self.currentBinary = try assembler.assemble(source: assemblyText)
					print(self.currentBinary.map {String(format: "0x%02x", $0)}.joined(separator:" "))
				} catch AssemblerError.OpcodeDne(mnemonic: let mnemonic, span: _) {
					self.errorMessage = "Opcode \'" + mnemonic + "\' does not exist!"
					self.haveError = true
				} catch AssemblerError.Unexpected(negatives: let negatives, positives: let positives, span:_){
					self.errorMessage = String(format: "Expected one of %s, got %s", positives.joined(separator:", "), negatives.joined(separator: ", "))
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
