import { useMemo } from "react";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";
import useStopDefaultKeyBoardEvent from "@/hooks/useStopDefaultKeyBoardEvent";
import useStopRightClickEvent from "@/hooks/useStopRightClickEvent";
import Home from "@/pages/home";
import Setting from "@/pages/setting";
import useStore from "@/stores";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />,
  },
  {
    path: "/setting",
    element: <Setting />,
  },
]);

export default function App() {
  const setting = useStore((store) => store.setting);
  useStopDefaultKeyBoardEvent(); // 禁用浏览器的默认键盘事件
  useStopRightClickEvent(); // 禁用右键菜单

  const darkTheme = useMemo(
    () =>
      createTheme({
        palette: {
          mode: "dark",
          background: {
            default: `rgba(12, 12, 12, ${setting.opciaty / 100})`,
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
