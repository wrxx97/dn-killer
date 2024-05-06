import { useEffect } from "react";

export default () => {
  useEffect(() => {
    const stopRightClickEvent = (e: MouseEvent) => {
      e.stopPropagation();
      e.preventDefault();
    };
    document.addEventListener("contextmenu", stopRightClickEvent);
    return () => {
      document.removeEventListener("contextmenu", stopRightClickEvent);
    };
  }, []);
};
