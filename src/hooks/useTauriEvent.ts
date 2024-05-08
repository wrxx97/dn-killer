import { useEffect, useRef } from "react";
import { appWindow } from "@tauri-apps/api/window";
import useStore, { Task } from "@/stores";
import { killProcess } from "@/invoke";

export default () => {
  const tasks = useStore((store) => store.tasks);
  const updateTask = useStore((store) => store.updateTask);
  const exitTask = useStore((store) => store.exitTask);
  const listener = useRef<any>();
  const storeRef = useRef<any>();

  useEffect(() => {
    storeRef.current = {
      tasks,
      updateTask,
      exitTask,
    };
    async function init() {
      const unListenKeyDown = await appWindow.listen(
        "key-down",
        ({ payload }) => {
          if (payload === storeRef.current.exitTask.hotkey) {
            storeRef.current.exitTask.processName &&
              killProcess(exitTask.title);
          }
        }
      );
      const unListenKeyup = await appWindow.listen("key-up", ({ payload }) => {
        storeRef.current.tasks
          .filter((t: Task) => t.hotkey === payload)
          .forEach((task: Task) => {
            storeRef.current.updateTask({ ...task, startTime: Date.now() });
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
