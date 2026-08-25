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
    }

    func applicationWillTerminate(_ notification: Notification) {
        bk_engine_shutdown()
    }

    static func preferencesPath() -> String {
        let configDir = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first!
            .appendingPathComponent("dev.banglakeyboard.settings")
        return configDir.appendingPathComponent("preferences.json").path
    }
}
