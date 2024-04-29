import { useEffect } from "react";

const useMouse = () => {
  useEffect(() => {
    const fn = (event) => {
      var x = event.clientX; // 鼠标的水平位置
      var y = event.clientY; // 鼠标的垂直位置

      var screenWidth = window.innerWidth; // 页面宽度
      var screenHeight = window.innerHeight; // 页面高度

      // 检查鼠标位置是否在页面范围内
      if (x >= 0 && x <= screenWidth && y >= 0 && y <= screenHeight) {
        console.log("鼠标在页面范围内");
        // 在这里执行你想要的操作
      } else {
        console.log("鼠标不在页面范围内");
        // 在这里执行你想要的操作
      }
    };
    document.addEventListener("mousemove", fn);
    return () => {
      document.removeEventListener("mousemove", fn);
    };
  }, []);
};

export default useMouse;
