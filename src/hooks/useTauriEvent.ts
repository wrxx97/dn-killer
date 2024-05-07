import { useEffect, useRef } from "react";
import { appWindow } from "@tauri-apps/api/window";
import useStore from "@/stores";
import { killProcess } from "@/invoke";

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
      const unListenKeyup = await appWindow.listen("key-up", ({ payload }) => {
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
