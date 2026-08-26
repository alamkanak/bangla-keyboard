import Cocoa
import InputMethodKit

// IMKit requires a custom NSApplication subclass as NSPrincipalClass
@objc(IMKApplication)
class IMKApplication: NSApplication {
    private let appDelegate = AppDelegate()

    override init() {
        super.init()
        self.delegate = appDelegate
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
}

class AppDelegate: NSObject, NSApplicationDelegate {
    var server: IMKServer!
    static var engineReady = false

    func applicationDidFinishLaunching(_ notification: Notification) {
        let connectionName = Bundle.main.infoDictionary?["InputMethodConnectionName"] as? String
            ?? "dev.banglakeyboard.inputmethod.BanglaKeyboard_Connection"
        let bundleId = Bundle.main.bundleIdentifier
            ?? "dev.banglakeyboard.inputmethod.BanglaKeyboard"

        // Init engine once at launch
        let dataPath = (Bundle.main.resourcePath ?? "") + "/data"
        NSLog("BanglaKeyboard: Data path: \(dataPath)")
        NSLog("BanglaKeyboard: Data exists: \(FileManager.default.fileExists(atPath: dataPath))")
        NSLog("BanglaKeyboard: avrophonetic.json exists: \(FileManager.default.fileExists(atPath: dataPath + "/avrophonetic.json"))")

        let result = bk_engine_init(dataPath)
        if result == 0 {
            AppDelegate.engineReady = true
            NSLog("BanglaKeyboard: Engine initialized OK")

            // Read saved layout preference
            let prefsPath = Self.preferencesPath()
            if let data = try? Data(contentsOf: URL(fileURLWithPath: prefsPath)),
               let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
               let layout = json["layout"] as? String {
                switch layout {
                case "unibijoy":
                    bk_set_mode(1)
                    NSLog("BanglaKeyboard: Mode set to UniBijoy")
                case "national":
                    bk_set_mode(2)
                    NSLog("BanglaKeyboard: Mode set to National")
                default:
                    NSLog("BanglaKeyboard: Mode set to Phonetic (default)")
                }
            } else {
                NSLog("BanglaKeyboard: Mode set to Phonetic (default)")
            }
        } else {
            NSLog("BanglaKeyboard: Engine init FAILED (code \(result))")
        }

        server = IMKServer(name: connectionName, bundleIdentifier: bundleId)
        NSLog("BanglaKeyboard: IMKServer started")

        // Auto-launch Settings/Onboarding on first run
        if !Self.isOnboardingComplete() {
            NSLog("BanglaKeyboard: Onboarding not complete, launching Settings")
            DispatchQueue.main.asyncAfter(deadline: .now() + 1.0) {
                Self.launchSettingsApp()
            }
        }
    }

    func applicationWillTerminate(_ notification: Notification) {
        bk_engine_shutdown()
    }

    static func preferencesPath() -> String {
        let configDir = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first!
            .appendingPathComponent("dev.banglakeyboard.settings")
        return configDir.appendingPathComponent("preferences.json").path
    }

    private static func isOnboardingComplete() -> Bool {
        let path = preferencesPath()
        guard let data = try? Data(contentsOf: URL(fileURLWithPath: path)),
              let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
              let complete = json["onboarding_complete"] as? Bool else {
            return false
        }
        return complete
    }

    static func launchSettingsApp() {
        let bundle = Bundle.main
        let candidates = [
            bundle.resourcePath! + "/BanglaKeyboardSettings.app",
            (bundle.bundlePath as NSString).deletingLastPathComponent + "/BanglaKeyboardSettings.app",
            "/Applications/Bangla Keyboard Settings.app",
        ]

        for path in candidates {
            if FileManager.default.fileExists(atPath: path) {
                NSWorkspace.shared.openApplication(
                    at: URL(fileURLWithPath: path),
                    configuration: .init()
                )
                return
            }
        }

        if let url = NSWorkspace.shared.urlForApplication(withBundleIdentifier: "dev.banglakeyboard.settings") {
            NSWorkspace.shared.openApplication(at: url, configuration: .init())
            return
        }

        NSLog("BanglaKeyboard: Settings app not found for onboarding")
    }
}
