import { create } from "zustand";

export type Task = {
  id: string;
  title: string;
  duration: number;
  completed: boolean;
  hotkey: string;
  startTime?: number | null;
  processName?: string;
  type?: "exit";
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
  exitTask: Task;
  updateExitTask: (task: Task) => void;
};

const getLocalConfig = () => {
  return JSON.parse(localStorage.getItem("appConfig") || "{}");
};

export const getLeftDuration = (task: Task): number => {
  const left = task.startTime
    ? (task.duration - (Date.now() - task?.startTime) / 1000).toFixed(0)
    : task.duration;
  return Number(left);
};

export const processTask = (task: Task) => {
  const left = getLeftDuration(task);
  return (left / task.duration) * 100;
};

const exitTask: Task = {
  id: "exit",
  title: "DragonNest_x64.exe",
  duration: -1,
  completed: false,
  hotkey: "F6",
  type: "exit",
  processName: "WeChat.exe",
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
      const tasks = state.tasks
        .map((t) => (t.id === task.id ? task : t))
        .sort((a, b) => {
          return getLeftDuration(a) - getLeftDuration(b);
        });
      return { tasks };
    });
  },
  exitTask: config.exitTask ?? exitTask,
  updateExitTask: (task: Task) => set({ exitTask: task }),
  save: () => {
    const { tasks, alwaysOnTop, exitTask } = get();
    localStorage.setItem(
      "appConfig",
      JSON.stringify({ tasks, alwaysOnTop, exitTask })
    );
  },
}));

export default useStore;
