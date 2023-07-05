//
//  ContentView.swift
//  using-in-swift
//
//  Created by admin on 7/5/23.
//

import SwiftUI
import Nand7000Asm

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundColor(.accentColor)
            Text("Hello, world!")
            Text("1 + 2 = " + String(add(1,2)))
        }
        .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
