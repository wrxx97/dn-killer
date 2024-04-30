import Header from "@components/header";
import TaskList from "@components/task-list";
import useStore from "./stores";
import useTauriEvent from "./hooks/useTauriEvent";

export default function App() {
  const store = useStore();
  useTauriEvent();
  return (
    <div
      style={{
        position: "relative",
        height: "calc(100vh - 4px)",
      }}
      className={store.hover ? "hover-app" : ""}
      onMouseOver={() => store.setHover(true)}
      onMouseLeave={() => store.setHover(false)}
    >
      {(store.hover || !store.tasks.length) && <Header></Header>}
      <TaskList></TaskList>
    </div>
  );
}
