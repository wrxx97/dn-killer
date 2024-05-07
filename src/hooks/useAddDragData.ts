import { useEffect } from "react";
import useStore from "@/stores";

export default () => {
  const focusTask = useStore((store) => store.focusTask);
  useEffect(() => {
    if (!focusTask) {
      // 给body下每个元素增加 draggable 属性
      document.body.querySelectorAll("*").forEach((el) => {
        // 只要不是svg元素，就增加 draggable 属性
        if (el.tagName !== "svg") {
          el.setAttribute("data-tauri-drag-region", "true");
          el.classList.add("drag-app");
        } else {
          el.classList.add("pointer");
        }
      });
    } else {
      document.body.querySelectorAll("*").forEach((el) => {
        // 只要元素id !== "drag-header"，就移除 draggable 属性
        if (el.id !== "drag-header") {
          el.removeAttribute("data-tauri-drag-region");
          el.classList.remove("drag-app");
          el.classList.remove("pointer");
        }
      });
    }
  }, [focusTask]);
};
