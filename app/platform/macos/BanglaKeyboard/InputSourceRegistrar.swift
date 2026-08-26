import Foundation
import Carbon

/// Handles programmatic registration and enabling of the input source via TIS APIs.
/// Called from the postinstall script and the Settings app.
enum InputSourceRegistrar {
    private static let bundleIdentifier = "dev.banglakeyboard.inputmethod.BanglaKeyboard"
    private static let inputSourceID = "dev.banglakeyboard.inputmethod.BanglaKeyboard.Bangla"

    static func register() -> Int32 {
        let installPath = NSString("~/Library/Input Methods/BanglaKeyboard.app").expandingTildeInPath
        let url = URL(fileURLWithPath: installPath)

        guard FileManager.default.fileExists(atPath: installPath) else {
            NSLog("BanglaKeyboard: .app bundle not found at \(installPath)")
            return 1
        }

        let status = TISRegisterInputSource(url as CFURL)
        if status == noErr {
            NSLog("BanglaKeyboard: Input source registered successfully")
            return 0
        } else {
            NSLog("BanglaKeyboard: TISRegisterInputSource failed with status \(status)")
            return 1
        }
    }

    static func enable() -> Int32 {
        guard let source = findInputSource(enabled: false) ?? findInputSource(enabled: true) else {
            NSLog("BanglaKeyboard: Could not find input source to enable")
            return 1
        }

        let status = TISEnableInputSource(source)
        if status == noErr {
            NSLog("BanglaKeyboard: Input source enabled successfully")
            return 0
        } else {
            NSLog("BanglaKeyboard: TISEnableInputSource failed with status \(status)")
            return 1
        }
    }

    static func select() -> Int32 {
        guard let source = findInputSource(enabled: true) else {
            NSLog("BanglaKeyboard: Could not find enabled input source to select")
            return 1
        }

        let status = TISSelectInputSource(source)
        if status == noErr {
            NSLog("BanglaKeyboard: Input source selected successfully")
            return 0
        } else {
            NSLog("BanglaKeyboard: TISSelectInputSource failed with status \(status)")
            return 1
        }
    }

    static func isEnabled() -> Bool {
        return findInputSource(enabled: true) != nil
    }

    private static func findInputSource(enabled: Bool) -> TISInputSource? {
        let properties: [String: Any] = [
            kTISPropertyBundleID as String: bundleIdentifier,
            kTISPropertyInputSourceIsEnabled as String: enabled,
        ]
        guard let sources = TISCreateInputSourceList(properties as CFDictionary, !enabled)?.takeRetainedValue() as? [TISInputSource],
              !sources.isEmpty else {
            return nil
        }
        return sources.first
    }
}
