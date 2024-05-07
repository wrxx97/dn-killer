import { useEffect } from "react";

export default () => {
  useEffect(() => {
    const stopDefaultKeyBoardEvent = (e: KeyboardEvent) => {
      const isDev = import.meta.env.MODE === "development";
      // 屏蔽的按键
      const stopKeys = isDev ? ["F3"] : ["F3", "F5", "F12"];
      if (stopKeys.includes(e.key) || (e.ctrlKey && e.key === "r")) {
        e.stopPropagation();
        e.preventDefault();
      }
    };
    document.addEventListener("keydown", stopDefaultKeyBoardEvent);
    return () => {
      document.removeEventListener("keydown", stopDefaultKeyBoardEvent);
    };
  }, []);
};
