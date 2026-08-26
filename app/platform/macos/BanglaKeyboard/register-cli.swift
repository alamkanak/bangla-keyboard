import Foundation
import Carbon

// Lightweight CLI tool for registering/enabling the input source.
// Separate from the main IME binary to avoid InputMethodKit initialization.

@main
struct RegisterCLI {
    static func main() {
        if CommandLine.arguments.count < 2 {
            print("Usage: bangla-keyboard-register --register|--enable|--select|--check-status")
            exit(1)
        }

        let command = CommandLine.arguments[1]

        switch command {
        case "--register-input-source", "--register":
            exit(InputSourceRegistrar.register())
        case "--enable-input-source", "--enable":
            exit(InputSourceRegistrar.enable())
        case "--select-input-source", "--select":
            exit(InputSourceRegistrar.select())
        case "--check-status":
            let enabled = InputSourceRegistrar.isEnabled()
            print(enabled ? "enabled" : "not-enabled")
            exit(enabled ? 0 : 1)
        default:
            print("Unknown command: \(command)")
            exit(1)
        }
    }
}
