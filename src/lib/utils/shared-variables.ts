export class EventStrings {
  static readonly startStudyEvent = "startStudyEvent";
  static readonly endStudyEvent = "endStudyEvent";
}

// See src-tauri/main.rs for the Tauri event names 
export class TauriEventStrings {
  static readonly onAppStart = "tauri_on_start";
  static readonly onWindowStateChange = "tauri_window_state_change";
  static readonly onStudyEnded = "tauri_study_ended";
}

export class TauriWindowChangeArgs {
  studying: boolean = false;
  list: boolean = false;
  programmes: boolean = false;
}