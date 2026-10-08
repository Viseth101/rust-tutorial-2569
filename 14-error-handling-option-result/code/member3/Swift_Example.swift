import Foundation

enum ParseError: Error {
    case invalidDigit
}

func parseAndDouble(_ text: String) throws -> Int {
    guard let number = Int(text) else {
        throw ParseError.invalidDigit
    }
    return number * 2
}

let input = "abc" 

do {
    let result = try parseAndDouble(input)
    print("Success: \(result)")
} catch ParseError.invalidDigit {
    print("Parsing Failure: Invalid digit found")
} catch {
    print("Parsing Failure: \(error)")
}
