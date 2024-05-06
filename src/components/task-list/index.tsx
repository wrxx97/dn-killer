import Box from "@mui/material/Box";
import useInterval from "ahooks/lib/useInterval";
import useUpdate from "ahooks/lib/useUpdate";
import useStore, { getLeftDuration } from "@/stores";
import Item from './Item';

export default function TaskList() {
  const tasks = useStore((store) => store.tasks);
  const updateTask = useStore((store) => store.updateTask);
  const exitTask = useStore((store) => store.exitTask);

  const update = useUpdate();
  useInterval(() => {
    tasks.forEach((task) => {
      if (getLeftDuration(task) <= 0) {
        updateTask({ ...task, startTime: null });
      }
    });
    update();
  }, 1000);

  return (
    <Box
      sx={{
        height: "calc(100% - 50px)",
        overflowY: "scroll",
      }}
    >
      {[exitTask, ...tasks].map((task) => <Item task={task} />)}
    </Box>
  );
}
