import { useMemo } from "react";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";
import useStopDefaultKeyBoardEvent from "@/hooks/useStopDefaultKeyBoardEvent";
import useStopRightClickEvent from "@/hooks/useStopRightClickEvent";
import Home from "@/pages/home";
import Setting from "@/pages/setting";
import Check from "@/pages/check";
import useStore from "@/stores";
import useUpdater from "./hooks/useUpdater";

const router = createBrowserRouter([
  {
    path: "/home",
    element: <Home />,
  },
  {
    path: "/setting",
    element: <Setting />,
  },
  {
    path: "/",
    element: <Check />,
  },
]);

export default function App() {
  const setting = useStore((store) => store.setting);
  useStopDefaultKeyBoardEvent(); // 禁用浏览器的默认键盘事件
  useStopRightClickEvent(); // 禁用右键菜单
  useUpdater(); // 检查更新

  const darkTheme = useMemo(
    () =>
      createTheme({
        palette: {
          mode: "dark",
          background: {
            default: `rgba(0, 0, 0, ${setting.opciaty / 100})`,
            paper: "#0c0c0c",
          },
        },
      }),
    [setting.opciaty]
  );

  return (
    <ThemeProvider theme={darkTheme}>
      <CssBaseline />
      <RouterProvider router={router} />
    </ThemeProvider>
  );
}
