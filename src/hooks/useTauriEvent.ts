import { useEffect } from "react";
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
  useEffect(() => {
    appWindow.listen("key-down", ({ payload }) => {
      if (payload === exitTask.hotkey) {
        exitTask.processName && killProcess(exitTask.title);
      }
    });
    appWindow.listen("key-up", ({ payload }) => {
      const task = tasks.find((t) => t.hotkey === payload);
      if (task) {
        updateTask({ ...task, startTime: Date.now() });
      }
    });
  }, [tasks, exitTask]);
};
