import { useEffect } from "react";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";
import useStore from "@/stores";
import { useLocation } from "react-router-dom";

export default () => {
  const tasks = useStore((store) => store.tasks);
  const focusTask = useStore((store) => store.focusTask);
  const location = useLocation();

  useEffect(() => {
    let height = 0;
    if (location.pathname === "/") {
      height = 200;
    } else if (location.pathname === "/setting") {
      const formHeight = document.querySelector("form")?.scrollHeight || 200;
      height = formHeight + 40;
    } else {
      height = focusTask
        ? 52 * tasks.length + 144 + 40
        : (tasks.length + 1) * 52;
    }
    appWindow.setSize(new LogicalSize(600, height));
  }, [tasks.length, focusTask, location]);
};
