import { create } from "zustand";

export type Task = {
  id: string;
  title: string;
  duration: number;
  completed: boolean;
  hotkey: string;
  startTime?: number | null;
  type?: "exit";
};

export type Setting = {
  notifyFlag?: boolean;
  notifyTemplate?: string;
  notifyDr?: number;
  opciaty: number;
  fontColor?: string;
  autoUpdate: boolean;
};

export type Group = {
  name: string;
  id: number;
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
  setting: Setting;
  updateSetting: (setting: Setting) => void;
  groups: Array<Group>;
  currentGroup: number;
  addGroup: (name: string) => void;
  checkUpdate: boolean;
  setCheckUpdate: (flag: boolean) => void;
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
};

const defaultSetting: Setting = {
  notifyFlag: true,
  notifyTemplate: "{title}还有{left}秒",
  notifyDr: 10,
  opciaty: 50,
  fontColor: "#fff",
  autoUpdate: true,
};

const config = getLocalConfig();

const useStore = create<Store>((set, get) => ({
  currentGroup: config.currentGroup ?? 0,
  groups: [{ name: "默认分组", id: 0 }],
  addGroup: (name: string) => {
    set((state) => {
      const groups = state.groups.concat({ name, id: Date.now() });
      return { groups };
    });
  },
  focusTask: null,
  setFocusTask: (task: Task | null) => set({ focusTask: task }),
  alwaysOnTop: config.alwaysOnTop ?? false,
  updateLock: () => set((state) => ({ alwaysOnTop: !state.alwaysOnTop })),
  tasks: config.tasks ?? [],
  addTask: () => {
    const { currentGroup } = get();
    const newTask = {
      id: Date.now().toString(),
      title: "New Task",
      duration: 0,
      completed: false,
      hotkey: "",
      group: currentGroup,
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
    const { tasks, alwaysOnTop, exitTask, setting, currentGroup, groups } =
      get();
    localStorage.setItem(
      "appConfig",
      JSON.stringify({
        tasks,
        alwaysOnTop,
        exitTask,
        setting,
        currentGroup,
        groups,
      })
    );
  },
  setting: config.setting ?? defaultSetting,
  updateSetting: (setting: Setting) => {
    const { setting: oldSetting } = get();
    set({
      setting: {
        ...oldSetting,
        ...setting,
      },
    });
  },
  checkUpdate: false,
  setCheckUpdate: (flag: boolean) => set({ checkUpdate: flag }),
}));

export default useStore;
