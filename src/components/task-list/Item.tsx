import Box from "@mui/material/Box";
import Accordion from "@mui/material/Accordion";
import AccordionSummary from "@mui/material/AccordionSummary";
import AccordionDetails from "@mui/material/AccordionDetails";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import LinearProgress from "@mui/material/LinearProgress";

import Form from "@/components/task-list/Form";
import useStore, { getLeftDuration, processTask, Task } from "@/stores";

export default ({ task }: { task: Task }) => {
  const focusTask = useStore((store) => store.focusTask);
  const setFocusTask = useStore((store) => store.setFocusTask);
  const isFocus = focusTask?.id === task.id;

  return (
    <Accordion
      defaultExpanded={false}
      expanded={isFocus}
      onChange={() => setFocusTask(isFocus ? null : task)}
      key={task.id}
      sx={
        isFocus
          ? {
              "&.Mui-expanded": {
                margin: 0,
              },
            }
          : {
              "&.MuiAccordion-root": {
                backgroundColor: "transparent",
              },
            }
      }
    >
      <AccordionSummary expandIcon={<ExpandMoreIcon fontSize="large" />}>
        <Box sx={{ width: "100%" }}>
          {task.type !== "exit" ? (
            <>
              {`${task.title} （${getLeftDuration(task)}秒）(快捷键：${
                task.hotkey
              })`}
              <LinearProgress variant="determinate" value={processTask(task)} />
            </>
          ) : (
            `退出 ${task.title} (快捷键：${task.hotkey})`
          )}
        </Box>
      </AccordionSummary>
      <AccordionDetails>
        <Form data={task} />
      </AccordionDetails>
    </Accordion>
  );
};
