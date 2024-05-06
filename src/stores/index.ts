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
  save: () => void;
  exitTask: Task;
  updateExitTask: (task: Task) => void;
  focusTask: Task | null;
  setFocusTask: (task: Task | null) => void;
  route: string;
  navigate: (route: string) => void;
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
  focusTask: null,
  setFocusTask: (task: Task | null) => set({ focusTask: task }),
  alwaysOnTop: config.alwaysOnTop ?? false,
  updateLock: () => set((state) => ({ alwaysOnTop: !state.alwaysOnTop })),
  tasks: config.tasks ?? [],
  addTask: () => {
    const newTask = {
      id: Date.now().toString(),
      title: "New Task",
      duration: 0,
      completed: false,
      hotkey: "",
    };
    set((state) => ({
      tasks: [newTask, ...state.tasks],
      focusTask: newTask,
    }));
  },
  deleteTask: (id: string) => {
    const { focusTask, tasks } = get();
    const newTarget = tasks.find((task) => task.id !== id);
    if (id === focusTask?.id) {
      set({ focusTask: newTarget });
    }
    set((state) => ({ tasks: state.tasks.filter((task) => task.id !== id) }));
  },
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
  route: "/",
  navigate: (route: string) => set({ route }),
}));

export default useStore;
