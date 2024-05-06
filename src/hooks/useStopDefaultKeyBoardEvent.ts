import { useEffect } from "react";
export default () => {
  useEffect(() => {
    const stopDefaultKeyBoardEvent = (e: KeyboardEvent) => {
      if (
        e.key === "F3" ||
        // e.key === "F5" ||
        // e.key === "F12" ||
        (e.ctrlKey && e.key === "r")
      ) {
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
