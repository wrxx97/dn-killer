import TaskList from "@components/task-list";
import Header from "@components/header";
import useStore from "@/stores";
import useTauriEvent from "@/hooks/useTauriEvent";
import useAddDragData from "@/hooks/useAddDragData";
import useResizeWindow from "@/hooks/useResizeWindow";
import SelectGroup from "./select-group";

export default function Home() {
  const store = useStore();
  useTauriEvent(); // rust后端监听到的事件
  useAddDragData(); // 设置app的窗口拖动
  useResizeWindow(); // 设置app的窗口大小

  return (
    <div>
      {store.focusTask && <Header left={<SelectGroup />}></Header>}
      <TaskList key="list"></TaskList>
    </div>
  );
}
