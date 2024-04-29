import { create } from "zustand";

export type Task = {
  id: string;
  title: string;
  duration: number;
  completed: boolean;
  hotkey: string;
  startTime?: number;
};

type Store = {
  alwaysOnTop: boolean;
  updateLock: () => void;
  tasks: Array<Task>;
  addTask: () => void;
  deleteTask: (id: string) => void;
  updateTask: (task: Task) => void;
  hover: boolean;
  setHover: (hover: boolean) => void;
  save: () => void;
};

const getLocalConfig = () => {
  return JSON.parse(localStorage.getItem("appConfig") || "{}");
};

const config = getLocalConfig();

const useStore = create<Store>((set, get) => ({
  hover: false,
  setHover: (hover) => set({ hover: hover }),
  alwaysOnTop: config.alwaysOnTop ?? false,
  updateLock: () => set((state) => ({ alwaysOnTop: !state.alwaysOnTop })),
  tasks: config.tasks ?? [],
  addTask: () => {
    set((state) => ({
      tasks: [
        {
          id: Date.now().toString(),
          title: "New Task",
          duration: 0,
          completed: false,
          hotkey: "",
        },
        ...state.tasks,
      ],
    }));
  },
  deleteTask: (id: string) =>
    set((state) => ({ tasks: state.tasks.filter((task) => task.id !== id) })),
  updateTask: (task: Task) => {
    set((state) => {
      const tasks = state.tasks.map((t) => (t.id === task.id ? task : t));
      return { tasks };
    });
  },
  save: () => {
    const { tasks, alwaysOnTop } = get();
    localStorage.setItem("appConfig", JSON.stringify({ tasks, alwaysOnTop }));
  },
}));

export default useStore;
