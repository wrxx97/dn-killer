import { invoke } from "@tauri-apps/api";
import throttle from "lodash/throttle";

export const killProcess = throttle(
  (processName: string) => {
    invoke("close_process_by_name", { processName }).then((response) =>
      console.log(response)
    );
  },
  2000,
  { trailing: false }
);

export const speakText = throttle(
  async (text: string) => {
    return invoke("speak_text", { text });
  },
  500,
  {
    trailing: false,
  }
);
