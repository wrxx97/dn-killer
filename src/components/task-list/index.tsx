import Box from "@mui/material/Box";
import Accordion from "@mui/material/Accordion";
import AccordionSummary from "@mui/material/AccordionSummary";
import AccordionDetails from "@mui/material/AccordionDetails";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import LinearProgress from "@mui/material/LinearProgress";
import useInterval from "ahooks/lib/useInterval";
import useUpdate from "ahooks/lib/useUpdate";

import Form from "@/components/task-list/Form";
import useStore, { getLeftDuration, processTask } from "@/stores";

export default function TaskList() {
  const tasks = useStore((store) => store.tasks);
  const updateTask = useStore((store) => store.updateTask);
  const hover = useStore((store) => store.hover);
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
      {[exitTask, ...tasks].map((task) => (
        <Accordion
          defaultExpanded={true}
          expanded={hover}
          key={task.id}
          sx={
            hover
              ? {}
              : {
                  backgroundColor: "rgba(0, 0, 0, 0.6)",
                  color: "white",
                }
          }
        >
          <AccordionSummary
            expandIcon={<ExpandMoreIcon />}
            aria-controls="panel1-content"
            id="panel1-header"
          >
            <Box sx={{ width: "100%" }}>
              {task.type !== "exit"
                ? <>
                  {`${task.title} （${getLeftDuration(task)}秒）(快捷键：${task.hotkey})`}
                  <LinearProgress
                    variant="determinate"
                    value={processTask(task)}
                  />
                </>
                : `退出 ${task.title} (快捷键：${task.hotkey})`
              }
            </Box>
          </AccordionSummary>
          <AccordionDetails>
            <Form data={task} />
          </AccordionDetails>
        </Accordion>
      ))}
    </Box>
  );
}
