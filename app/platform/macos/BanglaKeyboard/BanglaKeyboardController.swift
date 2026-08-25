import Cocoa
import InputMethodKit

@objc(BanglaKeyboardController)
class BanglaKeyboardController: IMKInputController {
    private var composingText = ""

    private var engineReady: Bool { AppDelegate.engineReady }

    // MARK: - Key Handling

    override func handle(_ event: NSEvent!, client sender: Any!) -> Bool {
        guard let event = event, event.type == .keyDown else {
            return false
        }
        guard let client = sender as? IMKTextInput else {
            return false
        }

        let modifiers = event.modifierFlags.intersection(.deviceIndependentFlagsMask)
        let isOption = modifiers.contains(.option)
        let isNationalMode = bk_get_mode() == 2

        // Command/Control always pass through. Option passes through unless in National mode.
        if modifiers.contains(.command) || modifiers.contains(.control) {
            commitComposition(client)
            return false
        }
        if isOption && !isNationalMode {
            commitComposition(client)
            return false
        }

        let keyCode = event.keyCode
        switch keyCode {
        case 36: // Return
            if engineReady && bk_is_composing() {
                return handleCommit(client)
            }
            if engineReady && isNationalMode { bk_reset() }
            return false
        case 49: // Space
            if engineReady && bk_is_composing() {
                return handleSpace(client)
            }
            if engineReady && isNationalMode { bk_reset() }
            return false
        case 51: // Backspace
            if engineReady && bk_is_composing() {
                return handleBackspace(client)
            }
            if engineReady && isNationalMode {
                let action = bk_handle_backspace()
                if action >= 4 {
                    // CommitReplaceN: engine consumed the backspace logically
                    // but since we commit immediately in National, the chars are already in the document.
                    // We just let the OS handle the backspace naturally.
                }
                return false
            }
            return false
        case 53: // Escape
            if engineReady && bk_is_composing() {
                return handleCancel(client)
            }
            return false
        case 48, 123, 124, 125, 126: // Tab, arrows
            commitComposition(client)
            return false
        default:
            break
        }

        // When Option is held (AltGr for National), use the base character ignoring modifiers
        let charSource = isOption ? event.charactersIgnoringModifiers : event.characters
        guard let chars = charSource, let ch = chars.first, ch.isASCII else {
            return false
        }

        let shift = modifiers.contains(.shift)
        let altgr = isOption
        if engineReady {
            let charVal = Int8(bitPattern: UInt8(ch.asciiValue ?? 0))
            let action = bk_handle_key_full(charVal, shift, altgr)
            if action == 1 { // UpdatePreview
                if let preview = bk_get_preview() {
                    let text = String(cString: preview)
                    bk_free_string(preview)
                    client.setMarkedText(
                        text,
                        selectionRange: NSRange(location: text.count, length: 0),
                        replacementRange: NSRange(location: NSNotFound, length: NSNotFound)
                    )
                }
                return true
            } else if action == 0 { // Commit
                if let preview = bk_get_preview() {
                    let text = String(cString: preview)
                    bk_free_string(preview)
                    if !text.isEmpty {
                        client.insertText(text, replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
                    }
                }
                return true
            } else if action >= 4 { // CommitReplaceN
                let backspaceCount = Int(action - 4)
                if let preview = bk_get_preview() {
                    let text = String(cString: preview)
                    bk_free_string(preview)
                    // Delete previous characters then insert new text
                    for _ in 0..<backspaceCount {
                        client.insertText("", replacementRange: NSRange(location: NSNotFound, length: 1))
                    }
                    if !text.isEmpty {
                        client.insertText(text, replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
                    }
                }
                return true
            } else if action == 3 { // CommitReplaceLast
                if let preview = bk_get_preview() {
                    let text = String(cString: preview)
                    bk_free_string(preview)
                    if !text.isEmpty {
                        client.insertText(text, replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
                    }
                }
                return true
            }
            return false
        } else {
            // Fallback: passthrough
            composingText.append(ch)
            client.setMarkedText(
                composingText,
                selectionRange: NSRange(location: composingText.count, length: 0),
                replacementRange: NSRange(location: NSNotFound, length: NSNotFound)
            )
            return true
        }
    }

    private func handleCommit(_ client: IMKTextInput) -> Bool {
        if engineReady {
            guard bk_is_composing() else { return false }
            if let committed = bk_handle_enter() {
                let text = String(cString: committed)
                bk_free_string(committed)
                client.insertText(text, replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
            }
            return true
        } else {
            guard !composingText.isEmpty else { return false }
            client.insertText(composingText, replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
            composingText = ""
            return true
        }
    }

    private func handleSpace(_ client: IMKTextInput) -> Bool {
        if engineReady {
            guard bk_is_composing() else { return false }
            if let committed = bk_handle_space() {
                let text = String(cString: committed)
                bk_free_string(committed)
                client.insertText(text, replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
            }
            return true
        } else {
            guard !composingText.isEmpty else { return false }
            client.insertText(composingText + " ", replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
            composingText = ""
            return true
        }
    }

    private func handleBackspace(_ client: IMKTextInput) -> Bool {
        if engineReady {
            guard bk_is_composing() else { return false }
            let result = bk_handle_backspace()
            if result == 0 { // emptied
                client.insertText("", replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
            } else if result == 1 { // updated
                if let preview = bk_get_preview() {
                    let text = String(cString: preview)
                    bk_free_string(preview)
                    client.setMarkedText(
                        text,
                        selectionRange: NSRange(location: text.count, length: 0),
                        replacementRange: NSRange(location: NSNotFound, length: NSNotFound)
                    )
                }
            }
            return true
        } else {
            guard !composingText.isEmpty else { return false }
            composingText.removeLast()
            if composingText.isEmpty {
                client.insertText("", replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
            } else {
                client.setMarkedText(
                    composingText,
                    selectionRange: NSRange(location: composingText.count, length: 0),
                    replacementRange: NSRange(location: NSNotFound, length: NSNotFound)
                )
            }
            return true
        }
    }

    private func handleCancel(_ client: IMKTextInput) -> Bool {
        if engineReady {
            guard bk_is_composing() else { return false }
            bk_reset()
        } else {
            guard !composingText.isEmpty else { return false }
            composingText = ""
        }
        client.insertText("", replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
        return true
    }

    private func commitComposition(_ client: IMKTextInput) {
        if engineReady {
            guard bk_is_composing() else { return }
            commitEngineText(client)
        } else {
            guard !composingText.isEmpty else { return }
            client.insertText(composingText, replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
            composingText = ""
        }
    }

    private func commitEngineText(_ client: IMKTextInput) {
        if let preview = bk_get_preview() {
            let text = String(cString: preview)
            bk_free_string(preview)
            if !text.isEmpty {
                client.insertText(text, replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
            }
        }
        bk_reset()
    }

    // MARK: - Menu (provides "Open Settings" option)

    override func menu() -> NSMenu! {
        let menu = NSMenu()
        menu.addItem(withTitle: "Open Settings…", action: #selector(openSettings(_:)), keyEquivalent: "")
        menu.addItem(NSMenuItem.separator())
        menu.addItem(withTitle: "About Bangla Keyboard", action: #selector(showAbout(_:)), keyEquivalent: "")
        return menu
    }

    @objc func openSettings(_ sender: Any?) {
        let bundle = Bundle.main

        // 1. Check inside IME bundle (production: embedded in Resources/)
        let embedded = bundle.resourcePath! + "/BanglaKeyboardSettings.app"
        // 2. Sibling to the IME in ~/Library/Input Methods/
        let sibling = (bundle.bundlePath as NSString).deletingLastPathComponent + "/BanglaKeyboardSettings.app"
        // 3. /Applications
        let global = "/Applications/Bangla Keyboard Settings.app"

        for path in [embedded, sibling, global] {
            if FileManager.default.fileExists(atPath: path) {
                NSWorkspace.shared.openApplication(
                    at: URL(fileURLWithPath: path),
                    configuration: .init()
                )
                return
            }
        }

        // Last resort: open the bundle identifier via Launch Services
        if let url = NSWorkspace.shared.urlForApplication(withBundleIdentifier: "dev.banglakeyboard.settings") {
            NSWorkspace.shared.openApplication(at: url, configuration: .init())
            return
        }

        NSLog("BanglaKeyboard: Settings app not found")
    }

    @objc func showAbout(_ sender: Any?) {
        let alert = NSAlert()
        alert.messageText = "Bangla Keyboard"
        alert.informativeText = "Version 0.1.0\nUniBijoy & Phonetic input for macOS"
        alert.alertStyle = .informational
        alert.runModal()
    }

    // MARK: - Lifecycle

    override func activateServer(_ sender: Any!) {
        super.activateServer(sender)
        composingText = ""
        if engineReady {
            bk_reset()
            reloadLayoutPreference()
        }
    }

    private func reloadLayoutPreference() {
        let prefsPath = AppDelegate.preferencesPath()
        guard let data = try? Data(contentsOf: URL(fileURLWithPath: prefsPath)),
              let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
              let layout = json["layout"] as? String else { return }
        switch layout {
        case "unibijoy": bk_set_mode(1)
        case "national": bk_set_mode(2)
        default: bk_set_mode(0)
        }
    }

    override func deactivateServer(_ sender: Any!) {
        if let client = sender as? IMKTextInput {
            commitComposition(client)
        }
        super.deactivateServer(sender)
    }
}
