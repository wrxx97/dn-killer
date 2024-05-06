import Header from "@components/header";
import TaskList from "@components/task-list";
import useStore from "./stores";
import useTauriEvent from "./hooks/useTauriEvent";
import useStopDefaultKeyBoardEvent from "./hooks/useStopDefaultKeyBoardEvent";
import useStopRightClickEvent from "./hooks/useStopRightClickEvent";
import useAddDragData from "./hooks/useAddDragData";
import useResizeWindow from "./hooks/useResizeWindow";

export default function App() {
  const store = useStore();
  useTauriEvent(); // rust后端监听到的事件
  useStopDefaultKeyBoardEvent(); // 禁用浏览器的默认键盘事件
  useStopRightClickEvent(); // 禁用右键菜单
  useAddDragData(); // 设置app的窗口拖动
  useResizeWindow(); // 设置app的窗口大小

  return (
    <div
      style={{
        position: "relative",
        height: "calc(100vh - 4px)",
      }}
    >
      {(store.focusTask || !store.tasks.length) && <Header></Header>}
      <TaskList key="list"></TaskList>
    </div>
  );
}
