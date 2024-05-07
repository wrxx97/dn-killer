import { useEffect } from "react";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";
import useStore from "@/stores";

export default () => {
  const tasks = useStore((store) => store.tasks);
  const focusTask = useStore((store) => store.focusTask);
  useEffect(() => {
    const height = focusTask
      ? 50 * tasks.length + 148 + 48
      : (tasks.length + 1) * 50;
    appWindow.setSize(new LogicalSize(600, height));
  }, [tasks.length, focusTask]);
};
