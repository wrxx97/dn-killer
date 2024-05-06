import { useEffect, useRef } from "react";
import { appWindow } from "@tauri-apps/api/window";
import useStore from "@/stores";
import { invoke } from "@tauri-apps/api";
import throttle from "lodash/throttle";

const killProcess = throttle(
  (processName: string) => {
    invoke("close_process_by_name", { processName }).then((response) =>
      console.log(response)
    );
  },
  2000,
  { trailing: false }
);

export default () => {
  const tasks = useStore((store) => store.tasks);
  const updateTask = useStore((store) => store.updateTask);
  const exitTask = useStore((store) => store.exitTask);

  const listener = useRef<any>();
  useEffect(() => {
    async function init() {
      const unListenKeyDown = await appWindow.listen(
        "key-down",
        ({ payload }) => {
          if (payload === exitTask.hotkey) {
            exitTask.processName && killProcess(exitTask.title);
          }
        }
      );
      // sendNotification("Tauri is awesome!");
      // sendNotification({ title: "TAURI", body: "Tauri is awesome!" });
      // console.log(permissionGranted);
      const unListenKeyup = await appWindow.listen("key-up", ({ payload }) => {
        if (payload === "F2") {
          invoke("speak_text", { text: "Hello" });
        }
        tasks
          .filter((t) => t.hotkey === payload)
          .forEach((task) => {
            updateTask({ ...task, startTime: Date.now() });
          });
      });
      listener.current = { unListenKeyDown, unListenKeyup };
    }
    init();
    return () => {
      listener.current?.unListenKeyDown();
      listener.current?.unListenKeyup();
    };
  }, [tasks, exitTask]);
};
