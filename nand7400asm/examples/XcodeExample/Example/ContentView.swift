//
//  ContentView.swift
//  Example
//
//  Created by admin on 7/6/23.
//

import SwiftUI
import Nand7400Asm

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundColor(.accentColor)
            Text("Hello, world!")
            Text("1 + 2 = " + String(add(a: 1, b: 2)))
            Text("The opposite of true is " + String(flip(a: true)))
            Text(hello(name: "SwiftUI"))
        }
        .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
