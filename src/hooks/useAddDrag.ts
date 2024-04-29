import { useEffect } from "react";

const useAddDrag = () => {
  useEffect(() => {
    const body = document.body;
    // 给body下每一个元素 设置参数
    const elements = body.querySelectorAll("*");
    elements.forEach((element) => {
      element.setAttribute("data-tauri-drag-region", "true");
    });
  }, []);
};

export default useAddDrag;
