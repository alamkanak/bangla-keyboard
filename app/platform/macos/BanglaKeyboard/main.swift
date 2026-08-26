import Cocoa
import InputMethodKit
import Carbon

// Move cwd out of protected folders (~/Desktop, ~/Documents, etc.)
// to prevent macOS TCC permission prompts on launch.
FileManager.default.changeCurrentDirectoryPath("/")

// Entry point for the IMKit input method app
let app = IMKApplication.shared
NSApp.run()
